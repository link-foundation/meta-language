// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptArrayConcatenationState;
use super::JavaScriptArrayConcatenationTransition;
use super::JavaScriptArraySliceCall;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSemanticValue;
use super::allocate_java_script_array;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_array_concatenation_state_error;
use super::read_java_script_ordinary_value_error;
use super::request_java_script_array_concatenation_allocation;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_array_concatenation_allocation(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArrayConcatenationState,
    identity: self::ml::Big,
) -> self::JavaScriptArrayConcatenationTransition {
    match state.clone() {
        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
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
            shared_constructor_field18,
            shared_constructor_field19,
            shared_constructor_field20,
            shared_constructor_field21,
            shared_constructor_field22,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field12 = *shared_constructor_field12;
            let shared_constructor_field13 = *shared_constructor_field13;
            let shared_constructor_field14 = *shared_constructor_field14;
            {
                let error = self::read_java_script_array_concatenation_state_error(
                    heap.clone(),
                    directory.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if ((shared_constructor_field15 != (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]))) || match (match state.clone() {
                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10691, _, _, _, _) => {
                            ((source_ml_source2_data_property_value10691.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                        }
                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                        self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 114, 101, 113, 117, 101, 115, 116]))
                    } else {
                        {
                            let preflight = self::request_java_script_array_concatenation_allocation(heap.clone(), state.clone());
                            match preflight.clone() {
                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationAllocate(_, _, _, _) => {
                                    {
                                        let result = self::allocate_java_script_array(heap.clone(), identity.clone(), (match state.clone() {
                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10696, _, _, _, _) => {
                                                ((source_ml_source2_data_property_value10696.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value"), 0f64);
                                        match result.clone() {
                                            self::JavaScriptArrayCompletion::ArrayNormal(heap_2, value) => {
                                                let heap_2 = *heap_2;
                                                let value = *value;
                                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(Box::new(heap_2.clone()), Box::new({
                                                    let source_ml_source2_nominal_field10705 = shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_nominal_field10707 = shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_nominal_field10709 = shared_constructor_field3.clone();
                                                            {
                                                                let source_ml_source2_nominal_field10711 = shared_constructor_field4.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field10713 = shared_constructor_field5;
                                                                    {
                                                                        let source_ml_source2_nominal_field10715 = shared_constructor_field6.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field10717 = shared_constructor_field7;
                                                                            {
                                                                                let source_ml_source2_nominal_field10719 = shared_constructor_field8;
                                                                                {
                                                                                    let source_ml_source2_nominal_field10721 = shared_constructor_field9;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field10723 = shared_constructor_field10;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field10725 = shared_constructor_field11;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field10727 = shared_constructor_field12.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field10729 = shared_constructor_field13.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field10731 = shared_constructor_field14.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field10733 = shared_constructor_field15.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field10735 = shared_constructor_field16.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field10737 = shared_constructor_field17.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field10739 = shared_constructor_field18.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field10741 = shared_constructor_field19.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field10743 = shared_constructor_field20.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field10745 = shared_constructor_field21.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field10747 = shared_constructor_field22.clone();
                                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field10705.clone()), source_ml_source2_nominal_field10707.clone(), Box::new(source_ml_source2_nominal_field10709.clone()), Box::new(source_ml_source2_nominal_field10711.clone()), source_ml_source2_nominal_field10713, Box::new(source_ml_source2_nominal_field10715.clone()), source_ml_source2_nominal_field10717, source_ml_source2_nominal_field10719, source_ml_source2_nominal_field10721, source_ml_source2_nominal_field10723, source_ml_source2_nominal_field10725, Box::new(value.clone()), Box::new(source_ml_source2_nominal_field10729.clone()), Box::new(source_ml_source2_nominal_field10731.clone()), self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116]), source_ml_source2_nominal_field10735.clone(), source_ml_source2_nominal_field10737.clone(), source_ml_source2_nominal_field10739.clone(), source_ml_source2_nominal_field10741.clone(), source_ml_source2_nominal_field10743.clone(), source_ml_source2_nominal_field10745.clone(), source_ml_source2_nominal_field10747.clone())
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
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
                                            self::JavaScriptArrayCompletion::ArrayPropertyComplete(heap_3, _, _) => {
                                                let heap_3 = *heap_3;
                                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap_3.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 97, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                            }
                                            self::JavaScriptArrayCompletion::ArrayNumberRequest(heap_4, _, _) => {
                                                let heap_4 = *heap_4;
                                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap_4.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 97, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                            }
                                            self::JavaScriptArrayCompletion::ArrayDefinitionContinue(heap_5, _) => {
                                                let heap_5 = *heap_5;
                                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap_5.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 97, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                            }
                                            self::JavaScriptArrayCompletion::ArrayInvalid(heap_6, source_ml_source2_f10702) => {
                                                let heap_6 = *heap_6;
                                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap_6.clone()), source_ml_source2_f10702.clone())
                                            }
                                            self::JavaScriptArrayCompletion::ArrayError(heap_7, error_class, reason) => {
                                                let heap_7 = *heap_7;
                                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationError(Box::new(heap_7.clone()), error_class.clone(), reason.clone())
                                            }
                                            self::JavaScriptArrayCompletion::ArrayResourceLimit(heap_8, reason_2, _) => {
                                                let heap_8 = *heap_8;
                                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationResourceLimit(Box::new(heap_8.clone()), Box::new(state.clone()), reason_2.clone())
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    preflight.clone()
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
pub fn resume_java_script_array_concatenation_construction(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArrayConcatenationState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptArrayConcatenationTransition {
    match state.clone() {
        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
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
            shared_constructor_field18,
            shared_constructor_field19,
            shared_constructor_field20,
            shared_constructor_field21,
            shared_constructor_field22,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field12 = *shared_constructor_field12;
            let shared_constructor_field13 = *shared_constructor_field13;
            let shared_constructor_field14 = *shared_constructor_field14;
            {
                let error = self::read_java_script_array_concatenation_state_error(
                    heap.clone(),
                    directory.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if ((shared_constructor_field15
                        != (self::ml_string::Text::from_units(vec![
                            99, 111, 110, 115, 116, 114, 117, 99, 116,
                        ])))
                        || !(self::ml_array::includes(
                            &{
                                let ml_reference_value733 =
                                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![
                                            110, 111, 114, 109, 97, 108, 67, 111, 109, 112, 108,
                                            101, 116, 105, 111, 110,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            116, 104, 114, 111, 119, 67, 111, 109, 112, 108, 101,
                                            116, 105, 111, 110,
                                        ]),
                                    ])))
                                    .borrow()
                                    .clone();
                                ml_reference_value733
                            },
                            &match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(
                                    _,
                                ) => self::ml_string::Text::from_units(vec![
                                    110, 111, 114, 109, 97, 108, 67, 111, 109, 112, 108, 101, 116,
                                    105, 111, 110,
                                ]),
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(
                                    _,
                                ) => self::ml_string::Text::from_units(vec![
                                    116, 104, 114, 111, 119, 67, 111, 109, 112, 108, 101, 116, 105,
                                    111, 110,
                                ]),
                            },
                            0f64,
                            false,
                            |a, b| a == b,
                        )))
                    {
                        self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32,
                                99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114,
                                101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 114, 101,
                                113, 117, 101, 115, 116, 32, 97, 110, 100, 32, 99, 111, 109, 112,
                                108, 101, 116, 105, 111, 110, 32, 107, 105, 110, 100,
                            ]),
                        )
                    } else {
                        {
                            let invalid = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source2_data_property_value10757) => {
                                    let source_ml_source2_data_property_value10757 = *source_ml_source2_data_property_value10757;
                                    source_ml_source2_data_property_value10757.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source2_data_property_value10758) => {
                                    let source_ml_source2_data_property_value10758 = *source_ml_source2_data_property_value10758;
                                    source_ml_source2_data_property_value10758.clone()
                                }
                            });
                            if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap.clone()), invalid.clone())
                            } else {
                                match completion.clone() {
                                    self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                                        let value = *value;
                                        if match value.clone() {
                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                true
                                            }
                                            _ => {
                                                false
                                            }
                                        } {
                                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(Box::new(heap.clone()), Box::new({
                                                let source_ml_source2_nominal_field10762 = shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source2_nominal_field10764 = shared_constructor_field2.clone();
                                                    {
                                                        let source_ml_source2_nominal_field10766 = shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field10768 = shared_constructor_field4.clone();
                                                            {
                                                                let source_ml_source2_nominal_field10770 = shared_constructor_field5;
                                                                {
                                                                    let source_ml_source2_nominal_field10772 = shared_constructor_field6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field10774 = shared_constructor_field7;
                                                                        {
                                                                            let source_ml_source2_nominal_field10776 = shared_constructor_field8;
                                                                            {
                                                                                let source_ml_source2_nominal_field10778 = shared_constructor_field9;
                                                                                {
                                                                                    let source_ml_source2_nominal_field10780 = shared_constructor_field10;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field10782 = shared_constructor_field11;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field10784 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field10786 = shared_constructor_field13.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field10788 = shared_constructor_field14.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field10790 = shared_constructor_field15.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field10792 = shared_constructor_field16.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field10794 = shared_constructor_field17.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field10796 = shared_constructor_field18.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field10798 = shared_constructor_field19.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field10800 = shared_constructor_field20.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field10802 = shared_constructor_field21.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field10804 = shared_constructor_field22.clone();
                                                                                                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field10762.clone()), source_ml_source2_nominal_field10764.clone(), Box::new(source_ml_source2_nominal_field10766.clone()), Box::new(source_ml_source2_nominal_field10768.clone()), source_ml_source2_nominal_field10770, Box::new(source_ml_source2_nominal_field10772.clone()), source_ml_source2_nominal_field10774, source_ml_source2_nominal_field10776, source_ml_source2_nominal_field10778, source_ml_source2_nominal_field10780, source_ml_source2_nominal_field10782, Box::new(value.clone()), Box::new(source_ml_source2_nominal_field10786.clone()), Box::new(source_ml_source2_nominal_field10788.clone()), self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116]), source_ml_source2_nominal_field10792.clone(), source_ml_source2_nominal_field10794.clone(), source_ml_source2_nominal_field10796.clone(), source_ml_source2_nominal_field10798.clone(), source_ml_source2_nominal_field10800.clone(), source_ml_source2_nominal_field10802.clone(), source_ml_source2_nominal_field10804.clone())
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
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
                                        } else {
                                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 67, 111, 110, 115, 116, 114, 117, 99, 116, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 115, 117, 108, 116]))
                                        }
                                    }
                                    self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value_2) => {
                                        let value_2 = *value_2;
                                        self::JavaScriptArrayConcatenationTransition::ArrayConcatenationThrow(Box::new(heap.clone()), Box::new(value_2.clone()))
                                    }
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
pub fn read_java_script_array_slice_relative_index(value: f64, length: f64) -> f64 {
    {
        let integer = if (value.is_nan() || (value == (0f64))) {
            0f64
        } else {
            value.trunc()
        };
        if (integer < (0f64)) {
            self::ml_math::max((length + integer), 0f64)
        } else {
            self::ml_math::min(integer, length)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn same_call(
    actual: self::JavaScriptArraySliceCall,
    callee: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> bool {
    match actual.clone() {
        self::JavaScriptArraySliceCall::ArraySliceCall(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            (((self::test_java_script_semantic_same_value(
                shared_constructor_field1.clone(),
                callee.clone(),
            ) && self::test_java_script_semantic_same_value(
                shared_constructor_field2.clone(),
                receiver.clone(),
            )) && (({
                let ml_reference_array734 = shared_constructor_field3.clone();
                let ml_reference_projection734 = ml_reference_array734.borrow().len() as f64;
                ml_reference_projection734
            }) == ({
                let ml_reference_array735 = arguments_.clone();
                let ml_reference_projection735 = ml_reference_array735.borrow().len() as f64;
                ml_reference_projection735
            }))) && {
                let ml_values736 = {
                    let ml_reference_value737 = shared_constructor_field3.borrow().clone();
                    ml_reference_value737
                };
                let ml_callback736 = {
                    let arguments_ = arguments_.clone();
                    std::rc::Rc::new(
                        move |value: self::JavaScriptSemanticValue, index: f64| -> bool {
                            self::test_java_script_semantic_same_value(
                                value.clone(),
                                ({
                                    let source_ml_source2_array_read_value10813 =
                                        arguments_.clone();
                                    {
                                        let source_ml_source2_array_read_index10814 = index;
                                        {
                                            let ml_reference_array738 =
                                                source_ml_source2_array_read_value10813.clone();
                                            let ml_reference_position738 =
                                                source_ml_source2_array_read_index10814;
                                            let ml_reference_projection738 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array738.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position738,
                                                    ),
                                                );
                                            ml_reference_projection738
                                        }
                                    }
                                })
                                .clone()
                                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
                            )
                        },
                    )
                };
                ml_values736
                    .iter()
                    .cloned()
                    .enumerate()
                    .all(|(ml_index736, ml_value736)| {
                        ml_callback736(ml_value736, (ml_index736 as f64))
                    })
            })
        }
    }
}
