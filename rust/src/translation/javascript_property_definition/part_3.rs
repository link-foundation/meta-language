// Generated from complete js/src/translation/javascript-property-definition.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result;
use super::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result;
use super::create_java_script_property_descriptor;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::plan_java_script_property_definition;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_define_java_script_ordinary_property_loop20;
use super::source_ml_temporary_find_java_script_ordinary_property_loop7;
use super::source_ml_temporary_locate_java_script_ordinary_property_loop18;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop11;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop12;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop13;
use super::test_java_script_ordinary_symbol;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_value_error(heap: self::JavaScriptOrdinaryHeap, value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    {
        let error = self::read_java_script_semantic_value_error(value.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            error.clone()
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptSemanticValue::Null => {
                    self::ml_string::Text::from_units(vec![])
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f153) => {
                    {
                        let source_ml_source1_f169 = source_ml_source1_f153;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f154) => {
                    {
                        let source_ml_source1_f175 = source_ml_source1_f154;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f155) => {
                    {
                        let source_ml_source1_f181 = source_ml_source1_f155.clone();
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f156) => {
                    {
                        let source_ml_source1_f187 = source_ml_source1_f156;
                        self::ml_string::Text::from_units(vec![])
                    }
                }
                self::JavaScriptSemanticValue::Symbol(identity) => {
                    if self::test_java_script_ordinary_symbol(match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value189) => {
                            source_ml_source1_data_property_value189
                        }
                    }, identity.clone()) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f157, source_ml_source1_f158) => {
                    {
                        let identity_2 = source_ml_source1_f157.clone();
                        {
                            let callable = source_ml_source1_f158;
                            {
                                let at = self::find_java_script_ordinary_object(match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value195, _) => {
                                        source_ml_source1_data_property_value195
                                    }
                                }, identity_2.clone());
                                if (at < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                } else {
                                    {
                                        let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value196, _) => source_ml_source1_data_property_value196.as_slice() }), self::ml_array::number_index(at));
                                        if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (({
                                            let source_ml_source1_nullish_field_receiver200 = (object.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver200.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source1_nullish_field_present201 = ((source_ml_source1_nullish_field_receiver200.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present201.clone() {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value202, _) => {
                                                            source_ml_source1_data_property_value202
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                            }
                                        }) == callable)) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121, 32, 109, 117, 115, 116, 32, 97, 103, 114, 101, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121])
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
pub fn read_java_script_ordinary_key_error(heap: self::JavaScriptOrdinaryHeap, key: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    if (match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f203) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f204) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f205) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f206) => {
            false
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f207) => {
            true
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f208, source_ml_source1_f209) => {
            true
        }
    } && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            true
        }
        self::JavaScriptSemanticValue::Null => {
            true
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f210) => {
            true
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f211) => {
            true
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f212) => {
            true
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f213) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f214) => {
            false
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f215, source_ml_source1_f216) => {
            true
        }
    }) {
        self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 101, 120, 116, 32, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 119, 105, 116, 104, 111, 117, 116, 32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 99, 111, 101, 114, 99, 105, 111, 110])
    } else {
        self::read_java_script_ordinary_value_error(heap.clone(), key.clone())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_ordinary_property(properties: Vec<self::JavaScriptOrdinaryProperty>, key: self::JavaScriptSemanticValue) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source1_s217 = self::source_ml_temporary_find_java_script_ordinary_property_loop7(properties, key.clone(), at);
            match source_ml_source1_s217.clone() {
                self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Done => {
                    (-1f64)
                }
                self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Return(source_ml_temporary_result7) => {
                    source_ml_temporary_result7
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_heap_error(heap: self::JavaScriptOrdinaryHeap) -> self::ml_string::Text {
    if (((((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value219, _) => source_ml_source1_data_property_value219.as_slice() }).len() as f64)) > (4096f64)) || ((((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value221) => source_ml_source1_data_property_value221.as_slice() }).len() as f64)) > (16384f64))) {
        self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116])
    } else {
        {
            let previous = self::ml::Big::from_i128(0);
            {
                let source_ml_temporary_values8 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value222, _) => {
                        source_ml_source1_data_property_value222
                    }
                };
                {
                    let source_ml_temporary_index8 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_s223 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop11(previous.clone(), source_ml_temporary_values8, source_ml_temporary_index8.clone());
                        match source_ml_source1_s223.clone() {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Done(previous_2) => {
                                {
                                    let previous_3 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_temporary_values9 = match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value224) => {
                                                source_ml_source1_data_property_value224
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index9 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source1_s225 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop12(heap.clone(), previous_3.clone(), source_ml_temporary_values9, source_ml_temporary_index9.clone());
                                                match source_ml_source1_s225.clone() {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Done => {
                                                        {
                                                            let property_count = 0f64;
                                                            {
                                                                let source_ml_temporary_values10 = match heap.clone() {
                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value226, _) => {
                                                                        source_ml_source1_data_property_value226
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_temporary_index10 = self::ml::Big::from_i128(0);
                                                                    {
                                                                        let source_ml_source1_s227 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop13(heap.clone(), property_count, source_ml_temporary_values10, source_ml_temporary_index10.clone());
                                                                        match source_ml_source1_s227.clone() {
                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Done => {
                                                                                self::ml_string::Text::from_units(vec![])
                                                                            }
                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(source_ml_temporary_result13) => {
                                                                                source_ml_temporary_result13.clone()
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Return(source_ml_temporary_result17) => {
                                                        source_ml_temporary_result17.clone()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Return(source_ml_temporary_result17_2) => {
                                source_ml_temporary_result17_2.clone()
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
pub fn replace_java_script_ordinary_object(heap: self::JavaScriptOrdinaryHeap, replacement: self::JavaScriptOrdinaryObject) -> self::JavaScriptOrdinaryHeap {
    {
        let objects = { let ml_values1 = match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value228, _) => {
                source_ml_source1_data_property_value228
            }
        }; let ml_callback1 = { let replacement = replacement.clone(); std::rc::Rc::new(move |object: self::JavaScriptOrdinaryObject| -> self::JavaScriptOrdinaryObject { if ((match object.clone() {
            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value231, _, _, _, _) => {
                source_ml_source1_data_property_value231.clone()
            }
        }) == (match replacement.clone() {
            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value232, _, _, _, _) => {
                source_ml_source1_data_property_value232.clone()
            }
        })) {
            replacement.clone()
        } else {
            object.clone()
        } }) }; ml_values1.iter().cloned().enumerate().map(|(ml_index1, ml_value1)| ml_callback1(ml_value1)).collect::<Vec<_>>() };
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(objects, match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value233) => {
                source_ml_source1_data_property_value233
            }
        })
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn locate_java_script_ordinary_property(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue) -> ((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), bool, self::ml::Big) {
    {
        let current = identity.clone();
        {
            let depth = 0f64;
            {
                let source_ml_source1_s234 = self::source_ml_temporary_locate_java_script_ordinary_property_loop18(heap.clone(), key.clone(), current.clone(), depth);
                match source_ml_source1_s234.clone() {
                    self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done => {
                        {
                            let source_ml_source1_field235 = false;
                            {
                                let source_ml_source1_field236 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source1_field237 = self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false);
                                    (source_ml_source1_field237.clone(), source_ml_source1_field235, source_ml_source1_field236.clone())
                                }
                            }
                        }
                    }
                    self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Return(source_ml_temporary_result18) => {
                        source_ml_temporary_result18.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_ordinary_get(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue) -> (self::ml_string::Text, Vec<self::JavaScriptSemanticValue>, self::JavaScriptSemanticValue, self::ml_string::Text, self::JavaScriptSemanticValue, self::JavaScriptSemanticValue) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
            {
                let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                {
                    let error = if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                        heap_error.clone()
                    } else {
                        if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                            key_error.clone()
                        } else {
                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                receiver_error.clone()
                            } else {
                                if ((self::find_java_script_ordinary_object(match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value239, _) => {
                                        source_ml_source1_data_property_value239
                                    }
                                }, identity.clone())) < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121])
                                } else {
                                    self::ml_string::Text::from_units(vec![])
                                }
                            }
                        }
                    };
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        {
                            let source_ml_source1_field240 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                            {
                                let source_ml_source1_field241 = error.clone();
                                {
                                    let source_ml_source1_field242 = self::JavaScriptSemanticValue::Undefined;
                                    {
                                        let source_ml_source1_field243 = self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source1_field244 = receiver.clone();
                                            {
                                                let source_ml_source1_field245 = Vec::<self::JavaScriptSemanticValue>::new();
                                                (source_ml_source1_field240.clone(), source_ml_source1_field245, source_ml_source1_field243.clone(), source_ml_source1_field241.clone(), source_ml_source1_field244.clone(), source_ml_source1_field242.clone())
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let property = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                            if !((property.clone()).1) {
                                {
                                    let source_ml_source1_field246 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                    {
                                        let source_ml_source1_field247 = self::ml_string::Text::from_units(vec![]);
                                        {
                                            let source_ml_source1_field248 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field249 = self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source1_field250 = receiver.clone();
                                                    {
                                                        let source_ml_source1_field251 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (source_ml_source1_field246.clone(), source_ml_source1_field251, source_ml_source1_field249.clone(), source_ml_source1_field247.clone(), source_ml_source1_field250.clone(), source_ml_source1_field248.clone())
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if ((property.clone()).0).9 {
                                    {
                                        let source_ml_source1_field252 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                        {
                                            let source_ml_source1_field253 = self::ml_string::Text::from_units(vec![]);
                                            {
                                                let source_ml_source1_field254 = ((property.clone()).0).8;
                                                {
                                                    let source_ml_source1_field255 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source1_field256 = receiver.clone();
                                                        {
                                                            let source_ml_source1_field257 = Vec::<self::JavaScriptSemanticValue>::new();
                                                            (source_ml_source1_field252.clone(), source_ml_source1_field257, source_ml_source1_field255.clone(), source_ml_source1_field253.clone(), source_ml_source1_field256.clone(), source_ml_source1_field254.clone())
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let getter = ((property.clone()).0).4;
                                        match getter.clone() {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                {
                                                    let source_ml_source1_field265 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                                    {
                                                        let source_ml_source1_field266 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field267 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field268 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source1_field269 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field270 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field265.clone(), source_ml_source1_field270, source_ml_source1_field268.clone(), source_ml_source1_field266.clone(), source_ml_source1_field269.clone(), source_ml_source1_field267.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                {
                                                    let source_ml_source1_field271 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field272 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field273 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field274 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field275 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field276 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field271.clone(), source_ml_source1_field276, source_ml_source1_field274.clone(), source_ml_source1_field272.clone(), source_ml_source1_field275.clone(), source_ml_source1_field273.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f258) => {
                                                {
                                                    let source_ml_source1_field277 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field278 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field279 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field280 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field281 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field282 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field277.clone(), source_ml_source1_field282, source_ml_source1_field280.clone(), source_ml_source1_field278.clone(), source_ml_source1_field281.clone(), source_ml_source1_field279.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Number(source_ml_source1_f259) => {
                                                {
                                                    let source_ml_source1_field283 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field284 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field285 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field286 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field287 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field288 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field283.clone(), source_ml_source1_field288, source_ml_source1_field286.clone(), source_ml_source1_field284.clone(), source_ml_source1_field287.clone(), source_ml_source1_field285.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f260) => {
                                                {
                                                    let source_ml_source1_field289 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field290 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field291 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field292 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field293 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field294 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field289.clone(), source_ml_source1_field294, source_ml_source1_field292.clone(), source_ml_source1_field290.clone(), source_ml_source1_field293.clone(), source_ml_source1_field291.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Text(source_ml_source1_f261) => {
                                                {
                                                    let source_ml_source1_field295 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field296 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field297 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field298 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field299 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field300 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field295.clone(), source_ml_source1_field300, source_ml_source1_field298.clone(), source_ml_source1_field296.clone(), source_ml_source1_field299.clone(), source_ml_source1_field297.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f262) => {
                                                {
                                                    let source_ml_source1_field301 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field302 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field303 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field304 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field305 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field306 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field301.clone(), source_ml_source1_field306, source_ml_source1_field304.clone(), source_ml_source1_field302.clone(), source_ml_source1_field305.clone(), source_ml_source1_field303.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Object(source_ml_source1_f263, source_ml_source1_f264) => {
                                                {
                                                    let source_ml_source1_field307 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source1_field308 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source1_field309 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field310 = getter.clone();
                                                                {
                                                                    let source_ml_source1_field311 = receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field312 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field307.clone(), source_ml_source1_field312, source_ml_source1_field310.clone(), source_ml_source1_field308.clone(), source_ml_source1_field311.clone(), source_ml_source1_field309.clone())
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
pub fn define_java_script_ordinary_property(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue, proposed: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool, Vec<self::ml_string::Text>) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
            {
                let error = if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                    heap_error.clone()
                } else {
                    key_error.clone()
                };
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    {
                        let source_ml_source1_field313 = heap.clone();
                        {
                            let source_ml_source1_field314 = false;
                            {
                                let source_ml_source1_field315 = error.clone();
                                {
                                    let source_ml_source1_field316 = Vec::<self::ml_string::Text>::new();
                                    (source_ml_source1_field315.clone(), source_ml_source1_field313.clone(), source_ml_source1_field314, source_ml_source1_field316)
                                }
                            }
                        }
                    }
                } else {
                    if ((proposed.clone()).9 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
                        {
                            let source_ml_source1_field317 = heap.clone();
                            {
                                let source_ml_source1_field318 = false;
                                {
                                    let source_ml_source1_field319 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 118, 97, 108, 117, 101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                    {
                                        let source_ml_source1_field320 = Vec::<self::ml_string::Text>::new();
                                        (source_ml_source1_field319.clone(), source_ml_source1_field317.clone(), source_ml_source1_field318, source_ml_source1_field320)
                                    }
                                }
                            }
                        }
                    } else {
                        if ((proposed.clone()).5 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).4)) != (self::ml_string::Text::from_units(vec![])))) {
                            {
                                let source_ml_source1_field321 = heap.clone();
                                {
                                    let source_ml_source1_field322 = false;
                                    {
                                        let source_ml_source1_field323 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 103, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                        {
                                            let source_ml_source1_field324 = Vec::<self::ml_string::Text>::new();
                                            (source_ml_source1_field323.clone(), source_ml_source1_field321.clone(), source_ml_source1_field322, source_ml_source1_field324)
                                        }
                                    }
                                }
                            }
                        } else {
                            if ((proposed.clone()).7 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).6)) != (self::ml_string::Text::from_units(vec![])))) {
                                {
                                    let source_ml_source1_field325 = heap.clone();
                                    {
                                        let source_ml_source1_field326 = false;
                                        {
                                            let source_ml_source1_field327 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 115, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                            {
                                                let source_ml_source1_field328 = Vec::<self::ml_string::Text>::new();
                                                (source_ml_source1_field327.clone(), source_ml_source1_field325.clone(), source_ml_source1_field326, source_ml_source1_field328)
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let index = self::find_java_script_ordinary_object(match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value329, _) => {
                                            source_ml_source1_data_property_value329
                                        }
                                    }, identity.clone());
                                    {
                                        let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value330, _) => source_ml_source1_data_property_value330.as_slice() }), self::ml_array::number_index(index));
                                        if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            {
                                                let source_ml_source1_field331 = heap.clone();
                                                {
                                                    let source_ml_source1_field332 = false;
                                                    {
                                                        let source_ml_source1_field333 = self::ml_string::Text::from_units(vec![68, 101, 102, 105, 110, 101, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                                        {
                                                            let source_ml_source1_field334 = Vec::<self::ml_string::Text>::new();
                                                            (source_ml_source1_field333.clone(), source_ml_source1_field331.clone(), source_ml_source1_field332, source_ml_source1_field334)
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value335) => {
                                                        source_ml_source1_data_property_value335
                                                    }
                                                }, key.clone());
                                                {
                                                    let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value336) => {
                                                            source_ml_source1_data_property_value336
                                                        }
                                                    }, self::ml_array::number_index(at));
                                                    {
                                                        let current = if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false)
                                                        } else {
                                                            match property.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value337) => {
                                                                    source_ml_source1_data_property_value337.clone()
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let transition = self::plan_java_script_property_definition(!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }), match object.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value338, _, _) => {
                                                                    source_ml_source1_data_property_value338
                                                                }
                                                            }, current.clone(), proposed.clone());
                                                            if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                {
                                                                    let source_ml_source1_field339 = heap.clone();
                                                                    {
                                                                        let source_ml_source1_field340 = false;
                                                                        {
                                                                            let source_ml_source1_field341 = (transition.clone()).2;
                                                                            {
                                                                                let source_ml_source1_field342 = Vec::<self::ml_string::Text>::new();
                                                                                (source_ml_source1_field341.clone(), source_ml_source1_field339.clone(), source_ml_source1_field340, source_ml_source1_field342)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![107, 101, 101, 112]))) {
                                                                    {
                                                                        let source_ml_source1_field343 = heap.clone();
                                                                        {
                                                                            let source_ml_source1_field344 = true;
                                                                            {
                                                                                let source_ml_source1_field345 = self::ml_string::Text::from_units(vec![]);
                                                                                {
                                                                                    let source_ml_source1_field346 = Vec::<self::ml_string::Text>::new();
                                                                                    (source_ml_source1_field345.clone(), source_ml_source1_field343.clone(), source_ml_source1_field344, source_ml_source1_field346)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    if ((at < (0f64)) && ((((match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value348) => {
                                                                            source_ml_source1_data_property_value348
                                                                        }
                                                                    }).len() as f64)) >= (4096f64))) {
                                                                        {
                                                                            let source_ml_source1_field349 = heap.clone();
                                                                            {
                                                                                let source_ml_source1_field350 = false;
                                                                                {
                                                                                    let source_ml_source1_field351 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]);
                                                                                    {
                                                                                        let source_ml_source1_field352 = Vec::<self::ml_string::Text>::new();
                                                                                        (source_ml_source1_field351.clone(), source_ml_source1_field349.clone(), source_ml_source1_field350, source_ml_source1_field352)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let count = 0f64;
                                                                            {
                                                                                let source_ml_temporary_values19 = match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value353, _) => {
                                                                                        source_ml_source1_data_property_value353
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_temporary_index19 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let count_2 = self::source_ml_temporary_define_java_script_ordinary_property_loop20(count, source_ml_temporary_values19, source_ml_temporary_index19.clone());
                                                                                        if ((at < (0f64)) && (count_2 >= (16384f64))) {
                                                                                            {
                                                                                                let source_ml_source1_field354 = heap.clone();
                                                                                                {
                                                                                                    let source_ml_source1_field355 = false;
                                                                                                    {
                                                                                                        let source_ml_source1_field356 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]);
                                                                                                        {
                                                                                                            let source_ml_source1_field357 = Vec::<self::ml_string::Text>::new();
                                                                                                            (source_ml_source1_field356.clone(), source_ml_source1_field354.clone(), source_ml_source1_field355, source_ml_source1_field357)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            {
                                                                                                let updated = self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(key.clone()), (transition.clone()).0);
                                                                                                {
                                                                                                    let properties = if (at < (0f64)) {
                                                                                                        {
                                                                                                            let source_ml_source1_concat_value359 = match object.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value358) => {
                                                                                                                    source_ml_source1_data_property_value358
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_concat_value360 = vec![updated.clone()];
                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryProperty>::new(), source_ml_source1_concat_value359), source_ml_source1_concat_value360)
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        { let ml_values2 = match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value361) => {
                                                                                                                source_ml_source1_data_property_value361
                                                                                                            }
                                                                                                        }; let ml_callback2 = { let at = at; let updated = updated.clone(); std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> self::JavaScriptOrdinaryProperty { if (position == at) {
                                                                                                            updated.clone()
                                                                                                        } else {
                                                                                                            entry.clone()
                                                                                                        } }) }; ml_values2.iter().cloned().enumerate().map(|(ml_index2, ml_value2)| ml_callback2(ml_value2, (ml_index2 as f64))).collect::<Vec<_>>() }
                                                                                                    };
                                                                                                    {
                                                                                                        let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value362, _, _, _, _) => {
                                                                                                                source_ml_source1_data_property_value362.clone()
                                                                                                            }
                                                                                                        }, Box::new(match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value363, _, _, _) => {
                                                                                                                let source_ml_source1_data_property_value363 = *source_ml_source1_data_property_value363;
                                                                                                                source_ml_source1_data_property_value363.clone()
                                                                                                            }
                                                                                                        }), match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value364, _, _) => {
                                                                                                                source_ml_source1_data_property_value364
                                                                                                            }
                                                                                                        }, match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value365, _) => {
                                                                                                                source_ml_source1_data_property_value365
                                                                                                            }
                                                                                                        }, properties);
                                                                                                        {
                                                                                                            let source_ml_source1_field366 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                                                            {
                                                                                                                let source_ml_source1_field367 = true;
                                                                                                                {
                                                                                                                    let source_ml_source1_field368 = self::ml_string::Text::from_units(vec![]);
                                                                                                                    {
                                                                                                                        let source_ml_source1_field369 = (transition.clone()).3;
                                                                                                                        (source_ml_source1_field368.clone(), source_ml_source1_field366.clone(), source_ml_source1_field367, source_ml_source1_field369)
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
