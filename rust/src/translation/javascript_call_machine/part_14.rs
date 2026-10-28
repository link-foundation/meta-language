// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryOwnKeysLoop114Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_value_error;
use super::source_ml_source1_temporary_insert_java_script_ordinary_object_loop117;
use super::source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop120;
use super::source_ml_source1_temporary_read_java_script_ordinary_own_keys_loop114;
use super::source_ml_source1_temporary_register_java_script_ordinary_symbol_loop119;
use super::test_java_script_ordinary_symbol;
use super::test_java_script_semantic_accessor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_own_keys(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big) -> (self::ml_string::Text, Vec<self::JavaScriptSemanticValue>) {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            {
                let source_ml_source2_field786 = Vec::<self::JavaScriptSemanticValue>::new();
                {
                    let source_ml_source2_field787 = error.clone();
                    (source_ml_source2_field787.clone(), source_ml_source2_field786)
                }
            }
        } else {
            {
                let index = self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value788, _) => {
                        source_ml_source2_data_property_value788
                    }
                }, identity.clone());
                {
                    let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value789, _) => source_ml_source2_data_property_value789.as_slice() }), self::ml_array::number_index(index));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        {
                            let source_ml_source2_field790 = Vec::<self::JavaScriptSemanticValue>::new();
                            {
                                let source_ml_source2_field791 = self::ml_string::Text::from_units(vec![79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 75, 101, 121, 115, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                (source_ml_source2_field791.clone(), source_ml_source2_field790)
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
                                            let source_ml_source1_temporary_values113 = match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value792) => {
                                                    source_ml_source2_data_property_value792
                                                }
                                            };
                                            {
                                                let source_ml_source1_temporary_index113 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s793 = self::source_ml_source1_temporary_read_java_script_ordinary_own_keys_loop114(numeric, positions, text, symbols, source_ml_source1_temporary_values113, source_ml_source1_temporary_index113.clone());
                                                    match source_ml_source2_s793.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryOwnKeysLoop114Result::MlSource1TemporaryReadJavaScriptOrdinaryOwnKeysLoop114Done(numeric_2, text_2, symbols_2) => {
                                                            {
                                                                let source_ml_source2_field797 = {
                                                                    let source_ml_source2_concat_value794 = numeric_2;
                                                                    {
                                                                        let source_ml_source2_concat_value795 = text_2;
                                                                        {
                                                                            let source_ml_source2_concat_value796 = symbols_2;
                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source2_concat_value794), source_ml_source2_concat_value795), source_ml_source2_concat_value796)
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_field798 = self::ml_string::Text::from_units(vec![]);
                                                                    (source_ml_source2_field798.clone(), source_ml_source2_field797)
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
pub fn insert_java_script_ordinary_object(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, prototype: self::JavaScriptSemanticValue) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            {
                let source_ml_source2_field799 = heap.clone();
                {
                    let source_ml_source2_field800 = false;
                    {
                        let source_ml_source2_field801 = heap_error.clone();
                        (source_ml_source2_field801.clone(), source_ml_source2_field799.clone(), source_ml_source2_field800)
                    }
                }
            }
        } else {
            if ((((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value803, _) => source_ml_source2_data_property_value803.as_slice() }).len() as f64)) >= (4096f64)) {
                {
                    let source_ml_source2_field804 = heap.clone();
                    {
                        let source_ml_source2_field805 = false;
                        {
                            let source_ml_source2_field806 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]);
                            (source_ml_source2_field806.clone(), source_ml_source2_field804.clone(), source_ml_source2_field805)
                        }
                    }
                }
            } else {
                if ((((identity <= (self::ml::Big::from_i128(0))) || (identity > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value808, _) => {
                        source_ml_source2_data_property_value808
                    }
                }, identity.clone())) >= (0f64))) || self::test_java_script_ordinary_symbol(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value809) => {
                        source_ml_source2_data_property_value809
                    }
                }, identity.clone())) {
                    {
                        let source_ml_source2_field810 = heap.clone();
                        {
                            let source_ml_source2_field811 = false;
                            {
                                let source_ml_source2_field812 = self::ml_string::Text::from_units(vec![97, 32, 102, 114, 101, 115, 104, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                (source_ml_source2_field812.clone(), source_ml_source2_field810.clone(), source_ml_source2_field811)
                            }
                        }
                    }
                } else {
                    if (match prototype.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        self::JavaScriptSemanticValue::Null => {
                            false
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f813) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source2_f814) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f815) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source2_f816) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f817) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f818, source_ml_source2_f819) => {
                            true
                        }
                    } && match prototype.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        self::JavaScriptSemanticValue::Null => {
                            true
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f820) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source2_f821) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f822) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source2_f823) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f824) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f825, source_ml_source2_f826) => {
                            false
                        }
                    }) {
                        {
                            let source_ml_source2_field827 = heap.clone();
                            {
                                let source_ml_source2_field828 = false;
                                {
                                    let source_ml_source2_field829 = self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]);
                                    (source_ml_source2_field829.clone(), source_ml_source2_field827.clone(), source_ml_source2_field828)
                                }
                            }
                        }
                    } else {
                        {
                            let error = self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone());
                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                {
                                    let source_ml_source2_field830 = heap.clone();
                                    {
                                        let source_ml_source2_field831 = false;
                                        {
                                            let source_ml_source2_field832 = error.clone();
                                            (source_ml_source2_field832.clone(), source_ml_source2_field830.clone(), source_ml_source2_field831)
                                        }
                                    }
                                }
                            } else {
                                {
                                    let at = 0f64;
                                    {
                                        let source_ml_source1_temporary_values116 = match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value833, _) => {
                                                source_ml_source2_data_property_value833
                                            }
                                        };
                                        {
                                            let source_ml_source1_temporary_index116 = self::ml::Big::from_i128(0);
                                            {
                                                let at_2 = self::source_ml_source1_temporary_insert_java_script_ordinary_object_loop117(identity.clone(), at, source_ml_source1_temporary_values116, source_ml_source1_temporary_index116.clone());
                                                {
                                                    let object = self::JavaScriptOrdinaryObject::OrdinaryObject(identity.clone(), Box::new(prototype.clone()), true, false, Vec::<self::JavaScriptOrdinaryProperty>::new());
                                                    {
                                                        let objects = {
                                                            let source_ml_source2_concat_value838 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value835, _) => source_ml_source2_data_property_value835.as_slice() }), 0f64, at_2);
                                                            {
                                                                let source_ml_source2_concat_value839 = vec![object.clone()];
                                                                {
                                                                    let source_ml_source2_concat_value840 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value837, _) => source_ml_source2_data_property_value837.as_slice() }), at_2, f64::INFINITY);
                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), source_ml_source2_concat_value838), source_ml_source2_concat_value839), source_ml_source2_concat_value840)
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_field842 = self::JavaScriptOrdinaryHeap::OrdinaryHeap(objects, match heap.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value841) => {
                                                                    source_ml_source2_data_property_value841
                                                                }
                                                            });
                                                            {
                                                                let source_ml_source2_field843 = true;
                                                                {
                                                                    let source_ml_source2_field844 = self::ml_string::Text::from_units(vec![]);
                                                                    (source_ml_source2_field844.clone(), source_ml_source2_field842.clone(), source_ml_source2_field843)
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
pub fn register_java_script_ordinary_symbol(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool) {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            {
                let source_ml_source2_field845 = heap.clone();
                {
                    let source_ml_source2_field846 = false;
                    {
                        let source_ml_source2_field847 = error.clone();
                        (source_ml_source2_field847.clone(), source_ml_source2_field845.clone(), source_ml_source2_field846)
                    }
                }
            }
        } else {
            if ((((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value849) => source_ml_source2_data_property_value849.as_slice() }).len() as f64)) >= (16384f64)) {
                {
                    let source_ml_source2_field850 = heap.clone();
                    {
                        let source_ml_source2_field851 = false;
                        {
                            let source_ml_source2_field852 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]);
                            (source_ml_source2_field852.clone(), source_ml_source2_field850.clone(), source_ml_source2_field851)
                        }
                    }
                }
            } else {
                if ((((identity <= (self::ml::Big::from_i128(0))) || (identity > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value854, _) => {
                        source_ml_source2_data_property_value854
                    }
                }, identity.clone())) >= (0f64))) || self::test_java_script_ordinary_symbol(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value855) => {
                        source_ml_source2_data_property_value855
                    }
                }, identity.clone())) {
                    {
                        let source_ml_source2_field856 = heap.clone();
                        {
                            let source_ml_source2_field857 = false;
                            {
                                let source_ml_source2_field858 = self::ml_string::Text::from_units(vec![97, 32, 102, 114, 101, 115, 104, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                (source_ml_source2_field858.clone(), source_ml_source2_field856.clone(), source_ml_source2_field857)
                            }
                        }
                    }
                } else {
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_temporary_values118 = match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value859) => {
                                    source_ml_source2_data_property_value859
                                }
                            };
                            {
                                let source_ml_source1_temporary_index118 = self::ml::Big::from_i128(0);
                                {
                                    let at_2 = self::source_ml_source1_temporary_register_java_script_ordinary_symbol_loop119(identity.clone(), at, source_ml_source1_temporary_values118, source_ml_source1_temporary_index118.clone());
                                    {
                                        let symbols = {
                                            let source_ml_source2_concat_value864 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value861) => source_ml_source2_data_property_value861.as_slice() }), 0f64, at_2);
                                            {
                                                let source_ml_source2_concat_value865 = vec![identity.clone()];
                                                {
                                                    let source_ml_source2_concat_value866 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value863) => source_ml_source2_data_property_value863.as_slice() }), at_2, f64::INFINITY);
                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml::Big>::new(), source_ml_source2_concat_value864), source_ml_source2_concat_value865), source_ml_source2_concat_value866)
                                                }
                                            }
                                        };
                                        {
                                            let source_ml_source2_field868 = self::JavaScriptOrdinaryHeap::OrdinaryHeap(match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value867, _) => {
                                                    source_ml_source2_data_property_value867
                                                }
                                            }, symbols);
                                            {
                                                let source_ml_source2_field869 = true;
                                                {
                                                    let source_ml_source2_field870 = self::ml_string::Text::from_units(vec![]);
                                                    (source_ml_source2_field870.clone(), source_ml_source2_field868.clone(), source_ml_source2_field869)
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
            let source_ml_source2_field871 = self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101]);
            {
                let source_ml_source2_field872 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                {
                    let source_ml_source2_field873 = vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                    (source_ml_source2_field872.clone(), source_ml_source2_field871.clone(), source_ml_source2_field873)
                }
            }
        }
    } else {
        if (index == (1f64)) {
            {
                let source_ml_source2_field874 = self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101]);
                {
                    let source_ml_source2_field875 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                    {
                        let source_ml_source2_field876 = vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                        (source_ml_source2_field875.clone(), source_ml_source2_field874.clone(), source_ml_source2_field876)
                    }
                }
            }
        } else {
            if (index == (2f64)) {
                {
                    let source_ml_source2_field877 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                    {
                        let source_ml_source2_field878 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                        {
                            let source_ml_source2_field879 = vec![118f64, 97f64, 108f64, 117f64, 101f64];
                            (source_ml_source2_field878.clone(), source_ml_source2_field877.clone(), source_ml_source2_field879)
                        }
                    }
                }
            } else {
                if (index == (3f64)) {
                    {
                        let source_ml_source2_field880 = self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]);
                        {
                            let source_ml_source2_field881 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                            {
                                let source_ml_source2_field882 = vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64];
                                (source_ml_source2_field881.clone(), source_ml_source2_field880.clone(), source_ml_source2_field882)
                            }
                        }
                    }
                } else {
                    if (index == (4f64)) {
                        {
                            let source_ml_source2_field883 = self::ml_string::Text::from_units(vec![103, 101, 116]);
                            {
                                let source_ml_source2_field884 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                {
                                    let source_ml_source2_field885 = vec![103f64, 101f64, 116f64];
                                    (source_ml_source2_field884.clone(), source_ml_source2_field883.clone(), source_ml_source2_field885)
                                }
                            }
                        }
                    } else {
                        if (index == (5f64)) {
                            {
                                let source_ml_source2_field886 = self::ml_string::Text::from_units(vec![115, 101, 116]);
                                {
                                    let source_ml_source2_field887 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                    {
                                        let source_ml_source2_field888 = vec![115f64, 101f64, 116f64];
                                        (source_ml_source2_field887.clone(), source_ml_source2_field886.clone(), source_ml_source2_field888)
                                    }
                                }
                            }
                        } else {
                            {
                                let source_ml_source2_field889 = self::ml_string::Text::from_units(vec![]);
                                {
                                    let source_ml_source2_field890 = self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]);
                                    {
                                        let source_ml_source2_field891 = Vec::<f64>::new();
                                        (source_ml_source2_field890.clone(), source_ml_source2_field889.clone(), source_ml_source2_field891)
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
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f892) => {
                true
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f893, source_ml_source2_f894) => {
                true
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_empty_descriptor() -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source2_field895 = false;
        {
            let source_ml_source2_field896 = false;
            {
                let source_ml_source2_field897 = false;
                {
                    let source_ml_source2_field898 = false;
                    {
                        let source_ml_source2_field899 = false;
                        {
                            let source_ml_source2_field900 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source2_field901 = false;
                                {
                                    let source_ml_source2_field902 = false;
                                    {
                                        let source_ml_source2_field903 = false;
                                        {
                                            let source_ml_source2_field904 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field905 = false;
                                                {
                                                    let source_ml_source2_field906 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source2_field896, source_ml_source2_field895, source_ml_source2_field898, source_ml_source2_field897, source_ml_source2_field904.clone(), source_ml_source2_field903, source_ml_source2_field906.clone(), source_ml_source2_field905, source_ml_source2_field900.clone(), source_ml_source2_field899, source_ml_source2_field902, source_ml_source2_field901)
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
                let source_ml_source2_s908 = match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source2_data_property_value916, _, _, _) => {
                        let source_ml_source2_data_property_value916 = *source_ml_source2_data_property_value916;
                        source_ml_source2_data_property_value916.clone()
                    }
                };
                match source_ml_source2_s908.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f909) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f910) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f911) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f912) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f913) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f914, source_ml_source2_f915) => {
                        false
                    }
                }
            } {
                self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])
            } else {
                {
                    let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source2_data_property_value917, _, _, _) => {
                            let source_ml_source2_data_property_value917 = *source_ml_source2_data_property_value917;
                            source_ml_source2_data_property_value917.clone()
                        }
                    });
                    if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                        receiver_error.clone()
                    } else {
                        if ((((!(self::ml_math::is_safe_integer(match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value918, _, _) => {
                                source_ml_source2_data_property_value918
                            }
                        })) || ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value920, _, _) => {
                                source_ml_source2_data_property_value920
                            }
                        }) < (0f64))) || ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value922, _, _) => {
                                source_ml_source2_data_property_value922
                            }
                        }) > (6f64))) || (((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value924, _, _) => {
                                source_ml_source2_data_property_value924
                            }
                        }) == (0f64)) && (((1f64 / match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value926, _, _) => {
                                source_ml_source2_data_property_value926
                            }
                        })) < (0f64)))) || (match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source2_data_property_value927) => {
                                source_ml_source2_data_property_value927
                            }
                        } && ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value929, _, _) => {
                                source_ml_source2_data_property_value929
                            }
                        }) == (6f64)))) {
                            self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 102, 105, 101, 108, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                        } else {
                            {
                                let index = match state.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value930, _, _) => {
                                        source_ml_source2_data_property_value930
                                    }
                                };
                                {
                                    let source_ml_source2_s931 = self::source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop120(state.clone(), index);
                                    match source_ml_source2_s931.clone() {
                                        self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Done => {
                                            {
                                                let descriptor = match state.clone() {
                                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value932, _) => {
                                                        source_ml_source2_data_property_value932.clone()
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
                                        self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Return(source_ml_source1_temporary_result120) => {
                                            source_ml_source1_temporary_result120.clone()
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
