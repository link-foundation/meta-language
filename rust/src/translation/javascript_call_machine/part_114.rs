// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptIndexedPropertyCreation;
use super::JavaScriptObjectEnumerationState;
use super::JavaScriptObjectEnumerationTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055;
use super::MlSource1TemporaryStepJavaScriptObjectEnumerationLoop1056Result;
use super::create_java_script_indexed_data_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_object_enumeration_state_error;
use super::read_java_script_ordinary_own_keys;
use super::read_java_script_semantic_text_length;
use super::source_ml_source1_temporary_step_java_script_object_enumeration_loop1056;
use super::step_java_script_object_enumeration_read;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_object_enumeration_value(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectEnumerationState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptObjectEnumerationTransition {
    match state.clone() {
        self::JavaScriptObjectEnumerationState::ObjectEnumeration(
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
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field12 = *shared_constructor_field12;
            if (shared_constructor_field1
                == (self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 101, 110, 116, 114, 105, 101, 115,
                ])))
            {
                self::JavaScriptObjectEnumerationTransition::ObjectEnumerationContinue(
                    Box::new(heap.clone()),
                    Box::new({
                        let source_ml_source2_nominal_field22815 =
                            shared_constructor_field1.clone();
                        {
                            let source_ml_source2_nominal_field22817 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_nominal_field22819 =
                                    shared_constructor_field3.clone();
                                {
                                    let source_ml_source2_nominal_field22821 =
                                        shared_constructor_field4.clone();
                                    {
                                        let source_ml_source2_nominal_field22823 =
                                            shared_constructor_field5.clone();
                                        {
                                            let source_ml_source2_nominal_field22825 =
                                                shared_constructor_field6;
                                            {
                                                let source_ml_source2_nominal_field22827 =
                                                    shared_constructor_field7.clone();
                                                {
                                                    let source_ml_source2_nominal_field22829 =
                                                        shared_constructor_field8.clone();
                                                    {
                                                        let source_ml_source2_nominal_field22831 =
                                                            shared_constructor_field9.clone();
                                                        {
                                                            let source_ml_source2_nominal_field22833 =
                                                                shared_constructor_field10.clone();
                                                            {
                                                                let source_ml_source2_nominal_field22835 =
                                                                    shared_constructor_field11;
                                                                {
                                                                    let source_ml_source2_nominal_field22837 =
                                                                        shared_constructor_field12
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field22839 = shared_constructor_field13;
                                                                        {
                                                                            let source_ml_source2_nominal_override22843 = self::JavaScriptSemanticValue::Undefined;
                                                                            self::JavaScriptObjectEnumerationState::ObjectEnumeration(source_ml_source2_nominal_field22815.clone(), Box::new(source_ml_source2_nominal_field22817.clone()), Box::new(source_ml_source2_nominal_field22819.clone()), source_ml_source2_nominal_field22821.clone(), source_ml_source2_nominal_field22823.clone(), source_ml_source2_nominal_field22825, self::ml_string::Text::from_units(vec![112, 97, 105, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]), Box::new(value.clone()), Box::new(source_ml_source2_nominal_override22843.clone()), Box::new(source_ml_source2_nominal_field22833.clone()), source_ml_source2_nominal_field22835, Box::new(source_ml_source2_nominal_field22837.clone()), source_ml_source2_nominal_field22839)
                                                                        }
                                                                    }
                                                                }
                                                            }
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
                self::JavaScriptObjectEnumerationTransition::ObjectEnumerationContinue(
                    Box::new(heap.clone()),
                    Box::new({
                        let source_ml_source2_nominal_field22849 =
                            shared_constructor_field1.clone();
                        {
                            let source_ml_source2_nominal_field22851 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_nominal_field22853 =
                                    shared_constructor_field3.clone();
                                {
                                    let source_ml_source2_nominal_field22855 =
                                        shared_constructor_field4.clone();
                                    {
                                        let source_ml_source2_nominal_field22857 =
                                            shared_constructor_field5.clone();
                                        {
                                            let source_ml_source2_nominal_field22859 =
                                                shared_constructor_field6;
                                            {
                                                let source_ml_source2_nominal_field22861 =
                                                    shared_constructor_field7.clone();
                                                {
                                                    let source_ml_source2_nominal_field22863 =
                                                        shared_constructor_field8.clone();
                                                    {
                                                        let source_ml_source2_nominal_field22865 =
                                                            shared_constructor_field9.clone();
                                                        {
                                                            let source_ml_source2_nominal_field22867 =
                                                                shared_constructor_field10.clone();
                                                            {
                                                                let source_ml_source2_nominal_field22869 =
                                                                    shared_constructor_field11;
                                                                {
                                                                    let source_ml_source2_nominal_field22871 =
                                                                        shared_constructor_field12
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field22873 = shared_constructor_field13;
                                                                        {
                                                                            let source_ml_source2_nominal_override22876 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                            let source_ml_source2_concat_value22845 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source2_concat_value22846 = std::rc::Rc::new(std::cell::RefCell::new(vec![value.clone()]));
                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), { let ml_reference_value1158 = source_ml_source2_concat_value22845.borrow().clone(); ml_reference_value1158 }), { let ml_reference_value1159 = source_ml_source2_concat_value22846.borrow().clone(); ml_reference_value1159 })
                                                                            }
                                                                        }));
                                                                            {
                                                                                let source_ml_source2_nominal_override22877 = (shared_constructor_field6 + 1f64);
                                                                                {
                                                                                    let source_ml_source2_nominal_override22878 = self::JavaScriptSemanticValue::Undefined;
                                                                                    {
                                                                                        let source_ml_source2_nominal_override22879 = self::JavaScriptSemanticValue::Undefined;
                                                                                        self::JavaScriptObjectEnumerationState::ObjectEnumeration(source_ml_source2_nominal_field22849.clone(), Box::new(source_ml_source2_nominal_field22851.clone()), Box::new(source_ml_source2_nominal_field22853.clone()), source_ml_source2_nominal_field22855.clone(), source_ml_source2_nominal_override22876.clone(), source_ml_source2_nominal_override22877, self::ml_string::Text::from_units(vec![114, 101, 97, 100]), Box::new(source_ml_source2_nominal_override22878.clone()), Box::new(source_ml_source2_nominal_override22879.clone()), Box::new(source_ml_source2_nominal_field22867.clone()), source_ml_source2_nominal_field22869, Box::new(source_ml_source2_nominal_field22871.clone()), source_ml_source2_nominal_field22873)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
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
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_object_enumeration(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectEnumerationState,
) -> self::JavaScriptObjectEnumerationTransition {
    match state.clone() {
        self::JavaScriptObjectEnumerationState::ObjectEnumeration(
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
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field12 = *shared_constructor_field12;
            {
                let error = self::read_java_script_object_enumeration_state_error(
                    heap.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if (shared_constructor_field7
                        == (self::ml_string::Text::from_units(vec![107, 101, 121, 115])))
                    {
                        {
                            let keys = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                self::JavaScriptSemanticValue,
                            >::new(
                            )));
                            {
                                let source_ml_source2_s22900 = if {
                                    let source_ml_source2_s22902 =
                                        shared_constructor_field2.clone();
                                    match source_ml_source2_s22902.clone() {
                                        self::JavaScriptSemanticValue::Object(_, _) => true,
                                        _ => false,
                                    }
                                } {
                                    {
                                        let own = self::read_java_script_ordinary_own_keys(
                                            heap.clone(),
                                            (((match shared_constructor_field2.clone() {
                                                self::JavaScriptSemanticValue::Object(
                                                    source_ml_source2_data_property_value22907,
                                                    _,
                                                ) => Some((
                                                    source_ml_source2_data_property_value22907
                                                        .clone(),
                                                )),
                                                _ => None::<(self::ml::Big,)>,
                                            })
                                            .clone()
                                            .expect("a proven present value"))
                                            .0)
                                                .clone(),
                                        );
                                        if (((({
                                            let ml_reference_value1160 = own.borrow().clone();
                                            ml_reference_value1160
                                        })
                                        .0)
                                            .0)
                                            != (self::ml_string::Text::from_units(vec![])))
                                        {
                                            self::MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055::MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055Return(Box::new(self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(Box::new(heap.clone()), (({ let ml_reference_value1161 = own.borrow().clone(); ml_reference_value1161 }).0).0)))
                                        } else {
                                            {
                                                let keys_2 = (({
                                                    let ml_reference_value1162 =
                                                        own.borrow().clone();
                                                    ml_reference_value1162
                                                })
                                                .0)
                                                    .1;
                                                self::MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055::MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055Next(keys_2.clone())
                                            }
                                        }
                                    }
                                } else {
                                    if ({
                                        let source_ml_source2_s22909 =
                                            shared_constructor_field2.clone();
                                        match source_ml_source2_s22909.clone() {
                                            self::JavaScriptSemanticValue::Text(_) => true,
                                            _ => false,
                                        }
                                    } || {
                                        let source_ml_source2_s22913 =
                                            shared_constructor_field2.clone();
                                        match source_ml_source2_s22913.clone() {
                                            self::JavaScriptSemanticValue::SourceText(_, _) => true,
                                            _ => false,
                                        }
                                    }) {
                                        {
                                            let length =
                                                self::read_java_script_semantic_text_length(
                                                    shared_constructor_field2.clone(),
                                                );
                                            if (length > (4096f64)) {
                                                self::MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055::MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055Return(Box::new(self::JavaScriptObjectEnumerationTransition::ObjectEnumerationResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 45, 107, 101, 121, 32, 115, 110, 97, 112, 115, 104, 111, 116, 32, 98, 117, 100, 103, 101, 116]))))
                                            } else {
                                                {
                                                    let at = 0f64;
                                                    {
                                                        let source_ml_source2_s22918 = self::source_ml_source1_temporary_step_java_script_object_enumeration_loop1056(keys.clone(), length, at);
                                                        match source_ml_source2_s22918.clone() {
                                                            self::MlSource1TemporaryStepJavaScriptObjectEnumerationLoop1056Result::MlSource1TemporaryStepJavaScriptObjectEnumerationLoop1056Done => {
                                                                self::MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055::MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055Next(keys.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055::MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055Next(keys.clone())
                                    }
                                };
                                match source_ml_source2_s22900.clone() {
                                    self::MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055::MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055Next(keys_3) => {
                                        if (({ let ml_reference_array1163 = keys_3.clone(); let ml_reference_projection1163 = ml_reference_array1163.borrow().len() as f64; ml_reference_projection1163 }) > (4096f64)) {
                                            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 45, 107, 101, 121, 32, 115, 110, 97, 112, 115, 104, 111, 116, 32, 98, 117, 100, 103, 101, 116]))
                                        } else {
                                            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationContinue(Box::new(heap.clone()), Box::new({
                                                let source_ml_source2_nominal_field22920 = shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source2_nominal_field22922 = shared_constructor_field2.clone();
                                                    {
                                                        let source_ml_source2_nominal_field22924 = shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field22926 = shared_constructor_field4.clone();
                                                            {
                                                                let source_ml_source2_nominal_field22928 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field22930 = shared_constructor_field6;
                                                                    {
                                                                        let source_ml_source2_nominal_field22932 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field22934 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field22936 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field22938 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field22940 = shared_constructor_field11;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field22942 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field22944 = shared_constructor_field13;
                                                                                                self::JavaScriptObjectEnumerationState::ObjectEnumeration(source_ml_source2_nominal_field22920.clone(), Box::new(source_ml_source2_nominal_field22922.clone()), Box::new(source_ml_source2_nominal_field22924.clone()), keys_3.clone(), source_ml_source2_nominal_field22928.clone(), source_ml_source2_nominal_field22930, self::ml_string::Text::from_units(vec![114, 101, 97, 100]), Box::new(source_ml_source2_nominal_field22934.clone()), Box::new(source_ml_source2_nominal_field22936.clone()), Box::new(source_ml_source2_nominal_field22938.clone()), source_ml_source2_nominal_field22940, Box::new(source_ml_source2_nominal_field22942.clone()), source_ml_source2_nominal_field22944)
                                                                                            }
                                                                                        }
                                                                                    }
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
                                    }
                                    self::MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055::MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055Return(source_ml_source1_temporary_result1055) => {
                                        let source_ml_source1_temporary_result1055 = *source_ml_source1_temporary_result1055;
                                        source_ml_source1_temporary_result1055.clone()
                                    }
                                }
                            }
                        }
                    } else {
                        if ((shared_constructor_field7
                            == (self::ml_string::Text::from_units(vec![
                                97, 108, 108, 111, 99, 97, 116, 101,
                            ])))
                            || (shared_constructor_field7
                                == (self::ml_string::Text::from_units(vec![
                                    112, 97, 105, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105,
                                    111, 110,
                                ]))))
                        {
                            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationAllocate(
                                Box::new(heap.clone()),
                                Box::new(state.clone()),
                                Box::new(shared_constructor_field3.clone()),
                                if (shared_constructor_field7
                                    == (self::ml_string::Text::from_units(vec![
                                        97, 108, 108, 111, 99, 97, 116, 101,
                                    ])))
                                {
                                    {
                                        let ml_reference_array1164 =
                                            shared_constructor_field5.clone();
                                        let ml_reference_projection1164 =
                                            ml_reference_array1164.borrow().len() as f64;
                                        ml_reference_projection1164
                                    }
                                } else {
                                    2f64
                                },
                            )
                        } else {
                            if (shared_constructor_field7
                                == (self::ml_string::Text::from_units(vec![
                                    103, 101, 116, 116, 101, 114,
                                ])))
                            {
                                self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 103, 101, 116, 116, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                            } else {
                                if ((shared_constructor_field7
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 97, 105, 114, 32, 119, 114, 105, 116, 101,
                                    ])))
                                    || (shared_constructor_field7
                                        == (self::ml_string::Text::from_units(vec![
                                            111, 117, 116, 112, 117, 116, 32, 119, 114, 105, 116,
                                            101,
                                        ]))))
                                {
                                    if ((shared_constructor_field7
                                        == (self::ml_string::Text::from_units(vec![
                                            112, 97, 105, 114, 32, 119, 114, 105, 116, 101,
                                        ])))
                                        && (shared_constructor_field11 == (2f64)))
                                    {
                                        self::JavaScriptObjectEnumerationTransition::ObjectEnumerationContinue(Box::new(heap.clone()), Box::new({
                                            let source_ml_source2_nominal_field22972 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field22974 = shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source2_nominal_field22976 = shared_constructor_field3.clone();
                                                    {
                                                        let source_ml_source2_nominal_field22978 = shared_constructor_field4.clone();
                                                        {
                                                            let source_ml_source2_nominal_field22980 = shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field22982 = shared_constructor_field6;
                                                                {
                                                                    let source_ml_source2_nominal_field22984 = shared_constructor_field7.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field22986 = shared_constructor_field8.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field22988 = shared_constructor_field9.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field22990 = shared_constructor_field10.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field22992 = shared_constructor_field11;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field22994 = shared_constructor_field12.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field22996 = shared_constructor_field13;
                                                                                            {
                                                                                                let source_ml_source2_nominal_override22999 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                    let source_ml_source2_concat_value22968 = shared_constructor_field5.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_concat_value22969 = std::rc::Rc::new(std::cell::RefCell::new(vec![shared_constructor_field10.clone()]));
                                                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), { let ml_reference_value1165 = source_ml_source2_concat_value22968.borrow().clone(); ml_reference_value1165 }), { let ml_reference_value1166 = source_ml_source2_concat_value22969.borrow().clone(); ml_reference_value1166 })
                                                                                                    }
                                                                                                }));
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override23000 = (shared_constructor_field6 + 1f64);
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override23001 = self::JavaScriptSemanticValue::Undefined;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override23002 = 0f64;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override23003 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                self::JavaScriptObjectEnumerationState::ObjectEnumeration(source_ml_source2_nominal_field22972.clone(), Box::new(source_ml_source2_nominal_field22974.clone()), Box::new(source_ml_source2_nominal_field22976.clone()), source_ml_source2_nominal_field22978.clone(), source_ml_source2_nominal_override22999.clone(), source_ml_source2_nominal_override23000, self::ml_string::Text::from_units(vec![114, 101, 97, 100]), Box::new(source_ml_source2_nominal_override23003.clone()), Box::new(source_ml_source2_nominal_field22988.clone()), Box::new(source_ml_source2_nominal_override23001.clone()), source_ml_source2_nominal_override23002, Box::new(source_ml_source2_nominal_field22994.clone()), source_ml_source2_nominal_field22996)
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
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
                                        if ((shared_constructor_field7
                                            == (self::ml_string::Text::from_units(vec![
                                                111, 117, 116, 112, 117, 116, 32, 119, 114, 105,
                                                116, 101,
                                            ])))
                                            && (shared_constructor_field13
                                                == ({
                                                    let ml_reference_array1167 =
                                                        shared_constructor_field5.clone();
                                                    let ml_reference_projection1167 =
                                                        ml_reference_array1167.borrow().len()
                                                            as f64;
                                                    ml_reference_projection1167
                                                })))
                                        {
                                            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationCompleted(Box::new(heap.clone()), Box::new(shared_constructor_field12.clone()))
                                        } else {
                                            {
                                                let index = if (shared_constructor_field7
                                                    == (self::ml_string::Text::from_units(vec![
                                                        112, 97, 105, 114, 32, 119, 114, 105, 116,
                                                        101,
                                                    ]))) {
                                                    shared_constructor_field11
                                                } else {
                                                    shared_constructor_field13
                                                };
                                                {
                                                    let target = if (shared_constructor_field7
                                                        == (self::ml_string::Text::from_units(
                                                            vec![
                                                                112, 97, 105, 114, 32, 119, 114,
                                                                105, 116, 101,
                                                            ],
                                                        ))) {
                                                        shared_constructor_field10.clone()
                                                    } else {
                                                        shared_constructor_field12.clone()
                                                    };
                                                    {
                                                        let value = if (shared_constructor_field7
                                                            == (self::ml_string::Text::from_units(
                                                                vec![
                                                                    112, 97, 105, 114, 32, 119,
                                                                    114, 105, 116, 101,
                                                                ],
                                                            ))) {
                                                            if (index == (0f64)) {
                                                                ({
                                                                    let source_ml_source2_array_read_value23023 = shared_constructor_field4.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index23024 = shared_constructor_field6;
                                                                        { let ml_reference_array1168 = source_ml_source2_array_read_value23023.clone(); let ml_reference_position1168 = source_ml_source2_array_read_index23024; let ml_reference_projection1168 = self::ml_array::optional_at(&*ml_reference_array1168.borrow(), self::ml_array::number_index(ml_reference_position1168)); ml_reference_projection1168 }
                                                                    }
                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)
                                                            } else {
                                                                shared_constructor_field8.clone()
                                                            }
                                                        } else {
                                                            ({
                                                                let source_ml_source2_array_read_value23027 = shared_constructor_field5.clone();
                                                                { let ml_reference_array1169 = source_ml_source2_array_read_value23027.clone(); let ml_reference_position1169 = index; let ml_reference_projection1169 = self::ml_array::optional_at(&*ml_reference_array1169.borrow(), self::ml_array::number_index(ml_reference_position1169)); ml_reference_projection1169 }
                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)
                                                        };
                                                        {
                                                            let written = self::create_java_script_indexed_data_property(heap.clone(), target.clone(), index, value.clone());
                                                            match written.clone() {
                                                                self::JavaScriptIndexedPropertyCreation::IndexedPropertyResourceLimit(heap_2, reason) => {
                                                                    let heap_2 = *heap_2;
                                                                    self::JavaScriptObjectEnumerationTransition::ObjectEnumerationResourceLimit(Box::new(heap_2.clone()), Box::new(state.clone()), reason.clone())
                                                                }
                                                                _ => {
                                                                    match written.clone() {
                                                                        self::JavaScriptIndexedPropertyCreation::IndexedPropertyCreated(heap_3) => {
                                                                            let heap_3 = *heap_3;
                                                                            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationContinue(Box::new(heap_3.clone()), Box::new(if (shared_constructor_field7 == (self::ml_string::Text::from_units(vec![112, 97, 105, 114, 32, 119, 114, 105, 116, 101]))) {
                                                                                {
                                                                                    let source_ml_source2_nominal_field23033 = shared_constructor_field1.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field23035 = shared_constructor_field2.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field23037 = shared_constructor_field3.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field23039 = shared_constructor_field4.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field23041 = shared_constructor_field5.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field23043 = shared_constructor_field6;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field23045 = shared_constructor_field7.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field23047 = shared_constructor_field8.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field23049 = shared_constructor_field9.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field23051 = shared_constructor_field10.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field23053 = shared_constructor_field11;
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field23055 = shared_constructor_field12.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field23057 = shared_constructor_field13;
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_override23059 = (shared_constructor_field11 + 1f64);
                                                                                                                                        self::JavaScriptObjectEnumerationState::ObjectEnumeration(source_ml_source2_nominal_field23033.clone(), Box::new(source_ml_source2_nominal_field23035.clone()), Box::new(source_ml_source2_nominal_field23037.clone()), source_ml_source2_nominal_field23039.clone(), source_ml_source2_nominal_field23041.clone(), source_ml_source2_nominal_field23043, source_ml_source2_nominal_field23045.clone(), Box::new(source_ml_source2_nominal_field23047.clone()), Box::new(source_ml_source2_nominal_field23049.clone()), Box::new(source_ml_source2_nominal_field23051.clone()), source_ml_source2_nominal_override23059, Box::new(source_ml_source2_nominal_field23055.clone()), source_ml_source2_nominal_field23057)
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
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
                                                                                    let source_ml_source2_nominal_field23062 = shared_constructor_field1.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field23064 = shared_constructor_field2.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field23066 = shared_constructor_field3.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field23068 = shared_constructor_field4.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field23070 = shared_constructor_field5.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field23072 = shared_constructor_field6;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field23074 = shared_constructor_field7.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field23076 = shared_constructor_field8.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field23078 = shared_constructor_field9.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field23080 = shared_constructor_field10.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field23082 = shared_constructor_field11;
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field23084 = shared_constructor_field12.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field23086 = shared_constructor_field13;
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_override23088 = (shared_constructor_field13 + 1f64);
                                                                                                                                        self::JavaScriptObjectEnumerationState::ObjectEnumeration(source_ml_source2_nominal_field23062.clone(), Box::new(source_ml_source2_nominal_field23064.clone()), Box::new(source_ml_source2_nominal_field23066.clone()), source_ml_source2_nominal_field23068.clone(), source_ml_source2_nominal_field23070.clone(), source_ml_source2_nominal_field23072, source_ml_source2_nominal_field23074.clone(), Box::new(source_ml_source2_nominal_field23076.clone()), Box::new(source_ml_source2_nominal_field23078.clone()), Box::new(source_ml_source2_nominal_field23080.clone()), source_ml_source2_nominal_field23082, Box::new(source_ml_source2_nominal_field23084.clone()), source_ml_source2_nominal_override23088)
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
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
                                                                        self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(heap_4, reason_2) => {
                                                                            let heap_4 = *heap_4;
                                                                            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(Box::new(heap_4.clone()), reason_2.clone())
                                                                        }
                                                                        self::JavaScriptIndexedPropertyCreation::IndexedPropertyInvalid(heap_5, reason_3) => {
                                                                            let heap_5 = *heap_5;
                                                                            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(Box::new(heap_5.clone()), reason_3.clone())
                                                                        }
                                                                        self::JavaScriptIndexedPropertyCreation::IndexedPropertyResourceLimit(heap_6, reason_4) => {
                                                                            let heap_6 = *heap_6;
                                                                            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(Box::new(heap_6.clone()), reason_4.clone())
                                                                        }
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
                                    self::step_java_script_object_enumeration_read(
                                        heap.clone(),
                                        state.clone(),
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
