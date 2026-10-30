// Generated from complete js/src/translation/javascript-primitive-conversion.js. Do not edit by hand.
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
use super::plan_java_script_property_definition;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_temporary_define_java_script_ordinary_property_loop20;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn define_java_script_ordinary_property(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
    proposed: (
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
) -> (String, self::JavaScriptOrdinaryHeap, bool, Vec<String>) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
            {
                let error = if !(heap_error).is_empty() {
                    heap_error.clone()
                } else {
                    key_error
                };
                {
                    let ml_library_inequality_1 = error != "";
                    if ml_library_inequality_1 {
                        {
                            let source_ml_source1_field313 = heap.clone();
                            {
                                let source_ml_source1_field314 = false;
                                {
                                    let source_ml_source1_field315 = error.clone();
                                    {
                                        let source_ml_source1_field316 = Vec::<String>::new();
                                        (
                                            source_ml_source1_field315,
                                            source_ml_source1_field313,
                                            source_ml_source1_field314,
                                            source_ml_source1_field316,
                                        )
                                    }
                                }
                            }
                        }
                    } else if (proposed.clone()).9
                        && (!(self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            (proposed.clone()).8,
                        ))
                        .is_empty())
                    {
                        {
                            let source_ml_source1_field317 = heap.clone();
                            {
                                let source_ml_source1_field318 = false;
                                {
                                    let source_ml_source1_field319 = String::from(
                                        "proposed property values must belong to the actual global context",
                                    );
                                    {
                                        let source_ml_source1_field320 = Vec::<String>::new();
                                        (
                                            source_ml_source1_field319,
                                            source_ml_source1_field317,
                                            source_ml_source1_field318,
                                            source_ml_source1_field320,
                                        )
                                    }
                                }
                            }
                        }
                    } else if (proposed.clone()).5
                        && (!(self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            (proposed.clone()).4,
                        ))
                        .is_empty())
                    {
                        {
                            let source_ml_source1_field321 = heap.clone();
                            {
                                let source_ml_source1_field322 = false;
                                {
                                    let source_ml_source1_field323 = String::from(
                                        "proposed getters must belong to the actual global context",
                                    );
                                    {
                                        let source_ml_source1_field324 = Vec::<String>::new();
                                        (
                                            source_ml_source1_field323,
                                            source_ml_source1_field321,
                                            source_ml_source1_field322,
                                            source_ml_source1_field324,
                                        )
                                    }
                                }
                            }
                        }
                    } else if (proposed.clone()).7
                        && (!(self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            (proposed.clone()).6,
                        ))
                        .is_empty())
                    {
                        {
                            let source_ml_source1_field325 = heap.clone();
                            {
                                let source_ml_source1_field326 = false;
                                {
                                    let source_ml_source1_field327 = String::from(
                                        "proposed setters must belong to the actual global context",
                                    );
                                    {
                                        let source_ml_source1_field328 = Vec::<String>::new();
                                        (
                                            source_ml_source1_field327,
                                            source_ml_source1_field325,
                                            source_ml_source1_field326,
                                            source_ml_source1_field328,
                                        )
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(
                                match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                        source_ml_source1_data_property_value329,
                                        _,
                                    ) => source_ml_source1_data_property_value329,
                                },
                                identity.clone(),
                            );
                            {
                                let object = self::ml_array::optional_at(
                                    (match &heap {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                            source_ml_source1_data_property_value330,
                                            _,
                                        ) => source_ml_source1_data_property_value330.as_slice(),
                                    }),
                                    self::ml_array::number_index(index),
                                );
                                if match (object.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    {
                                        let source_ml_source1_field331 = heap.clone();
                                        {
                                            let source_ml_source1_field332 = false;
                                            {
                                                let source_ml_source1_field333 = String::from(
                                                    "DefineOwnProperty needs its actual Object identity",
                                                );
                                                {
                                                    let source_ml_source1_field334 =
                                                        Vec::<String>::new();
                                                    (
                                                        source_ml_source1_field333,
                                                        source_ml_source1_field331,
                                                        source_ml_source1_field332,
                                                        source_ml_source1_field334,
                                                    )
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let at = self::find_java_script_ordinary_property(
                                            {
                                                let source_ml_source1_nullish_field_receiver335 =
                                                    (object.clone(),);
                                                if match (source_ml_source1_nullish_field_receiver335.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                        {
                                                            let source_ml_source1_nullish_field_present336 = ((source_ml_source1_nullish_field_receiver335.clone()).0).clone().expect("a proven present value");
                                                            match source_ml_source1_nullish_field_present336 {
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value337) => {
                                                                    source_ml_source1_data_property_value337
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'properties')")).encode_utf16().collect(), message_present: true }).raise()
                                                    }
                                            },
                                            key.clone(),
                                        );
                                        {
                                            let property = self::ml_array::optional_at(
                                                &{
                                                    let source_ml_source1_nullish_field_receiver338 =
                                                        (object.clone(),);
                                                    if match (source_ml_source1_nullish_field_receiver338.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                            {
                                                                let source_ml_source1_nullish_field_present339 = ((source_ml_source1_nullish_field_receiver338.clone()).0).clone().expect("a proven present value");
                                                                match source_ml_source1_nullish_field_present339 {
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value340) => {
                                                                        source_ml_source1_data_property_value340
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'properties')")).encode_utf16().collect(), message_present: true }).raise()
                                                        }
                                                },
                                                self::ml_array::number_index(at),
                                            );
                                            {
                                                let current = if match (
                                                    property.clone(),
                                                    None::<()>,
                                                ) {
                                                    (Some(_ml_left), Some(_ml_right)) => false,
                                                    (None, None) => true,
                                                    _ => false,
                                                } {
                                                    self::create_java_script_property_descriptor(
                                                        "data", false, false,
                                                    )
                                                } else {
                                                    {
                                                        let source_ml_source1_nullish_field_receiver341 =
                                                            (property.clone(),);
                                                        if match (source_ml_source1_nullish_field_receiver341.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                    {
                                                                        let source_ml_source1_nullish_field_present342 = ((source_ml_source1_nullish_field_receiver341.clone()).0).clone().expect("a proven present value");
                                                                        match source_ml_source1_nullish_field_present342 {
                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value343) => {
                                                                                source_ml_source1_data_property_value343.clone()
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'descriptor')")).encode_utf16().collect(), message_present: true }).raise()
                                                                }
                                                    }
                                                };
                                                {
                                                    let transition =
                                                        self::plan_java_script_property_definition(
                                                            !(match (property.clone(), None::<()>) {
                                                                (
                                                                    Some(_ml_left),
                                                                    Some(_ml_right),
                                                                ) => false,
                                                                (None, None) => true,
                                                                _ => false,
                                                            }),
                                                            {
                                                                let source_ml_source1_nullish_field_receiver344 =
                                                                    (object.clone(),);
                                                                if match (source_ml_source1_nullish_field_receiver344.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                    {
                                                                        let source_ml_source1_nullish_field_present345 = ((source_ml_source1_nullish_field_receiver344.clone()).0).clone().expect("a proven present value");
                                                                        match source_ml_source1_nullish_field_present345 {
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value346, _, _) => {
                                                                                source_ml_source1_data_property_value346
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'extensible')")).encode_utf16().collect(), message_present: true }).raise()
                                                                }
                                                            },
                                                            current,
                                                            proposed.clone(),
                                                        );
                                                    if ((transition.clone()).1) == "reject" {
                                                                {
                                                                    let source_ml_source1_field347 = heap.clone();
                                                                    {
                                                                        let source_ml_source1_field348 = false;
                                                                        {
                                                                            let source_ml_source1_field349 = (transition.clone()).2;
                                                                            {
                                                                                let source_ml_source1_field350 = Vec::<String>::new();
                                                                                (source_ml_source1_field349, source_ml_source1_field347, source_ml_source1_field348, source_ml_source1_field350)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else if ((transition.clone()).1) == "keep" {
                                                                    {
                                                                        let source_ml_source1_field351 = heap.clone();
                                                                        {
                                                                            let source_ml_source1_field352 = true;
                                                                            {
                                                                                let source_ml_source1_field353 = String::new();
                                                                                {
                                                                                    let source_ml_source1_field354 = Vec::<String>::new();
                                                                                    (source_ml_source1_field353, source_ml_source1_field351, source_ml_source1_field352, source_ml_source1_field354)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else if (at < 0f64) && ((({
                                                                        let source_ml_source1_nullish_field_receiver358 = (object.clone(),);
                                                                        if match (source_ml_source1_nullish_field_receiver358.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            {
                                                                                let source_ml_source1_nullish_field_present359 = ((source_ml_source1_nullish_field_receiver358.clone()).0).clone().expect("a proven present value");
                                                                                match source_ml_source1_nullish_field_present359 {
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value360) => {
                                                                                        source_ml_source1_data_property_value360
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'properties')")).encode_utf16().collect(), message_present: true }).raise()
                                                                        }
                                                                    }).len() as f64) >= 4096f64) {
                                                                        {
                                                                            let source_ml_source1_field361 = heap.clone();
                                                                            {
                                                                                let source_ml_source1_field362 = false;
                                                                                {
                                                                                    let source_ml_source1_field363 = String::from("ordinary properties exceed their explicit ordered storage budget");
                                                                                    {
                                                                                        let source_ml_source1_field364 = Vec::<String>::new();
                                                                                        (source_ml_source1_field363, source_ml_source1_field361, source_ml_source1_field362, source_ml_source1_field364)
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
                                                                                        let count_2 = self::source_ml_temporary_define_java_script_ordinary_property_loop20(count, source_ml_temporary_values19, source_ml_temporary_index19);
                                                                                        if (at < 0f64) && (count_2 >= 16_384f64) {
                                                                                            {
                                                                                                let source_ml_source1_field366 = heap.clone();
                                                                                                {
                                                                                                    let source_ml_source1_field367 = false;
                                                                                                    {
                                                                                                        let source_ml_source1_field368 = String::from("ordinary properties exceed their explicit ordered storage budget");
                                                                                                        {
                                                                                                            let source_ml_source1_field369 = Vec::<String>::new();
                                                                                                            (source_ml_source1_field368, source_ml_source1_field366, source_ml_source1_field367, source_ml_source1_field369)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            {
                                                                                                let updated = self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(key.clone()), (transition.clone()).0);
                                                                                                {
                                                                                                    let properties = if at < 0f64 {
                                                                                                        {
                                                                                                            let source_ml_source1_concat_value373 = {
                                                                                                                let source_ml_source1_nullish_field_receiver370 = (object.clone(),);
                                                                                                                if match (source_ml_source1_nullish_field_receiver370.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                    {
                                                                                                                        let source_ml_source1_nullish_field_present371 = ((source_ml_source1_nullish_field_receiver370.clone()).0).clone().expect("a proven present value");
                                                                                                                        match source_ml_source1_nullish_field_present371 {
                                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value372) => {
                                                                                                                                source_ml_source1_data_property_value372
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'properties')")).encode_utf16().collect(), message_present: true }).raise()
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
                                                                                                                    match source_ml_source1_nullish_field_present376 {
                                                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value377) => {
                                                                                                                            source_ml_source1_data_property_value377
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'properties')")).encode_utf16().collect(), message_present: true }).raise()
                                                                                                            }
                                                                                                        }; let ml_callback2 = { let at = at; let updated = updated.clone(); std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> self::JavaScriptOrdinaryProperty { if position == at {
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
                                                                                                                    match source_ml_source1_nullish_field_present379 {
                                                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value380, _, _, _, _) => {
                                                                                                                            source_ml_source1_data_property_value380.clone()
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'identity')")).encode_utf16().collect(), message_present: true }).raise()
                                                                                                            }
                                                                                                        }, Box::new({
                                                                                                            let source_ml_source1_nullish_field_receiver381 = (object.clone(),);
                                                                                                            if match (source_ml_source1_nullish_field_receiver381.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                {
                                                                                                                    let source_ml_source1_nullish_field_present382 = ((source_ml_source1_nullish_field_receiver381.clone()).0).clone().expect("a proven present value");
                                                                                                                    match source_ml_source1_nullish_field_present382 {
                                                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value383, _, _, _) => {
                                                                                                                            let source_ml_source1_data_property_value383 = *source_ml_source1_data_property_value383;
                                                                                                                            source_ml_source1_data_property_value383.clone()
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'prototype')")).encode_utf16().collect(), message_present: true }).raise()
                                                                                                            }
                                                                                                        }), {
                                                                                                            let source_ml_source1_nullish_field_receiver384 = (object.clone(),);
                                                                                                            if match (source_ml_source1_nullish_field_receiver384.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                {
                                                                                                                    let source_ml_source1_nullish_field_present385 = ((source_ml_source1_nullish_field_receiver384.clone()).0).clone().expect("a proven present value");
                                                                                                                    match source_ml_source1_nullish_field_present385 {
                                                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value386, _, _) => {
                                                                                                                            source_ml_source1_data_property_value386
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'extensible')")).encode_utf16().collect(), message_present: true }).raise()
                                                                                                            }
                                                                                                        }, {
                                                                                                            let source_ml_source1_nullish_field_receiver387 = (object.clone(),);
                                                                                                            if match (source_ml_source1_nullish_field_receiver387.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                {
                                                                                                                    let source_ml_source1_nullish_field_present388 = ((source_ml_source1_nullish_field_receiver387.clone()).0).clone().expect("a proven present value");
                                                                                                                    match source_ml_source1_nullish_field_present388 {
                                                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value389, _) => {
                                                                                                                            source_ml_source1_data_property_value389
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'callable')")).encode_utf16().collect(), message_present: true }).raise()
                                                                                                            }
                                                                                                        }, properties);
                                                                                                        {
                                                                                                            let source_ml_source1_field390 = self::replace_java_script_ordinary_object(heap.clone(), replacement);
                                                                                                            {
                                                                                                                let source_ml_source1_field391 = true;
                                                                                                                {
                                                                                                                    let source_ml_source1_field392 = String::new();
                                                                                                                    {
                                                                                                                        let source_ml_source1_field393 = (transition.clone()).3;
                                                                                                                        (source_ml_source1_field392, source_ml_source1_field390, source_ml_source1_field391, source_ml_source1_field393)
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
pub fn delete_java_script_ordinary_property(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
) -> (String, self::JavaScriptOrdinaryHeap, bool) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
            {
                let error = if !(heap_error).is_empty() {
                    heap_error.clone()
                } else {
                    key_error
                };
                {
                    let ml_library_inequality_1 = error != "";
                    if ml_library_inequality_1 {
                        {
                            let source_ml_source1_field394 = heap.clone();
                            {
                                let source_ml_source1_field395 = false;
                                {
                                    let source_ml_source1_field396 = error.clone();
                                    (
                                        source_ml_source1_field396,
                                        source_ml_source1_field394,
                                        source_ml_source1_field395,
                                    )
                                }
                            }
                        }
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(
                                match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                        source_ml_source1_data_property_value397,
                                        _,
                                    ) => source_ml_source1_data_property_value397,
                                },
                                identity.clone(),
                            );
                            {
                                let object = self::ml_array::optional_at(
                                    (match &heap {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                            source_ml_source1_data_property_value398,
                                            _,
                                        ) => source_ml_source1_data_property_value398.as_slice(),
                                    }),
                                    self::ml_array::number_index(index),
                                );
                                if match (object.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    {
                                        let source_ml_source1_field399 = heap.clone();
                                        {
                                            let source_ml_source1_field400 = false;
                                            {
                                                let source_ml_source1_field401 = String::from(
                                                    "Delete needs its actual Object identity",
                                                );
                                                (
                                                    source_ml_source1_field401,
                                                    source_ml_source1_field399,
                                                    source_ml_source1_field400,
                                                )
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let at = self::find_java_script_ordinary_property(
                                            {
                                                let source_ml_source1_nullish_field_receiver402 =
                                                    (object.clone(),);
                                                if match (source_ml_source1_nullish_field_receiver402.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                            {
                                                let source_ml_source1_nullish_field_present403 = ((source_ml_source1_nullish_field_receiver402.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source1_nullish_field_present403 {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value404) => {
                                                        source_ml_source1_data_property_value404
                                                    }
                                                }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'properties')")).encode_utf16().collect(), message_present: true }).raise()
                                        }
                                            },
                                            key.clone(),
                                        );
                                        {
                                            let property = self::ml_array::optional_at(
                                                &{
                                                    let source_ml_source1_nullish_field_receiver405 =
                                                        (object.clone(),);
                                                    if match (source_ml_source1_nullish_field_receiver405.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source1_nullish_field_present406 = ((source_ml_source1_nullish_field_receiver405.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present406 {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value407) => {
                                                            source_ml_source1_data_property_value407
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'properties')")).encode_utf16().collect(), message_present: true }).raise()
                                            }
                                                },
                                                self::ml_array::number_index(at),
                                            );
                                            if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            {
                                                let source_ml_source1_field408 = heap.clone();
                                                {
                                                    let source_ml_source1_field409 = true;
                                                    {
                                                        let source_ml_source1_field410 = String::new();
                                                        (source_ml_source1_field410, source_ml_source1_field408, source_ml_source1_field409)
                                                    }
                                                }
                                            }
                                        } else if (({
                                                let source_ml_source1_nullish_field_receiver411 = (property.clone(),);
                                                if match (source_ml_source1_nullish_field_receiver411.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source1_nullish_field_present412 = ((source_ml_source1_nullish_field_receiver411.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source1_nullish_field_present412 {
                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value413) => {
                                                                source_ml_source1_data_property_value413.clone()
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'descriptor')")).encode_utf16().collect(), message_present: true }).raise()
                                                }
                                            }).0) {
                                                {
                                                    let properties = { let ml_values3 = {
                                                        let source_ml_source1_nullish_field_receiver417 = (object.clone(),);
                                                        if match (source_ml_source1_nullish_field_receiver417.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                            {
                                                                let source_ml_source1_nullish_field_present418 = ((source_ml_source1_nullish_field_receiver417.clone()).0).clone().expect("a proven present value");
                                                                match source_ml_source1_nullish_field_present418 {
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value419) => {
                                                                        source_ml_source1_data_property_value419
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'properties')")).encode_utf16().collect(), message_present: true }).raise()
                                                        }
                                                    }; let ml_callback3 = { let at = at; std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> bool { position != at }) }; ml_values3.iter().cloned().enumerate().filter_map(|(ml_index3, ml_value3)| if ml_callback3(ml_value3.clone(), (ml_index3 as f64)) { Some(ml_value3) } else { None }).collect::<Vec<_>>() };
                                                    {
                                                        let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject({
                                                            let source_ml_source1_nullish_field_receiver420 = (object.clone(),);
                                                            if match (source_ml_source1_nullish_field_receiver420.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                {
                                                                    let source_ml_source1_nullish_field_present421 = ((source_ml_source1_nullish_field_receiver420.clone()).0).clone().expect("a proven present value");
                                                                    match source_ml_source1_nullish_field_present421 {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value422, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value422.clone()
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'identity')")).encode_utf16().collect(), message_present: true }).raise()
                                                            }
                                                        }, Box::new({
                                                            let source_ml_source1_nullish_field_receiver423 = (object.clone(),);
                                                            if match (source_ml_source1_nullish_field_receiver423.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                {
                                                                    let source_ml_source1_nullish_field_present424 = ((source_ml_source1_nullish_field_receiver423.clone()).0).clone().expect("a proven present value");
                                                                    match source_ml_source1_nullish_field_present424 {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value425, _, _, _) => {
                                                                            let source_ml_source1_data_property_value425 = *source_ml_source1_data_property_value425;
                                                                            source_ml_source1_data_property_value425.clone()
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'prototype')")).encode_utf16().collect(), message_present: true }).raise()
                                                            }
                                                        }), {
                                                            let source_ml_source1_nullish_field_receiver426 = (object.clone(),);
                                                            if match (source_ml_source1_nullish_field_receiver426.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                {
                                                                    let source_ml_source1_nullish_field_present427 = ((source_ml_source1_nullish_field_receiver426.clone()).0).clone().expect("a proven present value");
                                                                    match source_ml_source1_nullish_field_present427 {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value428, _, _) => {
                                                                            source_ml_source1_data_property_value428
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'extensible')")).encode_utf16().collect(), message_present: true }).raise()
                                                            }
                                                        }, {
                                                            let source_ml_source1_nullish_field_receiver429 = (object.clone(),);
                                                            if match (source_ml_source1_nullish_field_receiver429.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                {
                                                                    let source_ml_source1_nullish_field_present430 = ((source_ml_source1_nullish_field_receiver429.clone()).0).clone().expect("a proven present value");
                                                                    match source_ml_source1_nullish_field_present430 {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value431, _) => {
                                                                            source_ml_source1_data_property_value431
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'callable')")).encode_utf16().collect(), message_present: true }).raise()
                                                            }
                                                        }, properties);
                                                        {
                                                            let source_ml_source1_field432 = self::replace_java_script_ordinary_object(heap.clone(), replacement);
                                                            {
                                                                let source_ml_source1_field433 = true;
                                                                {
                                                                    let source_ml_source1_field434 = String::new();
                                                                    (source_ml_source1_field434, source_ml_source1_field432, source_ml_source1_field433)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                {
                                                    let source_ml_source1_field414 = heap.clone();
                                                    {
                                                        let source_ml_source1_field415 = false;
                                                        {
                                                            let source_ml_source1_field416 = String::new();
                                                            (source_ml_source1_field416, source_ml_source1_field414, source_ml_source1_field415)
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
