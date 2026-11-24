// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCallbackTransition;
use super::JavaScriptArraySearchTransition;
use super::JavaScriptArraySpreadTransition;
use super::JavaScriptCallArgumentList;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Result;
use super::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1218Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source1_temporary_read_java_script_call_argument_lists_error_loop1218;
use super::test_java_script_iterator_region_slot;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_argument_lists_error_loop1217(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut depth: f64,
    mut previous: f64,
    mut count: f64,
    mut source_ml_source1_temporary_values1215: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>,
    >,
    mut source_ml_source1_temporary_index1215: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Result {
    loop {
        return if (source_ml_source1_temporary_index1215
            < ({
                let ml_reference_array3032 = source_ml_source1_temporary_values1215.clone();
                let ml_reference_projection3032 =
                    self::ml::Big::from_u128(ml_reference_array3032.borrow().len() as u128);
                ml_reference_projection3032
            })) {
            {
                let list = {
                    let ml_reference_array3033 = source_ml_source1_temporary_values1215.clone();
                    let ml_reference_position3033 = source_ml_source1_temporary_index1215.clone();
                    let ml_reference_projection3033 = self::ml_array::at(
                        &*ml_reference_array3033.borrow(),
                        ml_reference_position3033.to_index(),
                    );
                    ml_reference_projection3033
                };
                match list.clone() {
                    self::JavaScriptCallArgumentList::CallArgumentList(
                        shared_constructor_field1,
                        shared_constructor_field2,
                        shared_constructor_field3,
                    ) => {
                        if ((((((match list.clone() {
                            self::JavaScriptCallArgumentList::CallArgumentList(_, _, _) => false,
                        } || !(self::test_java_script_iterator_region_slot(
                            shared_constructor_field1,
                        ))) || (shared_constructor_field1 <= previous))
                            || !(self::ml_math::is_safe_integer(shared_constructor_field2)))
                            || {
                                let source_ml_source2_same_value_operand41828 =
                                    shared_constructor_field2;
                                {
                                    let source_ml_source2_same_value_operand41829 = (-0f64);
                                    (((source_ml_source2_same_value_operand41828
                                        == source_ml_source2_same_value_operand41829)
                                        && ((source_ml_source2_same_value_operand41828 != (0f64))
                                            || ((1f64
                                                / source_ml_source2_same_value_operand41828)
                                                == (1f64
                                                    / source_ml_source2_same_value_operand41829))))
                                        || ((source_ml_source2_same_value_operand41828
                                            != source_ml_source2_same_value_operand41828)
                                            && (source_ml_source2_same_value_operand41829
                                                != source_ml_source2_same_value_operand41829)))
                                }
                            })
                            || (shared_constructor_field2 < (0f64)))
                            || (shared_constructor_field2 > depth))
                        {
                            self::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Result::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Return(self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105, 115, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 115, 108, 111, 116, 115, 32, 97, 110, 100, 32, 97, 99, 116, 117, 97, 108, 32, 101, 110, 99, 108, 111, 115, 105, 110, 103, 32, 104, 97, 110, 100, 108, 101, 114, 32, 100, 101, 112, 116, 104, 115]))
                        } else {
                            {
                                let previous_2 = shared_constructor_field1;
                                {
                                    let count_2 = (count + {
                                        let ml_reference_array3034 =
                                            shared_constructor_field3.clone();
                                        let ml_reference_projection3034 =
                                            ml_reference_array3034.borrow().len() as f64;
                                        ml_reference_projection3034
                                    });
                                    if (count_2 > (1048576f64)) {
                                        self::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Result::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Return(self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105, 115, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 102, 114, 97, 109, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 98, 117, 100, 103, 101, 116]))
                                    } else {
                                        {
                                            let source_ml_source1_temporary_values1216 =
                                                shared_constructor_field3.clone();
                                            {
                                                let source_ml_source1_temporary_index1216 =
                                                    self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s41837 = self::source_ml_source1_temporary_read_java_script_call_argument_lists_error_loop1218(heap.clone(), source_ml_source1_temporary_values1216.clone(), source_ml_source1_temporary_index1216.clone());
                                                    match source_ml_source2_s41837.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1218Result::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1218Done => {
                                                            {
                                                                let source_ml_source1_temporary_index1215_2 = source_ml_source1_temporary_index1215.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    (previous, count, source_ml_source1_temporary_index1215) = (previous_2, count_2, source_ml_source1_temporary_index1215_2.clone());
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1218Result::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1218Return(source_ml_source1_temporary_result1218) => {
                                                            self::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Result::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Return(source_ml_source1_temporary_result1218.clone())
                                                        }
                                                    }
                                                }
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
            self::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Result::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_open_java_script_call_argument_list_loop1220(
    mut lists: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>,
    mut slot: f64,
    mut at: f64,
) -> f64 {
    loop {
        return if ((at
            < ({
                let ml_reference_array3035 = lists.clone();
                let ml_reference_projection3035 = ml_reference_array3035.borrow().len() as f64;
                ml_reference_projection3035
            }))
            && ((({
                let source_ml_source2_optional_chain_receiver41844 = ({
                    let ml_reference_array3036 = lists.clone();
                    let ml_reference_position3036 = at;
                    let ml_reference_projection3036 = self::ml_array::optional_at(
                        &*ml_reference_array3036.borrow(),
                        self::ml_array::number_index(ml_reference_position3036),
                    );
                    ml_reference_projection3036
                },);
                if match (source_ml_source2_optional_chain_receiver41844.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    Some({
                        let source_ml_source1_temporary_optional_chain1219 =
                            ((source_ml_source2_optional_chain_receiver41844.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        match source_ml_source1_temporary_optional_chain1219.clone() {
                            self::JavaScriptCallArgumentList::CallArgumentList(
                                source_ml_source2_data_property_value41845,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41845,
                        }
                    })
                } else {
                    None::<f64>
                }
            })
            .clone()
            .unwrap_or_else(|| slot))
                < slot))
        {
            {
                let at_2 = (at + 1f64);
                {
                    at = at_2;
                    continue;
                }
            }
        } else {
            at
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_append_java_script_call_argument_loop1222(
    mut count: f64,
    mut source_ml_source1_temporary_values1221: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>,
    >,
    mut source_ml_source1_temporary_index1221: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index1221
            < ({
                let ml_reference_array3037 = source_ml_source1_temporary_values1221.clone();
                let ml_reference_projection3037 =
                    self::ml::Big::from_u128(ml_reference_array3037.borrow().len() as u128);
                ml_reference_projection3037
            })) {
            {
                let row = {
                    let ml_reference_array3038 = source_ml_source1_temporary_values1221.clone();
                    let ml_reference_position3038 = source_ml_source1_temporary_index1221.clone();
                    let ml_reference_projection3038 = self::ml_array::at(
                        &*ml_reference_array3038.borrow(),
                        ml_reference_position3038.to_index(),
                    );
                    ml_reference_projection3038
                };
                {
                    let count_2 = (count + {
                        let ml_reference_array3039 = match row.clone() {
                            self::JavaScriptCallArgumentList::CallArgumentList(
                                _,
                                _,
                                source_ml_source2_data_property_value41848,
                            ) => source_ml_source2_data_property_value41848.clone(),
                        };
                        let ml_reference_projection3039 =
                            ml_reference_array3039.borrow().len() as f64;
                        ml_reference_projection3039
                    });
                    {
                        let source_ml_source1_temporary_index1221_2 =
                            source_ml_source1_temporary_index1221.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_source1_temporary_index1221) =
                                (count_2, source_ml_source1_temporary_index1221_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            count
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227 {
    MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227Next,
    MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1226 {
    MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1226Next,
    MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1226Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySpreadStateErrorLoop1225Result {
    MlSource1TemporaryReadJavaScriptArraySpreadStateErrorLoop1225Done,
    MlSource1TemporaryReadJavaScriptArraySpreadStateErrorLoop1225Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_spread_state_error_loop1225(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1223: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1223: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorLoop1225Result {
    loop {
        return if (source_ml_source1_temporary_index1223
            < ({
                let ml_reference_array3040 = source_ml_source1_temporary_values1223.clone();
                let ml_reference_projection3040 =
                    self::ml::Big::from_u128(ml_reference_array3040.borrow().len() as u128);
                ml_reference_projection3040
            })) {
            {
                let value = {
                    let ml_reference_array3041 = source_ml_source1_temporary_values1223.clone();
                    let ml_reference_position3041 = source_ml_source1_temporary_index1223.clone();
                    let ml_reference_projection3041 = self::ml_array::at(
                        &*ml_reference_array3041.borrow(),
                        ml_reference_position3041.to_index(),
                    );
                    ml_reference_projection3041
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorLoop1225Result::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorLoop1225Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1223_2 =
                                source_ml_source1_temporary_index1223
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1223 =
                                    source_ml_source1_temporary_index1223_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorLoop1225Result::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorLoop1225Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryBeginJavaScriptArraySpreadJoin1229 {
    MlSource1TemporaryBeginJavaScriptArraySpreadJoin1229Next,
    MlSource1TemporaryBeginJavaScriptArraySpreadJoin1229Return(
        Box<self::JavaScriptArraySpreadTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235 {
    MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235Next,
    MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1236Result {
    MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1236Done,
    MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1236Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_search_state_error_loop1236(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1232: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1232: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1236Result {
    loop {
        return if (source_ml_source1_temporary_index1232
            < ({
                let ml_reference_array3042 = source_ml_source1_temporary_values1232.clone();
                let ml_reference_projection3042 =
                    self::ml::Big::from_u128(ml_reference_array3042.borrow().len() as u128);
                ml_reference_projection3042
            })) {
            {
                let value = {
                    let ml_reference_array3043 = source_ml_source1_temporary_values1232.clone();
                    let ml_reference_position3043 = source_ml_source1_temporary_index1232.clone();
                    let ml_reference_projection3043 = self::ml_array::at(
                        &*ml_reference_array3043.borrow(),
                        ml_reference_position3043.to_index(),
                    );
                    ml_reference_projection3043
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1236Result::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1236Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1232_2 =
                                source_ml_source1_temporary_index1232
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1232 =
                                    source_ml_source1_temporary_index1232_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1236Result::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1236Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234 {
    MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234Next,
    MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1233Result {
    MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1233Done,
    MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1233Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_search_state_error_loop1233(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1231: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1231: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1233Result {
    loop {
        return if (source_ml_source1_temporary_index1231
            < ({
                let ml_reference_array3044 = source_ml_source1_temporary_values1231.clone();
                let ml_reference_projection3044 =
                    self::ml::Big::from_u128(ml_reference_array3044.borrow().len() as u128);
                ml_reference_projection3044
            })) {
            {
                let value = {
                    let ml_reference_array3045 = source_ml_source1_temporary_values1231.clone();
                    let ml_reference_position3045 = source_ml_source1_temporary_index1231.clone();
                    let ml_reference_projection3045 = self::ml_array::at(
                        &*ml_reference_array3045.borrow(),
                        ml_reference_position3045.to_index(),
                    );
                    ml_reference_projection3045
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1233Result::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1233Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1231_2 =
                                source_ml_source1_temporary_index1231
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1231 =
                                    source_ml_source1_temporary_index1231_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1233Result::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1233Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryStepJavaScriptArraySearchJoin1239 {
    MlSource1TemporaryStepJavaScriptArraySearchJoin1239Next,
    MlSource1TemporaryStepJavaScriptArraySearchJoin1239Return(
        Box<self::JavaScriptArraySearchTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryStepJavaScriptArraySearchLoop1238Result {
    MlSource1TemporaryStepJavaScriptArraySearchLoop1238Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_array_search_loop1238(
    name: &self::ml_string::Text,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryStepJavaScriptArraySearchLoop1238Result {
    loop {
        return if (at < (name.length())) {
            {
                let source_ml_source1_temporary_step_java_script_array_search_ignored1240 = {
                    let ml_method_owner3046 = units.clone();
                    let ml_method_argument3046_0 = {
                        let source_ml_source2_string_unit_operand41855 = name.clone();
                        {
                            let source_ml_source2_string_unit_operand41856 = at;
                            (std::rc::Rc::new(
                                move |text: self::ml_string::Text, position: f64| -> f64 {
                                    {
                                        let index = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if ((index < (0f64)) || (index >= (text.length()))) {
                                            f64::NAN
                                        } else {
                                            ({
                                    let source_ml_source2_string_unit_operand41862 = {
                                        let source_ml_source2_string_range_operand41857 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand41858 = index;
                                            {
                                                let source_ml_source2_string_range_operand41859 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position41860 = source_ml_source2_string_range_operand41858;
                                                    {
                                                        let source_ml_source2_string_range_position41861 = source_ml_source2_string_range_operand41859;
                                                        source_ml_source2_string_range_operand41857.slice(source_ml_source2_string_range_position41860, source_ml_source2_string_range_position41861)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand41863 = 0f64;
                                        source_ml_source2_string_unit_operand41862.code_point_at(source_ml_source2_string_unit_operand41863)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source2_string_unit_operand41855.clone(),
                                source_ml_source2_string_unit_operand41856,
                            )
                        }
                    };
                    let ml_inserted = vec![ml_method_argument3046_0];
                    let mut ml_method_values3046 = ml_method_owner3046.borrow_mut();
                    let ml_length = ml_method_values3046
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values3046.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryStepJavaScriptArraySearchLoop1238Result::MlSource1TemporaryStepJavaScriptArraySearchLoop1238Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246 {
    MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246Next,
    MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1248Result {
    MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1248Done,
    MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1248Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_callback_state_error_loop1248(
    mut expected: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut args: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1248Result {
    loop {
        return if (at
            < ({
                let ml_reference_array3047 = expected.clone();
                let ml_reference_projection3047 = ml_reference_array3047.borrow().len() as f64;
                ml_reference_projection3047
            })) {
            if !(self::test_java_script_semantic_same_value(
                ({
                    let ml_reference_array3048 = args.clone();
                    let ml_reference_position3048 = at;
                    let ml_reference_projection3048 = self::ml_array::optional_at(
                        &*ml_reference_array3048.borrow(),
                        self::ml_array::number_index(ml_reference_position3048),
                    );
                    ml_reference_projection3048
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
                ({
                    let ml_reference_array3049 = expected.clone();
                    let ml_reference_position3049 = at;
                    let ml_reference_projection3049 = self::ml_array::optional_at(
                        &*ml_reference_array3049.borrow(),
                        self::ml_array::number_index(ml_reference_position3049),
                    );
                    ml_reference_projection3049
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
            )) {
                self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1248Result::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1248Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 112, 114, 101, 100, 105, 99, 97, 116, 101, 47, 114, 101, 100, 117, 99, 101, 114, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 101, 114, 101, 100, 32, 101, 108, 101, 109, 101, 110, 116, 32, 97, 110, 100, 32, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1248Result::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1248Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1247Result {
    MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1247Done,
    MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1247Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_callback_state_error_loop1247(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1243: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1243: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1247Result {
    loop {
        return if (source_ml_source1_temporary_index1243
            < ({
                let ml_reference_array3050 = source_ml_source1_temporary_values1243.clone();
                let ml_reference_projection3050 =
                    self::ml::Big::from_u128(ml_reference_array3050.borrow().len() as u128);
                ml_reference_projection3050
            })) {
            {
                let value = {
                    let ml_reference_array3051 = source_ml_source1_temporary_values1243.clone();
                    let ml_reference_position3051 = source_ml_source1_temporary_index1243.clone();
                    let ml_reference_projection3051 = self::ml_array::at(
                        &*ml_reference_array3051.borrow(),
                        ml_reference_position3051.to_index(),
                    );
                    ml_reference_projection3051
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1247Result::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1247Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1243_2 =
                                source_ml_source1_temporary_index1243
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1243 =
                                    source_ml_source1_temporary_index1243_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1247Result::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1247Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245 {
    MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245Next,
    MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1244Result {
    MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1244Done,
    MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1244Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_callback_state_error_loop1244(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1242: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1242: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1244Result {
    loop {
        return if (source_ml_source1_temporary_index1242
            < ({
                let ml_reference_array3052 = source_ml_source1_temporary_values1242.clone();
                let ml_reference_projection3052 =
                    self::ml::Big::from_u128(ml_reference_array3052.borrow().len() as u128);
                ml_reference_projection3052
            })) {
            {
                let value = {
                    let ml_reference_array3053 = source_ml_source1_temporary_values1242.clone();
                    let ml_reference_position3053 = source_ml_source1_temporary_index1242.clone();
                    let ml_reference_projection3053 = self::ml_array::at(
                        &*ml_reference_array3053.borrow(),
                        ml_reference_position3053.to_index(),
                    );
                    ml_reference_projection3053
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1244Result::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1244Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1242_2 =
                                source_ml_source1_temporary_index1242
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1242 =
                                    source_ml_source1_temporary_index1242_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1244Result::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1244Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryStepJavaScriptArrayCallbackJoin1251 {
    MlSource1TemporaryStepJavaScriptArrayCallbackJoin1251Next,
    MlSource1TemporaryStepJavaScriptArrayCallbackJoin1251Return(
        Box<self::JavaScriptArrayCallbackTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryStepJavaScriptArrayCallbackLoop1250Result {
    MlSource1TemporaryStepJavaScriptArrayCallbackLoop1250Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_array_callback_loop1250(
    name: &self::ml_string::Text,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryStepJavaScriptArrayCallbackLoop1250Result {
    loop {
        return if (at < (name.length())) {
            {
                let source_ml_source1_temporary_step_java_script_array_callback_ignored1252 = {
                    let ml_method_owner3054 = units.clone();
                    let ml_method_argument3054_0 = {
                        let source_ml_source2_string_unit_operand41872 = name.clone();
                        {
                            let source_ml_source2_string_unit_operand41873 = at;
                            (std::rc::Rc::new(
                                move |text: self::ml_string::Text, position: f64| -> f64 {
                                    {
                                        let index = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if ((index < (0f64)) || (index >= (text.length()))) {
                                            f64::NAN
                                        } else {
                                            ({
                                    let source_ml_source2_string_unit_operand41879 = {
                                        let source_ml_source2_string_range_operand41874 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand41875 = index;
                                            {
                                                let source_ml_source2_string_range_operand41876 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position41877 = source_ml_source2_string_range_operand41875;
                                                    {
                                                        let source_ml_source2_string_range_position41878 = source_ml_source2_string_range_operand41876;
                                                        source_ml_source2_string_range_operand41874.slice(source_ml_source2_string_range_position41877, source_ml_source2_string_range_position41878)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand41880 = 0f64;
                                        source_ml_source2_string_unit_operand41879.code_point_at(source_ml_source2_string_unit_operand41880)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source2_string_unit_operand41872.clone(),
                                source_ml_source2_string_unit_operand41873,
                            )
                        }
                    };
                    let ml_inserted = vec![ml_method_argument3054_0];
                    let mut ml_method_values3054 = ml_method_owner3054.borrow_mut();
                    let ml_length = ml_method_values3054
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values3054.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryStepJavaScriptArrayCallbackLoop1250Result::MlSource1TemporaryStepJavaScriptArrayCallbackLoop1250Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1257Result {
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1257Done,
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1257Return(self::ml_string::Text),
}
