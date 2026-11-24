// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptIteratorCompletion;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptIteratorProtocolTransition;
use super::JavaScriptObjectFromEntriesCall;
use super::JavaScriptObjectFromEntriesState;
use super::JavaScriptObjectFromEntriesTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::clear_java_script_object_from_entries_child;
use super::fail_java_script_object_from_entries;
use super::ml_string;
use super::throw_java_script_object_from_entries;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_object_from_entries_iterator(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectFromEntriesState,
    transition: self::JavaScriptIteratorProtocolTransition,
) -> self::JavaScriptObjectFromEntriesTransition {
    {
        let cleared = self::clear_java_script_object_from_entries_child(state.clone());
        match cleared.clone() {
            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
                shared_constructor_field21,
                shared_constructor_field22,
                shared_constructor_field23,
                shared_constructor_field24,
                shared_constructor_field25,
                shared_constructor_field26,
                shared_constructor_field27,
                shared_constructor_field28,
                shared_constructor_field29,
                shared_constructor_field30,
                shared_constructor_field31,
                shared_constructor_field32,
                shared_constructor_field33,
                shared_constructor_field34,
                shared_constructor_field35,
                shared_constructor_field36,
                shared_constructor_field37,
            ) => {
                let shared_constructor_field21 = *shared_constructor_field21;
                let shared_constructor_field22 = *shared_constructor_field22;
                let shared_constructor_field23 = *shared_constructor_field23;
                let shared_constructor_field24 = *shared_constructor_field24;
                let shared_constructor_field25 = *shared_constructor_field25;
                let shared_constructor_field27 = *shared_constructor_field27;
                let shared_constructor_field28 = *shared_constructor_field28;
                let shared_constructor_field29 = *shared_constructor_field29;
                let shared_constructor_field30 = *shared_constructor_field30;
                {
                    let abrupt = match state.clone() {
                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value20528,
                            _,
                            _,
                        ) => ((source_ml_source2_data_property_value20528
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                        .0)
                            .clone(),
                    };
                    {
                        let base = if (((match state.clone() {
                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value20530,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value20530.clone(),
                        }) == (self::ml_string::Text::from_units(vec![
                            99, 108, 111, 115, 101,
                        ]))) && !(match (abrupt.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        })) {
                            {
                                let source_ml_source2_nominal_field20532 =
                                    shared_constructor_field21.clone();
                                {
                                    let source_ml_source2_nominal_field20534 =
                                        shared_constructor_field22.clone();
                                    {
                                        let source_ml_source2_nominal_field20536 =
                                            shared_constructor_field23.clone();
                                        {
                                            let source_ml_source2_nominal_field20538 =
                                                shared_constructor_field24.clone();
                                            {
                                                let source_ml_source2_nominal_field20540 =
                                                    shared_constructor_field25.clone();
                                                {
                                                    let source_ml_source2_nominal_field20542 =
                                                        shared_constructor_field26.clone();
                                                    {
                                                        let source_ml_source2_nominal_field20544 =
                                                            shared_constructor_field27.clone();
                                                        {
                                                            let source_ml_source2_nominal_field20546 =
                                                                shared_constructor_field28.clone();
                                                            {
                                                                let source_ml_source2_nominal_field20548 =
                                                                    shared_constructor_field29
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field20550 =
                                                                        shared_constructor_field30
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field20552 = shared_constructor_field31.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field20554 = shared_constructor_field32.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field20556 = shared_constructor_field33.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field20558 = shared_constructor_field34.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field20560 = shared_constructor_field35.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field20562 = shared_constructor_field36.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field20564 = shared_constructor_field37.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override20566 = abrupt.clone().expect("a proven present value");
                                                                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field20532.clone()), Box::new(source_ml_source2_nominal_field20534.clone()), Box::new(source_ml_source2_nominal_field20536.clone()), Box::new(source_ml_source2_nominal_field20538.clone()), Box::new(source_ml_source2_nominal_field20540.clone()), source_ml_source2_nominal_field20542.clone(), Box::new(source_ml_source2_nominal_field20544.clone()), Box::new(source_ml_source2_nominal_field20546.clone()), Box::new(source_ml_source2_nominal_field20548.clone()), Box::new(source_ml_source2_nominal_field20550.clone()), source_ml_source2_nominal_field20552.clone(), source_ml_source2_nominal_field20554.clone(), source_ml_source2_nominal_field20556.clone(), source_ml_source2_nominal_field20558.clone(), Some((Some(source_ml_source2_nominal_override20566.clone()),)), source_ml_source2_nominal_field20562.clone(), source_ml_source2_nominal_field20564.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
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
                            cleared.clone()
                        };
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
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(heap_2, state_2) => {
                                        let heap_2 = *heap_2;
                                        let state_2 = *state_2;
                                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesContinue(Box::new(heap_2.clone()), Box::new({
                                            let source_ml_source2_nominal_field20569 = shared_constructor_field4.clone();
                                            {
                                                let source_ml_source2_nominal_field20571 = shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_nominal_field20573 = shared_constructor_field6.clone();
                                                    {
                                                        let source_ml_source2_nominal_field20575 = shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_nominal_field20577 = shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_nominal_field20579 = shared_constructor_field9.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field20581 = shared_constructor_field10.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field20583 = shared_constructor_field11.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field20585 = shared_constructor_field12.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field20587 = shared_constructor_field13.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field20589 = shared_constructor_field14.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field20591 = shared_constructor_field15.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field20593 = shared_constructor_field16.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field20595 = shared_constructor_field17.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field20597 = shared_constructor_field18.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field20599 = shared_constructor_field19.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field20601 = shared_constructor_field20.clone();
                                                                                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field20569.clone()), Box::new(source_ml_source2_nominal_field20571.clone()), Box::new(source_ml_source2_nominal_field20573.clone()), Box::new(source_ml_source2_nominal_field20575.clone()), Box::new(source_ml_source2_nominal_field20577.clone()), source_ml_source2_nominal_field20579.clone(), Box::new(source_ml_source2_nominal_field20581.clone()), Box::new(source_ml_source2_nominal_field20583.clone()), Box::new(source_ml_source2_nominal_field20585.clone()), Box::new(source_ml_source2_nominal_field20587.clone()), source_ml_source2_nominal_field20589.clone(), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field20593.clone(), source_ml_source2_nominal_field20595.clone(), source_ml_source2_nominal_field20597.clone(), source_ml_source2_nominal_field20599.clone(), source_ml_source2_nominal_field20601.clone())
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }))
                                    }
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolCall(heap_3, state_3, callee, receiver, arguments) => {
                                        let heap_3 = *heap_3;
                                        let state_3 = *state_3;
                                        let callee = *callee;
                                        let receiver = *receiver;
                                        {
                                            let call = self::JavaScriptObjectFromEntriesCall::ObjectFromEntriesCallRecord(Box::new(callee.clone()), Box::new(receiver.clone()), arguments.clone());
                                            match call.clone() {
                                                self::JavaScriptObjectFromEntriesCall::ObjectFromEntriesCallRecord(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                                    let shared_constructor_field1 = *shared_constructor_field1;
                                                    let shared_constructor_field2 = *shared_constructor_field2;
                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesCall(Box::new(heap_3.clone()), Box::new(if match (abrupt.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        {
                                                            let source_ml_source2_nominal_field20605 = shared_constructor_field4.clone();
                                                            {
                                                                let source_ml_source2_nominal_field20607 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field20609 = shared_constructor_field6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field20611 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field20613 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field20615 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field20617 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field20619 = shared_constructor_field11.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field20621 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field20623 = shared_constructor_field13.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field20625 = shared_constructor_field14.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field20627 = shared_constructor_field15.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field20629 = shared_constructor_field16.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field20631 = shared_constructor_field17.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field20633 = shared_constructor_field18.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field20635 = shared_constructor_field19.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field20637 = shared_constructor_field20.clone();
                                                                                                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field20605.clone()), Box::new(source_ml_source2_nominal_field20607.clone()), Box::new(source_ml_source2_nominal_field20609.clone()), Box::new(source_ml_source2_nominal_field20611.clone()), Box::new(source_ml_source2_nominal_field20613.clone()), source_ml_source2_nominal_field20615.clone(), Box::new(source_ml_source2_nominal_field20617.clone()), Box::new(source_ml_source2_nominal_field20619.clone()), Box::new(source_ml_source2_nominal_field20621.clone()), Box::new(source_ml_source2_nominal_field20623.clone()), source_ml_source2_nominal_field20625.clone(), Some((Some(state_3.clone()),)), source_ml_source2_nominal_field20629.clone(), Some((Some(call.clone()),)), source_ml_source2_nominal_field20633.clone(), source_ml_source2_nominal_field20635.clone(), source_ml_source2_nominal_field20637.clone())
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
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
                                                            let source_ml_source2_nominal_field20642 = shared_constructor_field4.clone();
                                                            {
                                                                let source_ml_source2_nominal_field20644 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field20646 = shared_constructor_field6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field20648 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field20650 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field20652 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field20654 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field20656 = shared_constructor_field11.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field20658 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field20660 = shared_constructor_field13.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field20662 = shared_constructor_field14.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field20664 = shared_constructor_field15.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field20666 = shared_constructor_field16.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field20668 = shared_constructor_field17.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field20670 = shared_constructor_field18.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field20672 = shared_constructor_field19.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field20674 = shared_constructor_field20.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_override20676 = abrupt.clone().expect("a proven present value");
                                                                                                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field20642.clone()), Box::new(source_ml_source2_nominal_field20644.clone()), Box::new(source_ml_source2_nominal_field20646.clone()), Box::new(source_ml_source2_nominal_field20648.clone()), Box::new(source_ml_source2_nominal_field20650.clone()), source_ml_source2_nominal_field20652.clone(), Box::new(source_ml_source2_nominal_field20654.clone()), Box::new(source_ml_source2_nominal_field20656.clone()), Box::new(source_ml_source2_nominal_field20658.clone()), Box::new(source_ml_source2_nominal_field20660.clone()), source_ml_source2_nominal_field20662.clone(), Some((Some(state_3.clone()),)), source_ml_source2_nominal_field20666.clone(), Some((Some(call.clone()),)), Some((Some(source_ml_source2_nominal_override20676.clone()),)), source_ml_source2_nominal_field20672.clone(), source_ml_source2_nominal_field20674.clone())
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
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
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolPrimitiveGet(heap_4, state_4, receiver_2, key) => {
                                        let heap_4 = *heap_4;
                                        let state_4 = *state_4;
                                        let receiver_2 = *receiver_2;
                                        let key = *key;
                                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesPrimitiveGet(Box::new(heap_4.clone()), Box::new({
                                            let source_ml_source2_nominal_field20683 = shared_constructor_field4.clone();
                                            {
                                                let source_ml_source2_nominal_field20685 = shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_nominal_field20687 = shared_constructor_field6.clone();
                                                    {
                                                        let source_ml_source2_nominal_field20689 = shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_nominal_field20691 = shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_nominal_field20693 = shared_constructor_field9.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field20695 = shared_constructor_field10.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field20697 = shared_constructor_field11.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field20699 = shared_constructor_field12.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field20701 = shared_constructor_field13.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field20703 = shared_constructor_field14.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field20705 = shared_constructor_field15.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field20707 = shared_constructor_field16.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field20709 = shared_constructor_field17.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field20711 = shared_constructor_field18.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field20713 = shared_constructor_field19.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field20715 = shared_constructor_field20.clone();
                                                                                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field20683.clone()), Box::new(source_ml_source2_nominal_field20685.clone()), Box::new(source_ml_source2_nominal_field20687.clone()), Box::new(source_ml_source2_nominal_field20689.clone()), Box::new(source_ml_source2_nominal_field20691.clone()), source_ml_source2_nominal_field20693.clone(), Box::new(source_ml_source2_nominal_field20695.clone()), Box::new(source_ml_source2_nominal_field20697.clone()), Box::new(source_ml_source2_nominal_field20699.clone()), Box::new(source_ml_source2_nominal_field20701.clone()), source_ml_source2_nominal_field20703.clone(), Some((Some(state_4.clone()),)), source_ml_source2_nominal_field20707.clone(), source_ml_source2_nominal_field20709.clone(), source_ml_source2_nominal_field20711.clone(), source_ml_source2_nominal_field20713.clone(), source_ml_source2_nominal_field20715.clone())
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }), Box::new(receiver_2.clone()), Box::new(key.clone()))
                                    }
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolAcquired(heap_5, record) => {
                                        let heap_5 = *heap_5;
                                        let record = *record;
                                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesContinue(Box::new(heap_5.clone()), Box::new({
                                            let source_ml_source2_nominal_field20719 = shared_constructor_field4.clone();
                                            {
                                                let source_ml_source2_nominal_field20721 = shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_nominal_field20723 = shared_constructor_field6.clone();
                                                    {
                                                        let source_ml_source2_nominal_field20725 = shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_nominal_field20727 = shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_nominal_field20729 = shared_constructor_field9.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field20731 = shared_constructor_field10.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field20733 = shared_constructor_field11.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field20735 = shared_constructor_field12.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field20737 = shared_constructor_field13.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field20739 = shared_constructor_field14.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field20741 = shared_constructor_field15.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field20743 = shared_constructor_field16.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field20745 = shared_constructor_field17.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field20747 = shared_constructor_field18.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field20749 = shared_constructor_field19.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field20751 = shared_constructor_field20.clone();
                                                                                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field20719.clone()), Box::new(source_ml_source2_nominal_field20721.clone()), Box::new(source_ml_source2_nominal_field20723.clone()), Box::new(source_ml_source2_nominal_field20725.clone()), Box::new(source_ml_source2_nominal_field20727.clone()), self::ml_string::Text::from_units(vec![115, 116, 101, 112]), Box::new(source_ml_source2_nominal_field20731.clone()), Box::new(source_ml_source2_nominal_field20733.clone()), Box::new(source_ml_source2_nominal_field20735.clone()), Box::new(source_ml_source2_nominal_field20737.clone()), Some((Some(record.clone()),)), source_ml_source2_nominal_field20741.clone(), source_ml_source2_nominal_field20743.clone(), source_ml_source2_nominal_field20745.clone(), source_ml_source2_nominal_field20747.clone(), source_ml_source2_nominal_field20749.clone(), source_ml_source2_nominal_field20751.clone())
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }))
                                    }
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolStep(heap_6, record_2, done, value) => {
                                        let heap_6 = *heap_6;
                                        let record_2 = *record_2;
                                        let value = *value;
                                        if done {
                                            self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesCompleted(Box::new(heap_6.clone()), Box::new(match state.clone() {
                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, source_ml_source2_data_property_value20755, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value20755 = *source_ml_source2_data_property_value20755;
                                                    source_ml_source2_data_property_value20755.clone()
                                                }
                                            }))
                                        } else {
                                            {
                                                let next = {
                                                    let source_ml_source2_nominal_field20757 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field20759 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field20761 = shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field20763 = shared_constructor_field7.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field20765 = shared_constructor_field8.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field20767 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field20769 = shared_constructor_field10.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field20771 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field20773 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field20775 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field20777 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field20779 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field20781 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field20783 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field20785 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field20787 = shared_constructor_field19.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field20789 = shared_constructor_field20.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override20794 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override20795 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_override20796 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field20757.clone()), Box::new(source_ml_source2_nominal_field20759.clone()), Box::new(source_ml_source2_nominal_field20761.clone()), Box::new(source_ml_source2_nominal_field20763.clone()), Box::new(source_ml_source2_nominal_field20765.clone()), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 107, 101, 121]), Box::new(value.clone()), Box::new(source_ml_source2_nominal_override20794.clone()), Box::new(source_ml_source2_nominal_override20795.clone()), Box::new(source_ml_source2_nominal_override20796.clone()), Some((Some(record_2.clone()),)), source_ml_source2_nominal_field20779.clone(), source_ml_source2_nominal_field20781.clone(), source_ml_source2_nominal_field20783.clone(), source_ml_source2_nominal_field20785.clone(), source_ml_source2_nominal_field20787.clone(), source_ml_source2_nominal_field20789.clone())
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
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
                                                if match value.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        true
                                                    }
                                                    _ => {
                                                        false
                                                    }
                                                } {
                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesContinue(Box::new(heap_6.clone()), Box::new(next.clone()))
                                                } else {
                                                    self::fail_java_script_object_from_entries(heap_6.clone(), next.clone(), &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![73, 116, 101, 114, 97, 116, 111, 114, 32, 118, 97, 108, 117, 101, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 101, 110, 116, 114, 121, 32, 79, 98, 106, 101, 99, 116])))
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolClosed(heap_7, _, completion) => {
                                        let heap_7 = *heap_7;
                                        let completion = *completion;
                                        match completion.clone() {
                                            self::JavaScriptIteratorCompletion::IteratorCompletion(shared_constructor_field1_2, shared_constructor_field2_2, _) => {
                                                let shared_constructor_field2_2 = *shared_constructor_field2_2;
                                                if (shared_constructor_field1_2 == (self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119]))) {
                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesThrow(Box::new(heap_7.clone()), Box::new(shared_constructor_field2_2.clone()))
                                                } else {
                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(Box::new(heap_7.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 99, 108, 101, 97, 110, 117, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 97, 98, 114, 117, 112, 116, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolThrow(heap_8, transition_state, value_2) => {
                                        let heap_8 = *heap_8;
                                        let transition_state = *transition_state;
                                        let value_2 = *value_2;
                                        if (((match state.clone() {
                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, source_ml_source2_data_property_value20803, _, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value20803.clone()
                                            }
                                        }) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) && !(match (match state.clone() {
                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20804, _, _) => {
                                                ((source_ml_source2_data_property_value20804.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                            }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                            self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesThrow(Box::new(heap_8.clone()), Box::new((match state.clone() {
                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20805, _, _) => {
                                                    ((source_ml_source2_data_property_value20805.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value")))
                                        } else {
                                            self::throw_java_script_object_from_entries(heap_8.clone(), if ((match state.clone() {
                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, source_ml_source2_data_property_value20807, _, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value20807.clone()
                                                }
                                            }) == (self::ml_string::Text::from_units(vec![115, 116, 101, 112]))) {
                                                {
                                                    let source_ml_source2_nominal_field20810 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field20812 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field20814 = shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field20816 = shared_constructor_field7.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field20818 = shared_constructor_field8.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field20820 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field20822 = shared_constructor_field10.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field20824 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field20826 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field20828 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field20830 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field20832 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field20834 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field20836 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field20838 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field20840 = shared_constructor_field19.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field20842 = shared_constructor_field20.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override20844 = match transition_state.clone() {
                                                                                                                            self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, source_ml_source2_data_property_value20808, _, _, _, _) => {
                                                                                                                                let source_ml_source2_data_property_value20808 = *source_ml_source2_data_property_value20808;
                                                                                                                                source_ml_source2_data_property_value20808.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field20810.clone()), Box::new(source_ml_source2_nominal_field20812.clone()), Box::new(source_ml_source2_nominal_field20814.clone()), Box::new(source_ml_source2_nominal_field20816.clone()), Box::new(source_ml_source2_nominal_field20818.clone()), source_ml_source2_nominal_field20820.clone(), Box::new(source_ml_source2_nominal_field20822.clone()), Box::new(source_ml_source2_nominal_field20824.clone()), Box::new(source_ml_source2_nominal_field20826.clone()), Box::new(source_ml_source2_nominal_field20828.clone()), Some((Some(source_ml_source2_nominal_override20844.clone()),)), source_ml_source2_nominal_field20832.clone(), source_ml_source2_nominal_field20834.clone(), source_ml_source2_nominal_field20836.clone(), source_ml_source2_nominal_field20838.clone(), source_ml_source2_nominal_field20840.clone(), source_ml_source2_nominal_field20842.clone())
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
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
                                                base.clone()
                                            }, value_2.clone())
                                        }
                                    }
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolError(heap_9, transition_state_2, error_class, reason) => {
                                        let heap_9 = *heap_9;
                                        let transition_state_2 = *transition_state_2;
                                        if (((match state.clone() {
                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, source_ml_source2_data_property_value20846, _, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value20846.clone()
                                            }
                                        }) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) && !(match (match state.clone() {
                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20847, _, _) => {
                                                ((source_ml_source2_data_property_value20847.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                            }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                            self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesThrow(Box::new(heap_9.clone()), Box::new((match state.clone() {
                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20848, _, _) => {
                                                    ((source_ml_source2_data_property_value20848.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value")))
                                        } else {
                                            self::fail_java_script_object_from_entries(heap_9.clone(), if ((match state.clone() {
                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, source_ml_source2_data_property_value20850, _, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value20850.clone()
                                                }
                                            }) == (self::ml_string::Text::from_units(vec![115, 116, 101, 112]))) {
                                                {
                                                    let source_ml_source2_nominal_field20853 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field20855 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field20857 = shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field20859 = shared_constructor_field7.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field20861 = shared_constructor_field8.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field20863 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field20865 = shared_constructor_field10.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field20867 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field20869 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field20871 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field20873 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field20875 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field20877 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field20879 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field20881 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field20883 = shared_constructor_field19.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field20885 = shared_constructor_field20.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override20887 = match transition_state_2.clone() {
                                                                                                                            self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, source_ml_source2_data_property_value20851, _, _, _, _) => {
                                                                                                                                let source_ml_source2_data_property_value20851 = *source_ml_source2_data_property_value20851;
                                                                                                                                source_ml_source2_data_property_value20851.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field20853.clone()), Box::new(source_ml_source2_nominal_field20855.clone()), Box::new(source_ml_source2_nominal_field20857.clone()), Box::new(source_ml_source2_nominal_field20859.clone()), Box::new(source_ml_source2_nominal_field20861.clone()), source_ml_source2_nominal_field20863.clone(), Box::new(source_ml_source2_nominal_field20865.clone()), Box::new(source_ml_source2_nominal_field20867.clone()), Box::new(source_ml_source2_nominal_field20869.clone()), Box::new(source_ml_source2_nominal_field20871.clone()), Some((Some(source_ml_source2_nominal_override20887.clone()),)), source_ml_source2_nominal_field20875.clone(), source_ml_source2_nominal_field20877.clone(), source_ml_source2_nominal_field20879.clone(), source_ml_source2_nominal_field20881.clone(), source_ml_source2_nominal_field20883.clone(), source_ml_source2_nominal_field20885.clone())
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
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
                                                base.clone()
                                            }, &(error_class), &(reason))
                                        }
                                    }
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(heap_10, reason_2) => {
                                        let heap_10 = *heap_10;
                                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(Box::new(heap_10.clone()), reason_2.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
