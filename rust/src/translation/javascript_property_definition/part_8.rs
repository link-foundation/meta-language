// Generated from complete js/src/translation/javascript-property-definition.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::create_java_script_property_descriptor;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::plan_java_script_property_definition;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_temporary_define_java_script_ordinary_property_loop20;

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
                                                let at = self::find_java_script_ordinary_property({
                                                    let source_ml_source1_nullish_field_receiver335 = (object.clone(),);
                                                    if match (source_ml_source1_nullish_field_receiver335.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                        {
                                                            let source_ml_source1_nullish_field_present336 = ((source_ml_source1_nullish_field_receiver335.clone()).0).clone().expect("a proven present value");
                                                            match source_ml_source1_nullish_field_present336.clone() {
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value337) => {
                                                                    source_ml_source1_data_property_value337
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                                    }
                                                }, key.clone());
                                                {
                                                    let property = self::ml_array::optional_at(&{
                                                        let source_ml_source1_nullish_field_receiver338 = (object.clone(),);
                                                        if match (source_ml_source1_nullish_field_receiver338.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                            {
                                                                let source_ml_source1_nullish_field_present339 = ((source_ml_source1_nullish_field_receiver338.clone()).0).clone().expect("a proven present value");
                                                                match source_ml_source1_nullish_field_present339.clone() {
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value340) => {
                                                                        source_ml_source1_data_property_value340
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                                        }
                                                    }, self::ml_array::number_index(at));
                                                    {
                                                        let current = if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false)
                                                        } else {
                                                            {
                                                                let source_ml_source1_nullish_field_receiver341 = (property.clone(),);
                                                                if match (source_ml_source1_nullish_field_receiver341.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                    {
                                                                        let source_ml_source1_nullish_field_present342 = ((source_ml_source1_nullish_field_receiver341.clone()).0).clone().expect("a proven present value");
                                                                        match source_ml_source1_nullish_field_present342.clone() {
                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value343) => {
                                                                                source_ml_source1_data_property_value343.clone()
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let transition = self::plan_java_script_property_definition(!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }), {
                                                                let source_ml_source1_nullish_field_receiver344 = (object.clone(),);
                                                                if match (source_ml_source1_nullish_field_receiver344.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                    {
                                                                        let source_ml_source1_nullish_field_present345 = ((source_ml_source1_nullish_field_receiver344.clone()).0).clone().expect("a proven present value");
                                                                        match source_ml_source1_nullish_field_present345.clone() {
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value346, _, _) => {
                                                                                source_ml_source1_data_property_value346
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 101, 120, 116, 101, 110, 115, 105, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                                                }
                                                            }, current.clone(), proposed.clone());
                                                            if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                                                                {
                                                                    let source_ml_source1_field347 = heap.clone();
                                                                    {
                                                                        let source_ml_source1_field348 = false;
                                                                        {
                                                                            let source_ml_source1_field349 = (transition.clone()).2;
                                                                            {
                                                                                let source_ml_source1_field350 = Vec::<self::ml_string::Text>::new();
                                                                                (source_ml_source1_field349.clone(), source_ml_source1_field347.clone(), source_ml_source1_field348, source_ml_source1_field350)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if (((transition.clone()).1) == (self::ml_string::Text::from_units(vec![107, 101, 101, 112]))) {
                                                                    {
                                                                        let source_ml_source1_field351 = heap.clone();
                                                                        {
                                                                            let source_ml_source1_field352 = true;
                                                                            {
                                                                                let source_ml_source1_field353 = self::ml_string::Text::from_units(vec![]);
                                                                                {
                                                                                    let source_ml_source1_field354 = Vec::<self::ml_string::Text>::new();
                                                                                    (source_ml_source1_field353.clone(), source_ml_source1_field351.clone(), source_ml_source1_field352, source_ml_source1_field354)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    if ((at < (0f64)) && (((({
                                                                        let source_ml_source1_nullish_field_receiver358 = (object.clone(),);
                                                                        if match (source_ml_source1_nullish_field_receiver358.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            {
                                                                                let source_ml_source1_nullish_field_present359 = ((source_ml_source1_nullish_field_receiver358.clone()).0).clone().expect("a proven present value");
                                                                                match source_ml_source1_nullish_field_present359.clone() {
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value360) => {
                                                                                        source_ml_source1_data_property_value360
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                                                        }
                                                                    }).len() as f64)) >= (4096f64))) {
                                                                        {
                                                                            let source_ml_source1_field361 = heap.clone();
                                                                            {
                                                                                let source_ml_source1_field362 = false;
                                                                                {
                                                                                    let source_ml_source1_field363 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]);
                                                                                    {
                                                                                        let source_ml_source1_field364 = Vec::<self::ml_string::Text>::new();
                                                                                        (source_ml_source1_field363.clone(), source_ml_source1_field361.clone(), source_ml_source1_field362, source_ml_source1_field364)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let count = 0f64;
                                                                            {
                                                                                let source_ml_temporary_values19 = match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value365, _) => {
                                                                                        source_ml_source1_data_property_value365
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_temporary_index19 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let count_2 = self::source_ml_temporary_define_java_script_ordinary_property_loop20(count, source_ml_temporary_values19, source_ml_temporary_index19.clone());
                                                                                        if ((at < (0f64)) && (count_2 >= (16384f64))) {
                                                                                            {
                                                                                                let source_ml_source1_field366 = heap.clone();
                                                                                                {
                                                                                                    let source_ml_source1_field367 = false;
                                                                                                    {
                                                                                                        let source_ml_source1_field368 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]);
                                                                                                        {
                                                                                                            let source_ml_source1_field369 = Vec::<self::ml_string::Text>::new();
                                                                                                            (source_ml_source1_field368.clone(), source_ml_source1_field366.clone(), source_ml_source1_field367, source_ml_source1_field369)
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
                                                                                                            let source_ml_source1_concat_value373 = {
                                                                                                                let source_ml_source1_nullish_field_receiver370 = (object.clone(),);
                                                                                                                if match (source_ml_source1_nullish_field_receiver370.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                    {
                                                                                                                        let source_ml_source1_nullish_field_present371 = ((source_ml_source1_nullish_field_receiver370.clone()).0).clone().expect("a proven present value");
                                                                                                                        match source_ml_source1_nullish_field_present371.clone() {
                                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value372) => {
                                                                                                                                source_ml_source1_data_property_value372
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_concat_value374 = vec![updated.clone()];
                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryProperty>::new(), source_ml_source1_concat_value373), source_ml_source1_concat_value374)
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        { let ml_values2 = {
                                                                                                            let source_ml_source1_nullish_field_receiver375 = (object.clone(),);
                                                                                                            if match (source_ml_source1_nullish_field_receiver375.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                {
                                                                                                                    let source_ml_source1_nullish_field_present376 = ((source_ml_source1_nullish_field_receiver375.clone()).0).clone().expect("a proven present value");
                                                                                                                    match source_ml_source1_nullish_field_present376.clone() {
                                                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value377) => {
                                                                                                                            source_ml_source1_data_property_value377
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                                                                                            }
                                                                                                        }; let ml_callback2 = { let at = at; let updated = updated.clone(); std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> self::JavaScriptOrdinaryProperty { if (position == at) {
                                                                                                            updated.clone()
                                                                                                        } else {
                                                                                                            entry.clone()
                                                                                                        } }) }; ml_values2.iter().cloned().enumerate().map(|(ml_index2, ml_value2)| ml_callback2(ml_value2, (ml_index2 as f64))).collect::<Vec<_>>() }
                                                                                                    };
                                                                                                    {
                                                                                                        let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject({
                                                                                                            let source_ml_source1_nullish_field_receiver378 = (object.clone(),);
                                                                                                            if match (source_ml_source1_nullish_field_receiver378.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                {
                                                                                                                    let source_ml_source1_nullish_field_present379 = ((source_ml_source1_nullish_field_receiver378.clone()).0).clone().expect("a proven present value");
                                                                                                                    match source_ml_source1_nullish_field_present379.clone() {
                                                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value380, _, _, _, _) => {
                                                                                                                            source_ml_source1_data_property_value380.clone()
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                                                                                                            }
                                                                                                        }, Box::new({
                                                                                                            let source_ml_source1_nullish_field_receiver381 = (object.clone(),);
                                                                                                            if match (source_ml_source1_nullish_field_receiver381.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                {
                                                                                                                    let source_ml_source1_nullish_field_present382 = ((source_ml_source1_nullish_field_receiver381.clone()).0).clone().expect("a proven present value");
                                                                                                                    match source_ml_source1_nullish_field_present382.clone() {
                                                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value383, _, _, _) => {
                                                                                                                            let source_ml_source1_data_property_value383 = *source_ml_source1_data_property_value383;
                                                                                                                            source_ml_source1_data_property_value383.clone()
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                                                                                                            }
                                                                                                        }), {
                                                                                                            let source_ml_source1_nullish_field_receiver384 = (object.clone(),);
                                                                                                            if match (source_ml_source1_nullish_field_receiver384.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                {
                                                                                                                    let source_ml_source1_nullish_field_present385 = ((source_ml_source1_nullish_field_receiver384.clone()).0).clone().expect("a proven present value");
                                                                                                                    match source_ml_source1_nullish_field_present385.clone() {
                                                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value386, _, _) => {
                                                                                                                            source_ml_source1_data_property_value386
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 101, 120, 116, 101, 110, 115, 105, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                                                                                            }
                                                                                                        }, {
                                                                                                            let source_ml_source1_nullish_field_receiver387 = (object.clone(),);
                                                                                                            if match (source_ml_source1_nullish_field_receiver387.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                {
                                                                                                                    let source_ml_source1_nullish_field_present388 = ((source_ml_source1_nullish_field_receiver387.clone()).0).clone().expect("a proven present value");
                                                                                                                    match source_ml_source1_nullish_field_present388.clone() {
                                                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value389, _) => {
                                                                                                                            source_ml_source1_data_property_value389
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                                                                                            }
                                                                                                        }, properties);
                                                                                                        {
                                                                                                            let source_ml_source1_field390 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                                                            {
                                                                                                                let source_ml_source1_field391 = true;
                                                                                                                {
                                                                                                                    let source_ml_source1_field392 = self::ml_string::Text::from_units(vec![]);
                                                                                                                    {
                                                                                                                        let source_ml_source1_field393 = (transition.clone()).3;
                                                                                                                        (source_ml_source1_field392.clone(), source_ml_source1_field390.clone(), source_ml_source1_field391, source_ml_source1_field393)
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
pub fn delete_java_script_ordinary_property(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool) {
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
                        let source_ml_source1_field394 = heap.clone();
                        {
                            let source_ml_source1_field395 = false;
                            {
                                let source_ml_source1_field396 = error.clone();
                                (source_ml_source1_field396.clone(), source_ml_source1_field394.clone(), source_ml_source1_field395)
                            }
                        }
                    }
                } else {
                    {
                        let index = self::find_java_script_ordinary_object(match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value397, _) => {
                                source_ml_source1_data_property_value397
                            }
                        }, identity.clone());
                        {
                            let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value398, _) => source_ml_source1_data_property_value398.as_slice() }), self::ml_array::number_index(index));
                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                {
                                    let source_ml_source1_field399 = heap.clone();
                                    {
                                        let source_ml_source1_field400 = false;
                                        {
                                            let source_ml_source1_field401 = self::ml_string::Text::from_units(vec![68, 101, 108, 101, 116, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                            (source_ml_source1_field401.clone(), source_ml_source1_field399.clone(), source_ml_source1_field400)
                                        }
                                    }
                                }
                            } else {
                                {
                                    let at = self::find_java_script_ordinary_property({
                                        let source_ml_source1_nullish_field_receiver402 = (object.clone(),);
                                        if match (source_ml_source1_nullish_field_receiver402.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                            {
                                                let source_ml_source1_nullish_field_present403 = ((source_ml_source1_nullish_field_receiver402.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source1_nullish_field_present403.clone() {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value404) => {
                                                        source_ml_source1_data_property_value404
                                                    }
                                                }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                        }
                                    }, key.clone());
                                    {
                                        let property = self::ml_array::optional_at(&{
                                            let source_ml_source1_nullish_field_receiver405 = (object.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver405.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source1_nullish_field_present406 = ((source_ml_source1_nullish_field_receiver405.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present406.clone() {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value407) => {
                                                            source_ml_source1_data_property_value407
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                            }
                                        }, self::ml_array::number_index(at));
                                        if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            {
                                                let source_ml_source1_field408 = heap.clone();
                                                {
                                                    let source_ml_source1_field409 = true;
                                                    {
                                                        let source_ml_source1_field410 = self::ml_string::Text::from_units(vec![]);
                                                        (source_ml_source1_field410.clone(), source_ml_source1_field408.clone(), source_ml_source1_field409)
                                                    }
                                                }
                                            }
                                        } else {
                                            if !(({
                                                let source_ml_source1_nullish_field_receiver411 = (property.clone(),);
                                                if match (source_ml_source1_nullish_field_receiver411.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source1_nullish_field_present412 = ((source_ml_source1_nullish_field_receiver411.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source1_nullish_field_present412.clone() {
                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value413) => {
                                                                source_ml_source1_data_property_value413.clone()
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                }
                                            }).0) {
                                                {
                                                    let source_ml_source1_field414 = heap.clone();
                                                    {
                                                        let source_ml_source1_field415 = false;
                                                        {
                                                            let source_ml_source1_field416 = self::ml_string::Text::from_units(vec![]);
                                                            (source_ml_source1_field416.clone(), source_ml_source1_field414.clone(), source_ml_source1_field415)
                                                        }
                                                    }
                                                }
                                            } else {
                                                {
                                                    let properties = { let ml_values3 = {
                                                        let source_ml_source1_nullish_field_receiver417 = (object.clone(),);
                                                        if match (source_ml_source1_nullish_field_receiver417.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                            {
                                                                let source_ml_source1_nullish_field_present418 = ((source_ml_source1_nullish_field_receiver417.clone()).0).clone().expect("a proven present value");
                                                                match source_ml_source1_nullish_field_present418.clone() {
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value419) => {
                                                                        source_ml_source1_data_property_value419
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                                        }
                                                    }; let ml_callback3 = { let at = at; std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> bool { (position != at) }) }; ml_values3.iter().cloned().enumerate().filter_map(|(ml_index3, ml_value3)| if ml_callback3(ml_value3.clone(), (ml_index3 as f64)) { Some(ml_value3) } else { None }).collect::<Vec<_>>() };
                                                    {
                                                        let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject({
                                                            let source_ml_source1_nullish_field_receiver420 = (object.clone(),);
                                                            if match (source_ml_source1_nullish_field_receiver420.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                {
                                                                    let source_ml_source1_nullish_field_present421 = ((source_ml_source1_nullish_field_receiver420.clone()).0).clone().expect("a proven present value");
                                                                    match source_ml_source1_nullish_field_present421.clone() {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value422, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value422.clone()
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                                                            }
                                                        }, Box::new({
                                                            let source_ml_source1_nullish_field_receiver423 = (object.clone(),);
                                                            if match (source_ml_source1_nullish_field_receiver423.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                {
                                                                    let source_ml_source1_nullish_field_present424 = ((source_ml_source1_nullish_field_receiver423.clone()).0).clone().expect("a proven present value");
                                                                    match source_ml_source1_nullish_field_present424.clone() {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value425, _, _, _) => {
                                                                            let source_ml_source1_data_property_value425 = *source_ml_source1_data_property_value425;
                                                                            source_ml_source1_data_property_value425.clone()
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                                                            }
                                                        }), {
                                                            let source_ml_source1_nullish_field_receiver426 = (object.clone(),);
                                                            if match (source_ml_source1_nullish_field_receiver426.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                {
                                                                    let source_ml_source1_nullish_field_present427 = ((source_ml_source1_nullish_field_receiver426.clone()).0).clone().expect("a proven present value");
                                                                    match source_ml_source1_nullish_field_present427.clone() {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value428, _, _) => {
                                                                            source_ml_source1_data_property_value428
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 101, 120, 116, 101, 110, 115, 105, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                                            }
                                                        }, {
                                                            let source_ml_source1_nullish_field_receiver429 = (object.clone(),);
                                                            if match (source_ml_source1_nullish_field_receiver429.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                {
                                                                    let source_ml_source1_nullish_field_present430 = ((source_ml_source1_nullish_field_receiver429.clone()).0).clone().expect("a proven present value");
                                                                    match source_ml_source1_nullish_field_present430.clone() {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value431, _) => {
                                                                            source_ml_source1_data_property_value431
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                                            }
                                                        }, properties);
                                                        {
                                                            let source_ml_source1_field432 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                            {
                                                                let source_ml_source1_field433 = true;
                                                                {
                                                                    let source_ml_source1_field434 = self::ml_string::Text::from_units(vec![]);
                                                                    (source_ml_source1_field434.clone(), source_ml_source1_field432.clone(), source_ml_source1_field433)
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
pub fn prevent_java_script_ordinary_extensions(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool) {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            {
                let source_ml_source1_field435 = heap.clone();
                {
                    let source_ml_source1_field436 = false;
                    {
                        let source_ml_source1_field437 = error.clone();
                        (source_ml_source1_field437.clone(), source_ml_source1_field435.clone(), source_ml_source1_field436)
                    }
                }
            }
        } else {
            {
                let index = self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value438, _) => {
                        source_ml_source1_data_property_value438
                    }
                }, identity.clone());
                {
                    let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value439, _) => source_ml_source1_data_property_value439.as_slice() }), self::ml_array::number_index(index));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        {
                            let source_ml_source1_field440 = heap.clone();
                            {
                                let source_ml_source1_field441 = false;
                                {
                                    let source_ml_source1_field442 = self::ml_string::Text::from_units(vec![80, 114, 101, 118, 101, 110, 116, 69, 120, 116, 101, 110, 115, 105, 111, 110, 115, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                    (source_ml_source1_field442.clone(), source_ml_source1_field440.clone(), source_ml_source1_field441)
                                }
                            }
                        }
                    } else {
                        {
                            let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject({
                                let source_ml_source1_nullish_field_receiver443 = (object.clone(),);
                                if match (source_ml_source1_nullish_field_receiver443.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source1_nullish_field_present444 = ((source_ml_source1_nullish_field_receiver443.clone()).0).clone().expect("a proven present value");
                                        match source_ml_source1_nullish_field_present444.clone() {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value445, _, _, _, _) => {
                                                source_ml_source1_data_property_value445.clone()
                                            }
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                                }
                            }, Box::new({
                                let source_ml_source1_nullish_field_receiver446 = (object.clone(),);
                                if match (source_ml_source1_nullish_field_receiver446.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source1_nullish_field_present447 = ((source_ml_source1_nullish_field_receiver446.clone()).0).clone().expect("a proven present value");
                                        match source_ml_source1_nullish_field_present447.clone() {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value448, _, _, _) => {
                                                let source_ml_source1_data_property_value448 = *source_ml_source1_data_property_value448;
                                                source_ml_source1_data_property_value448.clone()
                                            }
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                                }
                            }), false, {
                                let source_ml_source1_nullish_field_receiver449 = (object.clone(),);
                                if match (source_ml_source1_nullish_field_receiver449.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source1_nullish_field_present450 = ((source_ml_source1_nullish_field_receiver449.clone()).0).clone().expect("a proven present value");
                                        match source_ml_source1_nullish_field_present450.clone() {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value451, _) => {
                                                source_ml_source1_data_property_value451
                                            }
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                }
                            }, {
                                let source_ml_source1_nullish_field_receiver452 = (object.clone(),);
                                if match (source_ml_source1_nullish_field_receiver452.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source1_nullish_field_present453 = ((source_ml_source1_nullish_field_receiver452.clone()).0).clone().expect("a proven present value");
                                        match source_ml_source1_nullish_field_present453.clone() {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value454) => {
                                                source_ml_source1_data_property_value454
                                            }
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                }
                            });
                            {
                                let source_ml_source1_field455 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                {
                                    let source_ml_source1_field456 = true;
                                    {
                                        let source_ml_source1_field457 = self::ml_string::Text::from_units(vec![]);
                                        (source_ml_source1_field457.clone(), source_ml_source1_field455.clone(), source_ml_source1_field456)
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
