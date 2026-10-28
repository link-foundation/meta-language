// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
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
use super::plan_java_script_ordinary_get;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_ordinary_set(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue, value: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue) -> (self::ml_string::Text, Vec<self::JavaScriptSemanticValue>, self::JavaScriptSemanticValue, self::ml_string::Text, self::JavaScriptOrdinaryHeap, self::JavaScriptSemanticValue, bool) {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _) => {
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                {
                    let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
                    {
                        let value_error = self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                        {
                            let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                            {
                                let error = if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                                    heap_error.clone()
                                } else {
                                    if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                        key_error.clone()
                                    } else {
                                        if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                                            value_error.clone()
                                        } else {
                                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                                receiver_error.clone()
                                            } else {
                                                if ((self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone())) < (0f64)) {
                                                    self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![])
                                                }
                                            }
                                        }
                                    }
                                };
                                if (error != (self::ml_string::Text::from_units(vec![]))) {
                                    {
                                        let source_ml_source2_field469 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                        {
                                            let source_ml_source2_field471 = false;
                                            {
                                                let source_ml_source2_field472 = self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source2_field474 = Vec::<self::JavaScriptSemanticValue>::new();
                                                    (source_ml_source2_field469.clone(), source_ml_source2_field474, source_ml_source2_field472.clone(), error.clone(), heap.clone(), receiver.clone(), source_ml_source2_field471)
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let located = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                                        {
                                            let own = if (located.clone()).1 {
                                                (located.clone()).0
                                            } else {
                                                self::create_java_script_ordinary_set_descriptor(self::JavaScriptSemanticValue::Undefined, true)
                                            };
                                            if (own.clone()).5 {
                                                {
                                                    let setter = (own.clone()).6;
                                                    match setter.clone() {
                                                        self::JavaScriptSemanticValue::Undefined => {
                                                            {
                                                                let source_ml_source2_field483 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                {
                                                                    let source_ml_source2_field484 = self::ml_string::Text::from_units(vec![]);
                                                                    {
                                                                        let source_ml_source2_field485 = false;
                                                                        {
                                                                            let source_ml_source2_field486 = self::JavaScriptSemanticValue::Undefined;
                                                                            {
                                                                                let source_ml_source2_field488 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field483.clone(), source_ml_source2_field488, source_ml_source2_field486.clone(), source_ml_source2_field484.clone(), heap.clone(), receiver.clone(), source_ml_source2_field485)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Null => {
                                                            {
                                                                let source_ml_source2_field490 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                                {
                                                                    let source_ml_source2_field491 = self::ml_string::Text::from_units(vec![]);
                                                                    {
                                                                        let source_ml_source2_field492 = false;
                                                                        {
                                                                            let source_ml_source2_field495 = vec![value.clone()];
                                                                            (source_ml_source2_field490.clone(), source_ml_source2_field495, setter.clone(), source_ml_source2_field491.clone(), heap.clone(), receiver.clone(), source_ml_source2_field492)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                            {
                                                                let source_ml_source2_field497 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                                {
                                                                    let source_ml_source2_field498 = self::ml_string::Text::from_units(vec![]);
                                                                    {
                                                                        let source_ml_source2_field499 = false;
                                                                        {
                                                                            let source_ml_source2_field502 = vec![value.clone()];
                                                                            (source_ml_source2_field497.clone(), source_ml_source2_field502, setter.clone(), source_ml_source2_field498.clone(), heap.clone(), receiver.clone(), source_ml_source2_field499)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Number(_) => {
                                                            {
                                                                let source_ml_source2_field504 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                                {
                                                                    let source_ml_source2_field505 = self::ml_string::Text::from_units(vec![]);
                                                                    {
                                                                        let source_ml_source2_field506 = false;
                                                                        {
                                                                            let source_ml_source2_field509 = vec![value.clone()];
                                                                            (source_ml_source2_field504.clone(), source_ml_source2_field509, setter.clone(), source_ml_source2_field505.clone(), heap.clone(), receiver.clone(), source_ml_source2_field506)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                            {
                                                                let source_ml_source2_field511 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                                {
                                                                    let source_ml_source2_field512 = self::ml_string::Text::from_units(vec![]);
                                                                    {
                                                                        let source_ml_source2_field513 = false;
                                                                        {
                                                                            let source_ml_source2_field516 = vec![value.clone()];
                                                                            (source_ml_source2_field511.clone(), source_ml_source2_field516, setter.clone(), source_ml_source2_field512.clone(), heap.clone(), receiver.clone(), source_ml_source2_field513)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                            {
                                                                let source_ml_source2_field518 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                                {
                                                                    let source_ml_source2_field519 = self::ml_string::Text::from_units(vec![]);
                                                                    {
                                                                        let source_ml_source2_field520 = false;
                                                                        {
                                                                            let source_ml_source2_field523 = vec![value.clone()];
                                                                            (source_ml_source2_field518.clone(), source_ml_source2_field523, setter.clone(), source_ml_source2_field519.clone(), heap.clone(), receiver.clone(), source_ml_source2_field520)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                            {
                                                                let source_ml_source2_field525 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                                {
                                                                    let source_ml_source2_field526 = self::ml_string::Text::from_units(vec![]);
                                                                    {
                                                                        let source_ml_source2_field527 = false;
                                                                        {
                                                                            let source_ml_source2_field530 = vec![value.clone()];
                                                                            (source_ml_source2_field525.clone(), source_ml_source2_field530, setter.clone(), source_ml_source2_field526.clone(), heap.clone(), receiver.clone(), source_ml_source2_field527)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            {
                                                                let source_ml_source2_field532 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                                {
                                                                    let source_ml_source2_field533 = self::ml_string::Text::from_units(vec![]);
                                                                    {
                                                                        let source_ml_source2_field534 = false;
                                                                        {
                                                                            let source_ml_source2_field537 = vec![value.clone()];
                                                                            (source_ml_source2_field532.clone(), source_ml_source2_field537, setter.clone(), source_ml_source2_field533.clone(), heap.clone(), receiver.clone(), source_ml_source2_field534)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if !((own.clone()).10) {
                                                    {
                                                        let source_ml_source2_field539 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                        {
                                                            let source_ml_source2_field540 = self::ml_string::Text::from_units(vec![]);
                                                            {
                                                                let source_ml_source2_field541 = false;
                                                                {
                                                                    let source_ml_source2_field542 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source2_field544 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field539.clone(), source_ml_source2_field544, source_ml_source2_field542.clone(), source_ml_source2_field540.clone(), heap.clone(), receiver.clone(), source_ml_source2_field541)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    match receiver.clone() {
                                                        self::JavaScriptSemanticValue::Undefined => {
                                                            {
                                                                let source_ml_source2_field552 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                {
                                                                    let source_ml_source2_field553 = self::ml_string::Text::from_units(vec![]);
                                                                    {
                                                                        let source_ml_source2_field554 = false;
                                                                        {
                                                                            let source_ml_source2_field555 = self::JavaScriptSemanticValue::Undefined;
                                                                            {
                                                                                let source_ml_source2_field557 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field552.clone(), source_ml_source2_field557, source_ml_source2_field555.clone(), source_ml_source2_field553.clone(), heap.clone(), receiver.clone(), source_ml_source2_field554)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Null => {
                                                            {
                                                                let source_ml_source2_field559 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                {
                                                                    let source_ml_source2_field560 = self::ml_string::Text::from_units(vec![]);
                                                                    {
                                                                        let source_ml_source2_field561 = false;
                                                                        {
                                                                            let source_ml_source2_field562 = self::JavaScriptSemanticValue::Undefined;
                                                                            {
                                                                                let source_ml_source2_field564 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field559.clone(), source_ml_source2_field564, source_ml_source2_field562.clone(), source_ml_source2_field560.clone(), heap.clone(), receiver.clone(), source_ml_source2_field561)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                            {
                                                                let source_ml_source2_field566 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                {
                                                                    let source_ml_source2_field567 = self::ml_string::Text::from_units(vec![]);
                                                                    {
                                                                        let source_ml_source2_field568 = false;
                                                                        {
                                                                            let source_ml_source2_field569 = self::JavaScriptSemanticValue::Undefined;
                                                                            {
                                                                                let source_ml_source2_field571 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field566.clone(), source_ml_source2_field571, source_ml_source2_field569.clone(), source_ml_source2_field567.clone(), heap.clone(), receiver.clone(), source_ml_source2_field568)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Number(_) => {
                                                            {
                                                                let source_ml_source2_field573 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                {
                                                                    let source_ml_source2_field574 = self::ml_string::Text::from_units(vec![]);
                                                                    {
                                                                        let source_ml_source2_field575 = false;
                                                                        {
                                                                            let source_ml_source2_field576 = self::JavaScriptSemanticValue::Undefined;
                                                                            {
                                                                                let source_ml_source2_field578 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field573.clone(), source_ml_source2_field578, source_ml_source2_field576.clone(), source_ml_source2_field574.clone(), heap.clone(), receiver.clone(), source_ml_source2_field575)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                            {
                                                                let source_ml_source2_field580 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                {
                                                                    let source_ml_source2_field581 = self::ml_string::Text::from_units(vec![]);
                                                                    {
                                                                        let source_ml_source2_field582 = false;
                                                                        {
                                                                            let source_ml_source2_field583 = self::JavaScriptSemanticValue::Undefined;
                                                                            {
                                                                                let source_ml_source2_field585 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field580.clone(), source_ml_source2_field585, source_ml_source2_field583.clone(), source_ml_source2_field581.clone(), heap.clone(), receiver.clone(), source_ml_source2_field582)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                            {
                                                                let source_ml_source2_field587 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                {
                                                                    let source_ml_source2_field588 = self::ml_string::Text::from_units(vec![]);
                                                                    {
                                                                        let source_ml_source2_field589 = false;
                                                                        {
                                                                            let source_ml_source2_field590 = self::JavaScriptSemanticValue::Undefined;
                                                                            {
                                                                                let source_ml_source2_field592 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field587.clone(), source_ml_source2_field592, source_ml_source2_field590.clone(), source_ml_source2_field588.clone(), heap.clone(), receiver.clone(), source_ml_source2_field589)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                            {
                                                                let source_ml_source2_field594 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                {
                                                                    let source_ml_source2_field595 = self::ml_string::Text::from_units(vec![]);
                                                                    {
                                                                        let source_ml_source2_field596 = false;
                                                                        {
                                                                            let source_ml_source2_field597 = self::JavaScriptSemanticValue::Undefined;
                                                                            {
                                                                                let source_ml_source2_field599 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field594.clone(), source_ml_source2_field599, source_ml_source2_field597.clone(), source_ml_source2_field595.clone(), heap.clone(), receiver.clone(), source_ml_source2_field596)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                                            {
                                                                let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity_2.clone());
                                                                {
                                                                    let object = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(index));
                                                                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                        {
                                                                            let source_ml_source2_field603 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                                                            {
                                                                                let source_ml_source2_field604 = self::ml_string::Text::from_units(vec![83, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 79, 98, 106, 101, 99, 116]);
                                                                                {
                                                                                    let source_ml_source2_field605 = false;
                                                                                    {
                                                                                        let source_ml_source2_field606 = self::JavaScriptSemanticValue::Undefined;
                                                                                        {
                                                                                            let source_ml_source2_field608 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                            (source_ml_source2_field603.clone(), source_ml_source2_field608, source_ml_source2_field606.clone(), source_ml_source2_field604.clone(), heap.clone(), receiver.clone(), source_ml_source2_field605)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value609) => {
                                                                                    source_ml_source2_data_property_value609
                                                                                }
                                                                            }, key.clone());
                                                                            {
                                                                                let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value610) => {
                                                                                        source_ml_source2_data_property_value610
                                                                                    }
                                                                                }, self::ml_array::number_index(at));
                                                                                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (!(({
                                                                                    let source_ml_source2_nullish_field_receiver611 = (property.clone(),);
                                                                                    if match (source_ml_source2_nullish_field_receiver611.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                        {
                                                                                            let source_ml_source2_nullish_field_present612 = ((source_ml_source2_nullish_field_receiver611.clone()).0).clone().expect("a proven present value");
                                                                                            match source_ml_source2_nullish_field_present612.clone() {
                                                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value613) => {
                                                                                                    source_ml_source2_data_property_value613.clone()
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                                    }
                                                                                }).9) || !(({
                                                                                    let source_ml_source2_nullish_field_receiver614 = (property.clone(),);
                                                                                    if match (source_ml_source2_nullish_field_receiver614.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                        {
                                                                                            let source_ml_source2_nullish_field_present615 = ((source_ml_source2_nullish_field_receiver614.clone()).0).clone().expect("a proven present value");
                                                                                            match source_ml_source2_nullish_field_present615.clone() {
                                                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value616) => {
                                                                                                    source_ml_source2_data_property_value616.clone()
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                                    }
                                                                                }).10))) {
                                                                                    {
                                                                                        let source_ml_source2_field618 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                                        {
                                                                                            let source_ml_source2_field619 = self::ml_string::Text::from_units(vec![]);
                                                                                            {
                                                                                                let source_ml_source2_field620 = false;
                                                                                                {
                                                                                                    let source_ml_source2_field621 = self::JavaScriptSemanticValue::Undefined;
                                                                                                    {
                                                                                                        let source_ml_source2_field623 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                        (source_ml_source2_field618.clone(), source_ml_source2_field623, source_ml_source2_field621.clone(), source_ml_source2_field619.clone(), heap.clone(), receiver.clone(), source_ml_source2_field620)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    {
                                                                                        let proposed = self::create_java_script_ordinary_set_descriptor(value.clone(), match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false });
                                                                                        {
                                                                                            let transition = self::define_java_script_ordinary_property(heap.clone(), identity_2.clone(), key.clone(), proposed.clone());
                                                                                            {
                                                                                                let source_ml_source2_field624 = (transition.clone()).1;
                                                                                                {
                                                                                                    let source_ml_source2_field625 = if (((transition.clone()).0) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                        self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101])
                                                                                                    } else {
                                                                                                        self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_field626 = (transition.clone()).0;
                                                                                                        {
                                                                                                            let source_ml_source2_field627 = (transition.clone()).2;
                                                                                                            {
                                                                                                                let source_ml_source2_field628 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                {
                                                                                                                    let source_ml_source2_field630 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                                    (source_ml_source2_field625.clone(), source_ml_source2_field630, source_ml_source2_field628.clone(), source_ml_source2_field626.clone(), source_ml_source2_field624.clone(), receiver.clone(), source_ml_source2_field627)
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
pub fn read_java_script_primitive_conversion_spelling(name: &self::ml_string::Text) -> self::JavaScriptSemanticValue {
    if ((*name) == (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) {
        self::JavaScriptSemanticValue::Text(vec![100f64, 101f64, 102f64, 97f64, 117f64, 108f64, 116f64])
    } else {
        if ((*name) == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
            self::JavaScriptSemanticValue::Text(vec![110f64, 117f64, 109f64, 98f64, 101f64, 114f64])
        } else {
            if ((*name) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                self::JavaScriptSemanticValue::Text(vec![115f64, 116f64, 114f64, 105f64, 110f64, 103f64])
            } else {
                if ((*name) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102]))) {
                    self::JavaScriptSemanticValue::Text(vec![118f64, 97f64, 108f64, 117f64, 101f64, 79f64, 102f64])
                } else {
                    if ((*name) == (self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103]))) {
                        self::JavaScriptSemanticValue::Text(vec![116f64, 111f64, 83f64, 116f64, 114f64, 105f64, 110f64, 103f64])
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 115, 112, 101, 108, 108, 105, 110, 103, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 110, 97, 109, 101])).0, message_present: true }).raise()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_conversion_state_error(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    if {
                        let source_ml_source2_s717 = shared_constructor_field1.clone();
                        match source_ml_source2_s717.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                false
                            }
                            _ => {
                                true
                            }
                        }
                    } {
                        self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])
                    } else {
                        {
                            let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field1.clone());
                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                receiver_error.clone()
                            } else {
                                if {
                                    let source_ml_source2_s728 = shared_constructor_field2.clone();
                                    match source_ml_source2_s728.clone() {
                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }
                                } {
                                    self::ml_string::Text::from_units(vec![116, 104, 101, 32, 101, 120, 111, 116, 105, 99, 32, 109, 101, 116, 104, 111, 100, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108])
                                } else {
                                    {
                                        let key_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field2.clone());
                                        if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                            key_error.clone()
                                        } else {
                                            if (((shared_constructor_field3 != (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) && (shared_constructor_field3 != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) && (shared_constructor_field3 != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) {
                                                self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 101, 120, 97, 99, 116, 32, 112, 114, 101, 102, 101, 114, 114, 101, 100, 45, 116, 121, 112, 101, 32, 104, 105, 110, 116])
                                            } else {
                                                if !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108])], &shared_constructor_field4.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 109, 101, 116, 104, 111, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![])
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
pub fn continue_java_script_primitive_conversion(state: self::JavaScriptPrimitiveConversionState, phase: &self::ml_string::Text) -> self::JavaScriptPrimitiveConversionTransition {
    match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, _) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            self::JavaScriptPrimitiveConversionTransition::PrimitiveContinuation(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(shared_constructor_field1.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone(), phase.clone())))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_primitive_conversion_method(state: self::JavaScriptPrimitiveConversionState, method: self::JavaScriptSemanticValue) -> self::JavaScriptPrimitiveConversionTransition {
    match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let exotic = ((shared_constructor_field4 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]))) || (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]))));
                {
                    let first = ((shared_constructor_field4 == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116]))) || (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114]))));
                    if (exotic && (match method.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        _ => {
                            false
                        }
                    } || match method.clone() {
                        self::JavaScriptSemanticValue::Null => {
                            true
                        }
                        _ => {
                            false
                        }
                    })) {
                        self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116])))
                    } else {
                        match method.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                if exotic {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                } else {
                                    if first {
                                        self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                    } else {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                    }
                                }
                            }
                            self::JavaScriptSemanticValue::Null => {
                                if exotic {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                } else {
                                    if first {
                                        self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                    } else {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                    }
                                }
                            }
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                if exotic {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                } else {
                                    if first {
                                        self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                    } else {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                    }
                                }
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                if exotic {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                } else {
                                    if first {
                                        self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                    } else {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                    }
                                }
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                if exotic {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                } else {
                                    if first {
                                        self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                    } else {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                    }
                                }
                            }
                            self::JavaScriptSemanticValue::Text(_) => {
                                if exotic {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                } else {
                                    if first {
                                        self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                    } else {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                    }
                                }
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                if exotic {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                } else {
                                    if first {
                                        self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                    } else {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                    }
                                }
                            }
                            self::JavaScriptSemanticValue::Object(_, callable) => {
                                if !callable {
                                    if exotic {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                    } else {
                                        if first {
                                            self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                        } else {
                                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                        }
                                    }
                                } else {
                                    {
                                        let phase = if exotic {
                                            self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108])
                                        } else {
                                            if first {
                                                self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108])
                                            } else {
                                                self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108])
                                            }
                                        };
                                        {
                                            let arguments_ = if exotic {
                                                vec![self::read_java_script_primitive_conversion_spelling(&(shared_constructor_field3))]
                                            } else {
                                                Vec::<self::JavaScriptSemanticValue>::new()
                                            };
                                            self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(shared_constructor_field1.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone(), phase.clone())), Box::new(method.clone()), Box::new(shared_constructor_field1.clone()), arguments_)
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
pub fn step_java_script_primitive_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState) -> self::JavaScriptPrimitiveConversionTransition {
    match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let error = self::read_java_script_primitive_conversion_state_error(heap.clone(), state.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
                } else {
                    if (((shared_constructor_field4 != (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]))) && (shared_constructor_field4 != (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116])))) && (shared_constructor_field4 != (self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))) {
                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                    } else {
                        {
                            let receiver = shared_constructor_field1.clone();
                            match receiver.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Number(_) => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Object(identity, _) => {
                                    {
                                        let first = (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116])));
                                        {
                                            let method = if (shared_constructor_field3 == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                                                if first {
                                                    self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102])
                                                }
                                            } else {
                                                if first {
                                                    self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103])
                                                }
                                            };
                                            {
                                                let key = if (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]))) {
                                                    shared_constructor_field2.clone()
                                                } else {
                                                    self::read_java_script_primitive_conversion_spelling(&(method))
                                                };
                                                {
                                                    let plan = self::plan_java_script_ordinary_get(heap.clone(), identity.clone(), key.clone(), receiver.clone());
                                                    if (((plan.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid((plan.clone()).3)
                                                    } else {
                                                        if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                            {
                                                                let phase = if (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]))) {
                                                                    self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114])
                                                                } else {
                                                                    if first {
                                                                        self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114])
                                                                    }
                                                                };
                                                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(receiver.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone(), phase.clone())), Box::new((plan.clone()).2), Box::new((plan.clone()).4), (plan.clone()).1)
                                                            }
                                                        } else {
                                                            if (((plan.clone()).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 112, 101, 110, 100, 105, 110, 103, 32, 103, 101, 116, 116, 101, 114]))
                                                            } else {
                                                                self::apply_java_script_primitive_conversion_method(state.clone(), (plan.clone()).5)
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
pub fn begin_java_script_primitive_conversion(heap: self::JavaScriptOrdinaryHeap, input: self::JavaScriptSemanticValue, exotic_key: self::JavaScriptSemanticValue, hint: &self::ml_string::Text) -> self::JavaScriptPrimitiveConversionTransition {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(heap_error.clone())
        } else {
            {
                let error = self::read_java_script_ordinary_value_error(heap.clone(), input.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
                } else {
                    if ((((*hint) != (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) && ((*hint) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) && ((*hint) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) {
                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 101, 120, 97, 99, 116, 32, 112, 114, 101, 102, 101, 114, 114, 101, 100, 45, 116, 121, 112, 101, 32, 104, 105, 110, 116]))
                    } else {
                        match input.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Text(_) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                self::step_java_script_primitive_conversion(heap.clone(), self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(input.clone()), Box::new(exotic_key.clone()), hint.clone(), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116])))
                            }
                        }
                    }
                }
            }
        }
    }
}
