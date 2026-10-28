// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result;
use super::create_java_script_property_descriptor;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_property_definition;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_source1_temporary_define_java_script_ordinary_property_loop20;
use super::source_ml_source1_temporary_locate_java_script_ordinary_property_loop18;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn locate_java_script_ordinary_property(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue) -> ((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), bool, self::ml::Big) {
    {
        let depth = 0f64;
        {
            let source_ml_source2_s234 = self::source_ml_source1_temporary_locate_java_script_ordinary_property_loop18(heap.clone(), key.clone(), identity.clone(), depth);
            match source_ml_source2_s234.clone() {
                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done => {
                    {
                        let source_ml_source2_field235 = false;
                        {
                            let source_ml_source2_field236 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_field237 = self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false);
                                (source_ml_source2_field237.clone(), source_ml_source2_field235, source_ml_source2_field236.clone())
                            }
                        }
                    }
                }
                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Return(source_ml_source1_temporary_result18) => {
                    source_ml_source1_temporary_result18.clone()
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
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value239, _) => {
                                        source_ml_source2_data_property_value239
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
                            let source_ml_source2_field240 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                            {
                                let source_ml_source2_field242 = self::JavaScriptSemanticValue::Undefined;
                                {
                                    let source_ml_source2_field243 = self::JavaScriptSemanticValue::Undefined;
                                    {
                                        let source_ml_source2_field245 = Vec::<self::JavaScriptSemanticValue>::new();
                                        (source_ml_source2_field240.clone(), source_ml_source2_field245, source_ml_source2_field243.clone(), error.clone(), receiver.clone(), source_ml_source2_field242.clone())
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let property = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                            if !((property.clone()).1) {
                                {
                                    let source_ml_source2_field246 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                    {
                                        let source_ml_source2_field247 = self::ml_string::Text::from_units(vec![]);
                                        {
                                            let source_ml_source2_field248 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field249 = self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source2_field251 = Vec::<self::JavaScriptSemanticValue>::new();
                                                    (source_ml_source2_field246.clone(), source_ml_source2_field251, source_ml_source2_field249.clone(), source_ml_source2_field247.clone(), receiver.clone(), source_ml_source2_field248.clone())
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if ((property.clone()).0).9 {
                                    {
                                        let source_ml_source2_field252 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                        {
                                            let source_ml_source2_field253 = self::ml_string::Text::from_units(vec![]);
                                            {
                                                let source_ml_source2_field254 = ((property.clone()).0).8;
                                                {
                                                    let source_ml_source2_field255 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source2_field257 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (source_ml_source2_field252.clone(), source_ml_source2_field257, source_ml_source2_field255.clone(), source_ml_source2_field253.clone(), receiver.clone(), source_ml_source2_field254.clone())
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
                                                    let source_ml_source2_field265 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                                    {
                                                        let source_ml_source2_field266 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field267 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field268 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source2_field270 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                    (source_ml_source2_field265.clone(), source_ml_source2_field270, source_ml_source2_field268.clone(), source_ml_source2_field266.clone(), receiver.clone(), source_ml_source2_field267.clone())
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                {
                                                    let source_ml_source2_field271 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field272 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field273 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field276 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                (source_ml_source2_field271.clone(), source_ml_source2_field276, getter.clone(), source_ml_source2_field272.clone(), receiver.clone(), source_ml_source2_field273.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                {
                                                    let source_ml_source2_field277 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field278 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field279 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field282 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                (source_ml_source2_field277.clone(), source_ml_source2_field282, getter.clone(), source_ml_source2_field278.clone(), receiver.clone(), source_ml_source2_field279.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Number(_) => {
                                                {
                                                    let source_ml_source2_field283 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field284 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field285 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field288 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                (source_ml_source2_field283.clone(), source_ml_source2_field288, getter.clone(), source_ml_source2_field284.clone(), receiver.clone(), source_ml_source2_field285.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                {
                                                    let source_ml_source2_field289 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field290 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field291 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field294 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                (source_ml_source2_field289.clone(), source_ml_source2_field294, getter.clone(), source_ml_source2_field290.clone(), receiver.clone(), source_ml_source2_field291.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Text(_) => {
                                                {
                                                    let source_ml_source2_field295 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field296 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field297 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field300 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                (source_ml_source2_field295.clone(), source_ml_source2_field300, getter.clone(), source_ml_source2_field296.clone(), receiver.clone(), source_ml_source2_field297.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                {
                                                    let source_ml_source2_field301 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field302 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field303 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field306 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                (source_ml_source2_field301.clone(), source_ml_source2_field306, getter.clone(), source_ml_source2_field302.clone(), receiver.clone(), source_ml_source2_field303.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                {
                                                    let source_ml_source2_field307 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                    {
                                                        let source_ml_source2_field308 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field309 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field312 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                (source_ml_source2_field307.clone(), source_ml_source2_field312, getter.clone(), source_ml_source2_field308.clone(), receiver.clone(), source_ml_source2_field309.clone())
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
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _) => {
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
                                let source_ml_source2_field314 = false;
                                {
                                    let source_ml_source2_field316 = Vec::<self::ml_string::Text>::new();
                                    (error.clone(), heap.clone(), source_ml_source2_field314, source_ml_source2_field316)
                                }
                            }
                        } else {
                            if ((proposed.clone()).9 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
                                {
                                    let source_ml_source2_field318 = false;
                                    {
                                        let source_ml_source2_field319 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 118, 97, 108, 117, 101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                        {
                                            let source_ml_source2_field320 = Vec::<self::ml_string::Text>::new();
                                            (source_ml_source2_field319.clone(), heap.clone(), source_ml_source2_field318, source_ml_source2_field320)
                                        }
                                    }
                                }
                            } else {
                                if ((proposed.clone()).5 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).4)) != (self::ml_string::Text::from_units(vec![])))) {
                                    {
                                        let source_ml_source2_field322 = false;
                                        {
                                            let source_ml_source2_field323 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 103, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                            {
                                                let source_ml_source2_field324 = Vec::<self::ml_string::Text>::new();
                                                (source_ml_source2_field323.clone(), heap.clone(), source_ml_source2_field322, source_ml_source2_field324)
                                            }
                                        }
                                    }
                                } else {
                                    if ((proposed.clone()).7 && ((self::read_java_script_ordinary_value_error(heap.clone(), (proposed.clone()).6)) != (self::ml_string::Text::from_units(vec![])))) {
                                        {
                                            let source_ml_source2_field326 = false;
                                            {
                                                let source_ml_source2_field327 = self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 115, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]);
                                                {
                                                    let source_ml_source2_field328 = Vec::<self::ml_string::Text>::new();
                                                    (source_ml_source2_field327.clone(), heap.clone(), source_ml_source2_field326, source_ml_source2_field328)
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                            {
                                                let object = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(index));
                                                if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    {
                                                        let source_ml_source2_field332 = false;
                                                        {
                                                            let source_ml_source2_field333 = self::ml_string::Text::from_units(vec![68, 101, 102, 105, 110, 101, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                                            {
                                                                let source_ml_source2_field334 = Vec::<self::ml_string::Text>::new();
                                                                (source_ml_source2_field333.clone(), heap.clone(), source_ml_source2_field332, source_ml_source2_field334)
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value335) => {
                                                                source_ml_source2_data_property_value335
                                                            }
                                                        }, key.clone());
                                                        {
                                                            let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value336) => {
                                                                    source_ml_source2_data_property_value336
                                                                }
                                                            }, self::ml_array::number_index(at));
                                                            {
                                                                let current = if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false)
                                                                } else {
                                                                    match property.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value337) => {
                                                                            source_ml_source2_data_property_value337.clone()
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let transition = self::plan_java_script_property_definition(!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }), match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value338, _, _) => {
                                                                            source_ml_source2_data_property_value338
                                                                        }
                                                                    }, current.clone(), proposed.clone());
                                                                    if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                        {
                                                                            let source_ml_source2_field340 = false;
                                                                            {
                                                                                let source_ml_source2_field341 = (transition.clone()).2;
                                                                                {
                                                                                    let source_ml_source2_field342 = Vec::<self::ml_string::Text>::new();
                                                                                    (source_ml_source2_field341.clone(), heap.clone(), source_ml_source2_field340, source_ml_source2_field342)
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![107, 101, 101, 112]))) {
                                                                            {
                                                                                let source_ml_source2_field344 = true;
                                                                                {
                                                                                    let source_ml_source2_field345 = self::ml_string::Text::from_units(vec![]);
                                                                                    {
                                                                                        let source_ml_source2_field346 = Vec::<self::ml_string::Text>::new();
                                                                                        (source_ml_source2_field345.clone(), heap.clone(), source_ml_source2_field344, source_ml_source2_field346)
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            if ((at < (0f64)) && ((((match object.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value348) => {
                                                                                    source_ml_source2_data_property_value348
                                                                                }
                                                                            }).len() as f64)) >= (4096f64))) {
                                                                                {
                                                                                    let source_ml_source2_field350 = false;
                                                                                    {
                                                                                        let source_ml_source2_field351 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]);
                                                                                        {
                                                                                            let source_ml_source2_field352 = Vec::<self::ml_string::Text>::new();
                                                                                            (source_ml_source2_field351.clone(), heap.clone(), source_ml_source2_field350, source_ml_source2_field352)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                {
                                                                                    let count = 0f64;
                                                                                    {
                                                                                        let source_ml_source1_temporary_values19 = shared_constructor_field1;
                                                                                        {
                                                                                            let source_ml_source1_temporary_index19 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let count_2 = self::source_ml_source1_temporary_define_java_script_ordinary_property_loop20(count, source_ml_source1_temporary_values19, source_ml_source1_temporary_index19.clone());
                                                                                                if ((at < (0f64)) && (count_2 >= (16384f64))) {
                                                                                                    {
                                                                                                        let source_ml_source2_field355 = false;
                                                                                                        {
                                                                                                            let source_ml_source2_field356 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]);
                                                                                                            {
                                                                                                                let source_ml_source2_field357 = Vec::<self::ml_string::Text>::new();
                                                                                                                (source_ml_source2_field356.clone(), heap.clone(), source_ml_source2_field355, source_ml_source2_field357)
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    {
                                                                                                        let updated = self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(key.clone()), (transition.clone()).0);
                                                                                                        {
                                                                                                            let properties = if (at < (0f64)) {
                                                                                                                {
                                                                                                                    let source_ml_source2_concat_value359 = match object.clone().expect("a proven present value") {
                                                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value358) => {
                                                                                                                            source_ml_source2_data_property_value358
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_concat_value360 = vec![updated.clone()];
                                                                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryProperty>::new(), source_ml_source2_concat_value359), source_ml_source2_concat_value360)
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                { let ml_values2 = match object.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value361) => {
                                                                                                                        source_ml_source2_data_property_value361
                                                                                                                    }
                                                                                                                }; let ml_callback2 = { let at = at; let updated = updated.clone(); std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> self::JavaScriptOrdinaryProperty { if (position == at) {
                                                                                                                    updated.clone()
                                                                                                                } else {
                                                                                                                    entry.clone()
                                                                                                                } }) }; ml_values2.iter().cloned().enumerate().map(|(ml_index2, ml_value2)| ml_callback2(ml_value2, (ml_index2 as f64))).collect::<Vec<_>>() }
                                                                                                            };
                                                                                                            {
                                                                                                                let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value362, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value362.clone()
                                                                                                                    }
                                                                                                                }, Box::new(match object.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value363, _, _, _) => {
                                                                                                                        let source_ml_source2_data_property_value363 = *source_ml_source2_data_property_value363;
                                                                                                                        source_ml_source2_data_property_value363.clone()
                                                                                                                    }
                                                                                                                }), match object.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value364, _, _) => {
                                                                                                                        source_ml_source2_data_property_value364
                                                                                                                    }
                                                                                                                }, match object.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source2_data_property_value365, _) => {
                                                                                                                        source_ml_source2_data_property_value365
                                                                                                                    }
                                                                                                                }, properties);
                                                                                                                {
                                                                                                                    let source_ml_source2_field366 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                                                                    {
                                                                                                                        let source_ml_source2_field367 = true;
                                                                                                                        {
                                                                                                                            let source_ml_source2_field368 = self::ml_string::Text::from_units(vec![]);
                                                                                                                            {
                                                                                                                                let source_ml_source2_field369 = (transition.clone()).3;
                                                                                                                                (source_ml_source2_field368.clone(), source_ml_source2_field366.clone(), source_ml_source2_field367, source_ml_source2_field369)
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_ordinary_set_descriptor(value: self::JavaScriptSemanticValue, creation: bool) -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source2_field458 = true;
        {
            let source_ml_source2_field462 = false;
            {
                let source_ml_source2_field463 = self::JavaScriptSemanticValue::Undefined;
                {
                    let source_ml_source2_field464 = false;
                    {
                        let source_ml_source2_field465 = self::JavaScriptSemanticValue::Undefined;
                        (creation, creation, creation, creation, source_ml_source2_field463.clone(), source_ml_source2_field462, source_ml_source2_field465.clone(), source_ml_source2_field464, value.clone(), source_ml_source2_field458, creation, creation)
                    }
                }
            }
        }
    }
}
