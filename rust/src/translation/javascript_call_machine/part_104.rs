// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptObjectFromEntriesCall;
use super::JavaScriptObjectFromEntriesState;
use super::JavaScriptObjectFromEntriesTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptSemanticValue;
use super::begin_java_script_property_key_conversion;
use super::clear_java_script_object_from_entries_child;
use super::fail_java_script_object_from_entries;
use super::ml_string;
use super::throw_java_script_object_from_entries;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_object_from_entries_key(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectFromEntriesState,
    transition: self::JavaScriptPropertyKeyConversionTransition,
) -> self::JavaScriptObjectFromEntriesTransition {
    {
        let base = self::clear_java_script_object_from_entries_child(state.clone());
        match base.clone() {
            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
                shared_constructor_field4,
                shared_constructor_field5,
                shared_constructor_field6,
                shared_constructor_field7,
                shared_constructor_field8,
                shared_constructor_field9,
                shared_constructor_field10,
                shared_constructor_field11,
                shared_constructor_field12,
                shared_constructor_field13,
                shared_constructor_field14,
                shared_constructor_field15,
                shared_constructor_field16,
                shared_constructor_field17,
                shared_constructor_field18,
                shared_constructor_field19,
                shared_constructor_field20,
            ) => {
                let shared_constructor_field4 = *shared_constructor_field4;
                let shared_constructor_field5 = *shared_constructor_field5;
                let shared_constructor_field6 = *shared_constructor_field6;
                let shared_constructor_field7 = *shared_constructor_field7;
                let shared_constructor_field8 = *shared_constructor_field8;
                let shared_constructor_field10 = *shared_constructor_field10;
                let shared_constructor_field11 = *shared_constructor_field11;
                let shared_constructor_field12 = *shared_constructor_field12;
                let shared_constructor_field13 = *shared_constructor_field13;
                match transition.clone() {
                    self::JavaScriptPropertyKeyConversionTransition::PropertyKeyContinuation(
                        state_2,
                    ) => {
                        let state_2 = *state_2;
                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesContinue(
                            Box::new(heap.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field20889 =
                                    shared_constructor_field4.clone();
                                {
                                    let source_ml_source2_nominal_field20891 =
                                        shared_constructor_field5.clone();
                                    {
                                        let source_ml_source2_nominal_field20893 =
                                            shared_constructor_field6.clone();
                                        {
                                            let source_ml_source2_nominal_field20895 =
                                                shared_constructor_field7.clone();
                                            {
                                                let source_ml_source2_nominal_field20897 =
                                                    shared_constructor_field8.clone();
                                                {
                                                    let source_ml_source2_nominal_field20899 =
                                                        shared_constructor_field9.clone();
                                                    {
                                                        let source_ml_source2_nominal_field20901 =
                                                            shared_constructor_field10.clone();
                                                        {
                                                            let source_ml_source2_nominal_field20903 =
                                                                shared_constructor_field11.clone();
                                                            {
                                                                let source_ml_source2_nominal_field20905 =
                                                                    shared_constructor_field12
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field20907 =
                                                                        shared_constructor_field13
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field20909 = shared_constructor_field14.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field20911 = shared_constructor_field15.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field20913 = shared_constructor_field16.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field20915 = shared_constructor_field17.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field20917 = shared_constructor_field18.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field20919 = shared_constructor_field19.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field20921 = shared_constructor_field20.clone();
                                                                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field20889.clone()), Box::new(source_ml_source2_nominal_field20891.clone()), Box::new(source_ml_source2_nominal_field20893.clone()), Box::new(source_ml_source2_nominal_field20895.clone()), Box::new(source_ml_source2_nominal_field20897.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 107, 101, 121]), Box::new(source_ml_source2_nominal_field20901.clone()), Box::new(source_ml_source2_nominal_field20903.clone()), Box::new(source_ml_source2_nominal_field20905.clone()), Box::new(source_ml_source2_nominal_field20907.clone()), source_ml_source2_nominal_field20909.clone(), source_ml_source2_nominal_field20911.clone(), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field20915.clone(), source_ml_source2_nominal_field20917.clone(), source_ml_source2_nominal_field20919.clone(), source_ml_source2_nominal_field20921.clone())
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
                            }),
                        )
                    }
                    self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCall(
                        state_3,
                        callee,
                        receiver,
                        arguments,
                    ) => {
                        let state_3 = *state_3;
                        let callee = *callee;
                        let receiver = *receiver;
                        {
                            let call =
                                self::JavaScriptObjectFromEntriesCall::ObjectFromEntriesCallRecord(
                                    Box::new(callee.clone()),
                                    Box::new(receiver.clone()),
                                    arguments.clone(),
                                );
                            match call.clone() {
                                self::JavaScriptObjectFromEntriesCall::ObjectFromEntriesCallRecord(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                    let shared_constructor_field1 = *shared_constructor_field1;
                                    let shared_constructor_field2 = *shared_constructor_field2;
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesCall(Box::new(heap.clone()), Box::new({
                                        let source_ml_source2_nominal_field20926 = shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_nominal_field20928 = shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_nominal_field20930 = shared_constructor_field6.clone();
                                                {
                                                    let source_ml_source2_nominal_field20932 = shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_nominal_field20934 = shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_nominal_field20936 = shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_nominal_field20938 = shared_constructor_field10.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field20940 = shared_constructor_field11.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field20942 = shared_constructor_field12.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field20944 = shared_constructor_field13.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field20946 = shared_constructor_field14.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field20948 = shared_constructor_field15.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field20950 = shared_constructor_field16.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field20952 = shared_constructor_field17.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field20954 = shared_constructor_field18.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field20956 = shared_constructor_field19.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field20958 = shared_constructor_field20.clone();
                                                                                                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field20926.clone()), Box::new(source_ml_source2_nominal_field20928.clone()), Box::new(source_ml_source2_nominal_field20930.clone()), Box::new(source_ml_source2_nominal_field20932.clone()), Box::new(source_ml_source2_nominal_field20934.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 107, 101, 121]), Box::new(source_ml_source2_nominal_field20938.clone()), Box::new(source_ml_source2_nominal_field20940.clone()), Box::new(source_ml_source2_nominal_field20942.clone()), Box::new(source_ml_source2_nominal_field20944.clone()), source_ml_source2_nominal_field20946.clone(), source_ml_source2_nominal_field20948.clone(), Some((Some(state_3.clone()),)), Some((Some(call.clone()),)), source_ml_source2_nominal_field20954.clone(), source_ml_source2_nominal_field20956.clone(), source_ml_source2_nominal_field20958.clone())
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
                                    }), Box::new(shared_constructor_field1.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone())
                                }
                            }
                        }
                    }
                    self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(key) => {
                        let key = *key;
                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesContinue(
                            Box::new(heap.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field20967 =
                                    shared_constructor_field4.clone();
                                {
                                    let source_ml_source2_nominal_field20969 =
                                        shared_constructor_field5.clone();
                                    {
                                        let source_ml_source2_nominal_field20971 =
                                            shared_constructor_field6.clone();
                                        {
                                            let source_ml_source2_nominal_field20973 =
                                                shared_constructor_field7.clone();
                                            {
                                                let source_ml_source2_nominal_field20975 =
                                                    shared_constructor_field8.clone();
                                                {
                                                    let source_ml_source2_nominal_field20977 =
                                                        shared_constructor_field9.clone();
                                                    {
                                                        let source_ml_source2_nominal_field20979 =
                                                            shared_constructor_field10.clone();
                                                        {
                                                            let source_ml_source2_nominal_field20981 =
                                                                shared_constructor_field11.clone();
                                                            {
                                                                let source_ml_source2_nominal_field20983 =
                                                                    shared_constructor_field12
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field20985 =
                                                                        shared_constructor_field13
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field20987 = shared_constructor_field14.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field20989 = shared_constructor_field15.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field20991 = shared_constructor_field16.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field20993 = shared_constructor_field17.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field20995 = shared_constructor_field18.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field20997 = shared_constructor_field19.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field20999 = shared_constructor_field20.clone();
                                                                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field20967.clone()), Box::new(source_ml_source2_nominal_field20969.clone()), Box::new(source_ml_source2_nominal_field20971.clone()), Box::new(source_ml_source2_nominal_field20973.clone()), Box::new(source_ml_source2_nominal_field20975.clone()), self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101]), Box::new(source_ml_source2_nominal_field20979.clone()), Box::new(source_ml_source2_nominal_field20981.clone()), Box::new(source_ml_source2_nominal_field20983.clone()), Box::new(key.clone()), source_ml_source2_nominal_field20987.clone(), source_ml_source2_nominal_field20989.clone(), source_ml_source2_nominal_field20991.clone(), source_ml_source2_nominal_field20993.clone(), source_ml_source2_nominal_field20995.clone(), source_ml_source2_nominal_field20997.clone(), source_ml_source2_nominal_field20999.clone())
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
                            }),
                        )
                    }
                    self::JavaScriptPropertyKeyConversionTransition::PropertyKeyThrow(value) => {
                        let value = *value;
                        self::throw_java_script_object_from_entries(
                            heap.clone(),
                            base.clone(),
                            value.clone(),
                        )
                    }
                    self::JavaScriptPropertyKeyConversionTransition::PropertyKeyIntrinsicError(
                        error_class,
                        reason,
                    ) => self::fail_java_script_object_from_entries(
                        heap.clone(),
                        base.clone(),
                        &(error_class),
                        &(reason),
                    ),
                    self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(
                        reason_2,
                    ) => self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(
                        Box::new(heap.clone()),
                        reason_2.clone(),
                    ),
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_object_from_entries_entry_value(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectFromEntriesState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptObjectFromEntriesTransition {
    match state.clone() {
        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
            _,
            _,
            _,
            shared_constructor_field18,
            _,
            shared_constructor_field19,
            _,
            shared_constructor_field20,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field18 = *shared_constructor_field18;
            let shared_constructor_field20 = *shared_constructor_field20;
            {
                let base = self::clear_java_script_object_from_entries_child(state.clone());
                match base.clone() {
                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
                        shared_constructor_field1,
                        shared_constructor_field2,
                        shared_constructor_field3,
                        shared_constructor_field4,
                        shared_constructor_field5,
                        shared_constructor_field6,
                        shared_constructor_field7,
                        shared_constructor_field8,
                        shared_constructor_field9,
                        shared_constructor_field10,
                        shared_constructor_field11,
                        shared_constructor_field12,
                        shared_constructor_field13,
                        shared_constructor_field14,
                        shared_constructor_field15,
                        shared_constructor_field16,
                        shared_constructor_field17,
                    ) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        let shared_constructor_field2 = *shared_constructor_field2;
                        let shared_constructor_field3 = *shared_constructor_field3;
                        let shared_constructor_field4 = *shared_constructor_field4;
                        let shared_constructor_field5 = *shared_constructor_field5;
                        let shared_constructor_field7 = *shared_constructor_field7;
                        let shared_constructor_field8 = *shared_constructor_field8;
                        let shared_constructor_field9 = *shared_constructor_field9;
                        let shared_constructor_field10 = *shared_constructor_field10;
                        if ((shared_constructor_field19
                            == (self::ml_string::Text::from_units(vec![
                                101, 110, 116, 114, 121, 32, 107, 101, 121,
                            ])))
                            || (shared_constructor_field19
                                == (self::ml_string::Text::from_units(vec![
                                    101, 110, 116, 114, 121, 32, 107, 101, 121, 32, 103, 101, 116,
                                    116, 101, 114,
                                ]))))
                        {
                            self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesContinue(
                                Box::new(heap.clone()),
                                Box::new({
                                    let source_ml_source2_nominal_field21008 =
                                        shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_nominal_field21010 =
                                            shared_constructor_field2.clone();
                                        {
                                            let source_ml_source2_nominal_field21012 =
                                                shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field21014 =
                                                    shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_nominal_field21016 =
                                                        shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_nominal_field21018 =
                                                            shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field21020 =
                                                                shared_constructor_field7.clone();
                                                            {
                                                                let source_ml_source2_nominal_field21022 =
                                                                    shared_constructor_field8
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field21024 =
                                                                        shared_constructor_field9
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field21026 = shared_constructor_field10.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field21028 = shared_constructor_field11.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field21030 = shared_constructor_field12.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field21032 = shared_constructor_field13.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field21034 = shared_constructor_field14.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field21036 = shared_constructor_field15.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field21038 = shared_constructor_field16.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field21040 = shared_constructor_field17.clone();
                                                                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field21008.clone()), Box::new(source_ml_source2_nominal_field21010.clone()), Box::new(source_ml_source2_nominal_field21012.clone()), Box::new(source_ml_source2_nominal_field21014.clone()), Box::new(source_ml_source2_nominal_field21016.clone()), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 118, 97, 108, 117, 101]), Box::new(source_ml_source2_nominal_field21020.clone()), Box::new(value.clone()), Box::new(source_ml_source2_nominal_field21024.clone()), Box::new(source_ml_source2_nominal_field21026.clone()), source_ml_source2_nominal_field21028.clone(), source_ml_source2_nominal_field21030.clone(), source_ml_source2_nominal_field21032.clone(), source_ml_source2_nominal_field21034.clone(), source_ml_source2_nominal_field21036.clone(), source_ml_source2_nominal_field21038.clone(), source_ml_source2_nominal_field21040.clone())
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
                                }),
                            )
                        } else {
                            self::accept_java_script_object_from_entries_key(
                                heap.clone(),
                                {
                                    let source_ml_source2_nominal_field21045 =
                                        shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_nominal_field21047 =
                                            shared_constructor_field2.clone();
                                        {
                                            let source_ml_source2_nominal_field21049 =
                                                shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field21051 =
                                                    shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_nominal_field21053 =
                                                        shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_nominal_field21055 =
                                                            shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field21057 =
                                                                shared_constructor_field7.clone();
                                                            {
                                                                let source_ml_source2_nominal_field21059 =
                                                                    shared_constructor_field8
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field21061 =
                                                                        shared_constructor_field9
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field21063 = shared_constructor_field10.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field21065 = shared_constructor_field11.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field21067 = shared_constructor_field12.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field21069 = shared_constructor_field13.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field21071 = shared_constructor_field14.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field21073 = shared_constructor_field15.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field21075 = shared_constructor_field16.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field21077 = shared_constructor_field17.clone();
                                                                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field21045.clone()), Box::new(source_ml_source2_nominal_field21047.clone()), Box::new(source_ml_source2_nominal_field21049.clone()), Box::new(source_ml_source2_nominal_field21051.clone()), Box::new(source_ml_source2_nominal_field21053.clone()), source_ml_source2_nominal_field21055.clone(), Box::new(source_ml_source2_nominal_field21057.clone()), Box::new(source_ml_source2_nominal_field21059.clone()), Box::new(value.clone()), Box::new(source_ml_source2_nominal_field21063.clone()), source_ml_source2_nominal_field21065.clone(), source_ml_source2_nominal_field21067.clone(), source_ml_source2_nominal_field21069.clone(), source_ml_source2_nominal_field21071.clone(), source_ml_source2_nominal_field21073.clone(), source_ml_source2_nominal_field21075.clone(), source_ml_source2_nominal_field21077.clone())
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
                                },
                                self::begin_java_script_property_key_conversion(
                                    heap.clone(),
                                    shared_constructor_field20.clone(),
                                    shared_constructor_field18.clone(),
                                ),
                            )
                        }
                    }
                }
            }
        }
    }
}
