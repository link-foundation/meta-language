// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayIterationTransition;
use super::JavaScriptArrayIteratorCall;
use super::JavaScriptArrayIteratorState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryBeginJavaScriptArrayIteratorNextLoop1100Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_source1_temporary_begin_java_script_array_iterator_next_loop1100;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn replace_java_script_array_iterator_slots(
    iterator: self::JavaScriptOrdinaryObject,
    collection: self::ml::Big,
    cursor: f64,
    executing: bool,
) -> self::JavaScriptOrdinaryObject {
    match iterator.clone() {
        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
            source_ml_source1_temporary_constructor_field1090,
            source_ml_source1_temporary_constructor_field1091,
            source_ml_source1_temporary_constructor_field1092,
            source_ml_source1_temporary_constructor_field1093,
            source_ml_source1_temporary_constructor_field1094,
            _,
            _,
            kind,
            collection_kind,
            _,
            iterated_text,
        ) => {
            let source_ml_source1_temporary_constructor_field1091 =
                *source_ml_source1_temporary_constructor_field1091;
            if {
                let source_ml_source2_own_property_operands24003 = (
                    iterator.clone(),
                    self::ml_string::Text::from_units(vec![
                        105, 116, 101, 114, 97, 116, 101, 100, 84, 101, 120, 116,
                    ]),
                );
                {
                    let source_ml_source2_own_property_key24004 =
                        ((source_ml_source2_own_property_operands24003.clone()).1,);
                    match (source_ml_source2_own_property_operands24003.clone()).0 {
                        self::JavaScriptOrdinaryObject::ClosureObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_owned_constructor_field24005,
                        ) => {
                            (((((((((((((source_ml_source2_own_property_key24004
                                .clone())
                            .0) == (self::ml_string::Text::from_units(vec![
                                36,
                            ]))) || (((source_ml_source2_own_property_key24004.clone()).0)
                                == (self::ml_string::Text::from_units(vec![
                                    105, 100, 101, 110, 116, 105, 116, 121,
                                ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 116, 111, 116, 121, 112, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        101, 120, 116, 101, 110, 115, 105, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        99, 97, 108, 108, 97, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 112, 101, 114, 116, 105, 101, 115,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        111, 119, 110, 101, 114,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        111, 114, 100, 105, 110, 97, 108,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        99, 97, 112, 116, 117, 114, 101, 115,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        108, 101, 120, 105, 99, 97, 108, 82, 101, 99, 101, 105,
                                        118, 101, 114,
                                    ]))))
                                || ((((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        108, 101, 120, 105, 99, 97, 108, 78, 101, 119, 84, 97, 114,
                                        103, 101, 116,
                                    ])))
                                    && match source_ml_source2_owned_constructor_field24005.clone()
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    }))
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            (((((((((((source_ml_source2_own_property_key24004.clone()).0)
                                == (self::ml_string::Text::from_units(vec![36])))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        105, 100, 101, 110, 116, 105, 116, 121,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 116, 111, 116, 121, 112, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        101, 120, 116, 101, 110, 115, 105, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        99, 97, 108, 108, 97, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 112, 101, 114, 116, 105, 101, 115,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        98, 111, 117, 110, 100, 84, 97, 114, 103, 101, 116,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        98, 111, 117, 110, 100, 84, 104, 105, 115,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        98, 111, 117, 110, 100, 65, 114, 103, 117, 109, 101, 110,
                                        116, 115,
                                    ]))))
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                            ((((((((source_ml_source2_own_property_key24004.clone()).0)
                                == (self::ml_string::Text::from_units(vec![36])))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        105, 100, 101, 110, 116, 105, 116, 121,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 116, 111, 116, 121, 112, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        101, 120, 116, 101, 110, 115, 105, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        99, 97, 108, 108, 97, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 112, 101, 114, 116, 105, 101, 115,
                                    ]))))
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_owned_constructor_field24006,
                            source_ml_source2_owned_constructor_field24007,
                        ) => {
                            ((((((((((((((source_ml_source2_own_property_key24004
                                .clone())
                            .0) == (self::ml_string::Text::from_units(
                                vec![36],
                            ))) || (((source_ml_source2_own_property_key24004.clone()).0)
                                == (self::ml_string::Text::from_units(vec![
                                    105, 100, 101, 110, 116, 105, 116, 121,
                                ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 116, 111, 116, 121, 112, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        101, 120, 116, 101, 110, 115, 105, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        99, 97, 108, 108, 97, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 112, 101, 114, 116, 105, 101, 115,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        99, 111, 108, 108, 101, 99, 116, 105, 111, 110,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        99, 117, 114, 115, 111, 114,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        107, 105, 110, 100,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 75, 105,
                                        110, 100,
                                    ]))))
                                || ((((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        101, 120, 101, 99, 117, 116, 105, 110, 103,
                                    ])))
                                    && match source_ml_source2_owned_constructor_field24006.clone()
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    }))
                                || ((((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        105, 116, 101, 114, 97, 116, 101, 100, 84, 101, 120, 116,
                                    ])))
                                    && match source_ml_source2_owned_constructor_field24007.clone()
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    }))
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                            (((((((((source_ml_source2_own_property_key24004.clone()).0)
                                == (self::ml_string::Text::from_units(vec![36])))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        105, 100, 101, 110, 116, 105, 116, 121,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 116, 111, 116, 121, 112, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        101, 120, 116, 101, 110, 115, 105, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        99, 97, 108, 108, 97, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 112, 101, 114, 116, 105, 101, 115,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        101, 110, 116, 114, 105, 101, 115,
                                    ]))))
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                            (((((((((source_ml_source2_own_property_key24004.clone()).0)
                                == (self::ml_string::Text::from_units(vec![36])))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        105, 100, 101, 110, 116, 105, 116, 121,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 116, 111, 116, 121, 112, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        101, 120, 116, 101, 110, 115, 105, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        99, 97, 108, 108, 97, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 112, 101, 114, 116, 105, 101, 115,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        101, 110, 116, 114, 105, 101, 115,
                                    ]))))
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                            (((((((((source_ml_source2_own_property_key24004.clone()).0)
                                == (self::ml_string::Text::from_units(vec![36])))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        105, 100, 101, 110, 116, 105, 116, 121,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 116, 111, 116, 121, 112, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        101, 120, 116, 101, 110, 115, 105, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        99, 97, 108, 108, 97, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 112, 101, 114, 116, 105, 101, 115,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        119, 101, 97, 107, 69, 110, 116, 114, 105, 101, 115,
                                    ]))))
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                            (((((((((source_ml_source2_own_property_key24004.clone()).0)
                                == (self::ml_string::Text::from_units(vec![36])))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        105, 100, 101, 110, 116, 105, 116, 121,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 116, 111, 116, 121, 112, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        101, 120, 116, 101, 110, 115, 105, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        99, 97, 108, 108, 97, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 112, 101, 114, 116, 105, 101, 115,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        119, 101, 97, 107, 69, 110, 116, 114, 105, 101, 115,
                                    ]))))
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            _,
                            _,
                            _,
                            source_ml_source2_owned_constructor_field24008,
                            source_ml_source2_owned_constructor_field24009,
                            source_ml_source2_owned_constructor_field24010,
                            source_ml_source2_owned_constructor_field24011,
                            _,
                            _,
                        ) => {
                            ((((((((((((source_ml_source2_own_property_key24004.clone())
                                .0)
                                == (self::ml_string::Text::from_units(vec![36])))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        105, 100, 101, 110, 116, 105, 116, 121,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 116, 111, 116, 121, 112, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        101, 120, 116, 101, 110, 115, 105, 98, 108, 101,
                                    ]))))
                                || ((((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        105, 109, 109, 117, 116, 97, 98, 108, 101, 80, 114, 111,
                                        116, 111, 116, 121, 112, 101,
                                    ])))
                                    && match source_ml_source2_owned_constructor_field24008.clone()
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    }))
                                || ((((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        114, 101, 103, 117, 108, 97, 114, 69, 120, 112, 114, 101,
                                        115, 115, 105, 111, 110,
                                    ])))
                                    && match source_ml_source2_owned_constructor_field24009.clone()
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    }))
                                || ((((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        115, 116, 114, 105, 110, 103, 68, 97, 116, 97,
                                    ])))
                                    && match source_ml_source2_owned_constructor_field24010.clone()
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    }))
                                || ((((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 105, 109, 105, 116, 105, 118, 101, 68, 97, 116,
                                        97,
                                    ])))
                                    && match source_ml_source2_owned_constructor_field24011.clone()
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    }))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        99, 97, 108, 108, 97, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 112, 101, 114, 116, 105, 101, 115,
                                    ]))))
                        }
                        _ => {
                            ((((((((((source_ml_source2_own_property_key24004.clone()).0)
                                == (self::ml_string::Text::from_units(vec![36])))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        105, 100, 101, 110, 116, 105, 116, 121,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 116, 111, 116, 121, 112, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        101, 120, 116, 101, 110, 115, 105, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        99, 97, 108, 108, 97, 98, 108, 101,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 112, 101, 114, 116, 105, 101, 115,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        104, 111, 115, 116, 71, 101, 116,
                                    ]))))
                                || (((source_ml_source2_own_property_key24004.clone()).0)
                                    == (self::ml_string::Text::from_units(vec![
                                        104, 111, 115, 116, 83, 101, 116,
                                    ]))))
                        }
                    }
                }
            } {
                {
                    let source_ml_source2_constructor_field24021 = ((iterated_text
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                    .0)
                        .clone();
                    {
                        let source_ml_source2_constructor_field24022 = Some(executing);
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                            source_ml_source1_temporary_constructor_field1090.clone(),
                            Box::new(source_ml_source1_temporary_constructor_field1091.clone()),
                            source_ml_source1_temporary_constructor_field1092,
                            source_ml_source1_temporary_constructor_field1093,
                            source_ml_source1_temporary_constructor_field1094.clone(),
                            collection.clone(),
                            cursor,
                            kind.clone(),
                            collection_kind.clone(),
                            Some((source_ml_source2_constructor_field24022.clone(),)),
                            Some((source_ml_source2_constructor_field24021.clone(),)),
                        )
                    }
                }
            } else {
                {
                    let source_ml_source2_constructor_field24032 = Some(executing);
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        source_ml_source1_temporary_constructor_field1090.clone(),
                        Box::new(source_ml_source1_temporary_constructor_field1091.clone()),
                        source_ml_source1_temporary_constructor_field1092,
                        source_ml_source1_temporary_constructor_field1093,
                        source_ml_source1_temporary_constructor_field1094.clone(),
                        collection.clone(),
                        cursor,
                        kind.clone(),
                        collection_kind.clone(),
                        Some((source_ml_source2_constructor_field24032.clone(),)),
                        None::<(Option<self::JavaScriptSemanticValue>,)>,
                    )
                }
            }
        }
        _ => iterator.clone(),
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_array_iterator_next(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
    exotic_key: self::JavaScriptSemanticValue,
) -> self::JavaScriptArrayIterationTransition {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                {
                    let source_ml_source1_temporary_values1099 =
                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                            receiver.clone(),
                            exotic_key.clone(),
                        ]));
                    {
                        let source_ml_source1_temporary_index1099 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s24033 = self::source_ml_source1_temporary_begin_java_script_array_iterator_next_loop1100(heap.clone(), source_ml_source1_temporary_values1099.clone(), source_ml_source1_temporary_index1099.clone());
                            match source_ml_source2_s24033.clone() {
                                    self::MlSource1TemporaryBeginJavaScriptArrayIteratorNextLoop1100Result::MlSource1TemporaryBeginJavaScriptArrayIteratorNextLoop1100Done => {
                                        match exotic_key.clone() {
                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                {
                                                    let iterator = match receiver.clone() {
                                                        self::JavaScriptSemanticValue::Object(identity, _) => {
                                                            {
                                                                let source_ml_source2_array_read_value24038 = shared_constructor_field1.clone();
                                                                {
                                                                    let source_ml_source2_array_read_index24039 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                                                    { let ml_reference_array1254 = source_ml_source2_array_read_value24038.clone(); let ml_reference_position1254 = source_ml_source2_array_read_index24039; let ml_reference_projection1254 = self::ml_array::optional_at(&*ml_reference_array1254.borrow(), self::ml_array::number_index(ml_reference_position1254)); ml_reference_projection1254 }
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            None::<self::JavaScriptOrdinaryObject>
                                                        }
                                                    };
                                                    if (match (iterator.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match iterator.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) {
                                                        self::JavaScriptArrayIterationTransition::ArrayIteratorError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 73, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                    } else {
                                                        if (((((match iterator.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value24052, _, _) => {
                                                                Some((source_ml_source2_data_property_value24052.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml_string::Text,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone()) != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) {
                                                            self::JavaScriptArrayIterationTransition::ArrayIteratorError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 73, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                        } else {
                                                            if match (((match iterator.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value24053, _) => {
                                                                    Some((((source_ml_source2_data_property_value24053.clone().unwrap_or_else(|| (None::<bool>,))).0).clone(),))
                                                                }
                                                                _ => {
                                                                    None::<(Option<bool>,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0).clone() { Some(ml_present) => ml_present, None => false } {
                                                                self::JavaScriptArrayIterationTransition::ArrayIteratorError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![71, 101, 110, 101, 114, 97, 116, 111, 114, 32, 105, 115, 32, 97, 108, 114, 101, 97, 100, 121, 32, 114, 117, 110, 110, 105, 110, 103]))
                                                            } else {
                                                                if (((((match iterator.clone().expect("a proven present value") {
                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, source_ml_source2_data_property_value24055, _, _, _, _, _) => {
                                                                        Some((source_ml_source2_data_property_value24055.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(self::ml::Big,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone()) == (self::ml::Big::from_i128(0))) {
                                                                    self::JavaScriptArrayIterationTransition::ArrayIteratorComponents(Box::new(heap.clone()), Box::new(receiver.clone()), true, Box::new(self::JavaScriptSemanticValue::Undefined), Box::new(self::JavaScriptSemanticValue::Undefined), false)
                                                                } else {
                                                                    {
                                                                        let source = {
                                                                            let source_ml_source2_array_read_value24059 = shared_constructor_field1.clone();
                                                                            {
                                                                                let source_ml_source2_array_read_index24060 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), (((match iterator.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, source_ml_source2_data_property_value24058, _, _, _, _, _) => {
                                                                                        Some((source_ml_source2_data_property_value24058.clone(),))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(self::ml::Big,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0).clone());
                                                                                { let ml_reference_array1255 = source_ml_source2_array_read_value24059.clone(); let ml_reference_position1255 = source_ml_source2_array_read_index24060; let ml_reference_projection1255 = self::ml_array::optional_at(&*ml_reference_array1255.borrow(), self::ml_array::number_index(ml_reference_position1255)); ml_reference_projection1255 }
                                                                            }
                                                                        };
                                                                        if match (source.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                            self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 109, 97, 105, 110, 115, 32, 105, 110, 32, 116, 104, 101, 32, 99, 111, 109, 109, 111, 110, 32, 104, 101, 97, 112]))
                                                                        } else {
                                                                            self::JavaScriptArrayIterationTransition::ArrayIteratorContinue(Box::new(self::replace_java_script_ordinary_object(heap.clone(), self::replace_java_script_array_iterator_slots(iterator.clone().expect("a proven present value"), (((match iterator.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, source_ml_source2_data_property_value24061, _, _, _, _, _) => {
                                                                                    Some((source_ml_source2_data_property_value24061.clone(),))
                                                                                }
                                                                                _ => {
                                                                                    None::<(self::ml::Big,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0).clone(), ((match iterator.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, source_ml_source2_data_property_value24062, _, _, _, _) => {
                                                                                    Some((source_ml_source2_data_property_value24062,))
                                                                                }
                                                                                _ => {
                                                                                    None::<(f64,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0, true))), Box::new({
                                                                                let source_ml_source2_constructor_field24090 = self::JavaScriptSemanticValue::Object(match source.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value24064, _, _, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24064.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value24065, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24065.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value24066, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24066.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value24067, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24067.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value24068, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24068.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value24069, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24069.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value24070, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24070.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value24071, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24071.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value24072, _, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24072.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value24073, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24073.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value24074, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24074.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value24075, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24075.clone()
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value24076, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24076.clone()
                                                                                    }
                                                                                }, match source.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value24077, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24077
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value24078, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24078
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value24079, _) => {
                                                                                        source_ml_source2_data_property_value24079
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value24080, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24080
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value24081, _, _) => {
                                                                                        source_ml_source2_data_property_value24081
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value24082, _, _) => {
                                                                                        source_ml_source2_data_property_value24082
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value24083, _, _) => {
                                                                                        source_ml_source2_data_property_value24083
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value24084, _, _) => {
                                                                                        source_ml_source2_data_property_value24084
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value24085, _) => {
                                                                                        source_ml_source2_data_property_value24085
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value24086, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24086
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value24087, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24087
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value24088, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24088
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value24089, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24089
                                                                                    }
                                                                                });
                                                                                {
                                                                                    let source_ml_source2_constructor_field24093 = ((match iterator.clone().expect("a proven present value") {
                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, source_ml_source2_data_property_value24092, _, _, _, _) => {
                                                                                            Some((source_ml_source2_data_property_value24092,))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(f64,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0;
                                                                                    {
                                                                                        let source_ml_source2_constructor_field24095 = (((match iterator.clone().expect("a proven present value") {
                                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value24094, _, _, _) => {
                                                                                                Some((source_ml_source2_data_property_value24094.clone(),))
                                                                                            }
                                                                                            _ => {
                                                                                                None::<(self::ml_string::Text,)>
                                                                                            }
                                                                                        }).clone().expect("a proven present value")).0).clone();
                                                                                        {
                                                                                            let source_ml_source2_constructor_field24097 = self::JavaScriptSemanticValue::Undefined;
                                                                                            self::JavaScriptArrayIteratorState::ArrayIteratorState(Box::new(receiver.clone()), Box::new(source_ml_source2_constructor_field24090.clone()), Box::new(exotic_key.clone()), source_ml_source2_constructor_field24093, source_ml_source2_constructor_field24095.clone(), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]), Box::new(source_ml_source2_constructor_field24097.clone()), None::<(Option<self::JavaScriptPrimitiveConversionState>,)>, None::<(Option<self::JavaScriptArrayIteratorCall>,)>)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 101, 120, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 83, 121, 109, 98, 111, 108]))
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryBeginJavaScriptArrayIteratorNextLoop1100Result::MlSource1TemporaryBeginJavaScriptArrayIteratorNextLoop1100Return(source_ml_source1_temporary_result1100) => {
                                        let source_ml_source1_temporary_result1100 = *source_ml_source1_temporary_result1100;
                                        source_ml_source1_temporary_result1100.clone()
                                    }
                                }
                        }
                    }
                }
            }
        }
    }
}
