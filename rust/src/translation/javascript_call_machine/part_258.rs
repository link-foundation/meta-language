// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallArgumentList;
use super::JavaScriptCallCompletion;
use super::JavaScriptCallCompletionTransition;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallPending;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_call_frame;
use super::retain_java_script_call_argument_lists;
use super::test_java_script_call_coordinate;
use super::test_java_script_semantic_same_value;
use super::write_java_script_call_register;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481 {
    MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481Next,
    MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480 {
    MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480Next,
    MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1488 {
    MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1488Next,
    MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1488Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1487 {
    MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1487Next,
    MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1487Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1489Result {
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1489Done,
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1489Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_frame_error_loop1489(
    mut frame: self::JavaScriptCallFrame,
    mut fn_: self::JavaScriptCallFunction,
    mut closure: Option<self::JavaScriptOrdinaryObject>,
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1489Result {
    loop {
        return if (index
            < ({
                let source_ml_source2_nullish_field_receiver42344 = ({
                    let source_ml_source2_nullish_field_receiver42341 = (closure.clone(),);
                    if match (source_ml_source2_nullish_field_receiver42341.clone()).0 {
                        Some(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        None => false,
                    } {
                        {
                            let source_ml_source2_nullish_field_present42342 =
                                ((source_ml_source2_nullish_field_receiver42341.clone()).0)
                                    .clone()
                                    .expect("a proven present value");
                            match source_ml_source2_nullish_field_present42342.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value42343,
                                    _,
                                    _,
                                ) => Some(source_ml_source2_data_property_value42343.clone()),
                                _ => {
                                    None::<
                                        std::rc::Rc<
                                            std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
                                        >,
                                    >
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units: (self::ml_string::Text::from_units(vec![
                                67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114,
                                111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110,
                                100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105,
                                110, 103, 32, 39, 99, 97, 112, 116, 117, 114, 101, 115, 39, 41,
                            ]))
                            .0,
                            message_present: true,
                        })
                        .raise()
                    }
                },);
                if match (source_ml_source2_nullish_field_receiver42344.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    {
                        let source_ml_source2_nullish_field_present42345 =
                            ((source_ml_source2_nullish_field_receiver42344.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        {
                            let ml_reference_array3244 =
                                source_ml_source2_nullish_field_present42345.clone();
                            let ml_reference_projection3244 =
                                ml_reference_array3244.borrow().len() as f64;
                            ml_reference_projection3244
                        }
                    }
                } else {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("TypeError"),
                        message_units: (self::ml_string::Text::from_units(vec![
                            67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111,
                            112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100,
                            101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103,
                            32, 39, 108, 101, 110, 103, 116, 104, 39, 41,
                        ]))
                        .0,
                        message_present: true,
                    })
                    .raise()
                }
            })) {
            if !(self::test_java_script_semantic_same_value(
                ({
                    let source_ml_source2_array_read_value42348 = match frame.clone() {
                        self::JavaScriptCallFrame::CallFrame(
                            _,
                            _,
                            source_ml_source2_data_property_value42346,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value42346.clone(),
                    };
                    {
                        let source_ml_source2_array_read_index42349 = ((match fn_.clone() {
                            self::JavaScriptCallFunction::CallFunction(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value42347,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value42347,
                        } + 1f64)
                            + index);
                        {
                            let ml_reference_array3245 =
                                source_ml_source2_array_read_value42348.clone();
                            let ml_reference_position3245 = source_ml_source2_array_read_index42349;
                            let ml_reference_projection3245 = self::ml_array::optional_at(
                                &*ml_reference_array3245.borrow(),
                                self::ml_array::number_index(ml_reference_position3245),
                            );
                            ml_reference_projection3245
                        }
                    }
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
                ({
                    let source_ml_source2_nullish_field_receiver42353 = (
                        {
                            let source_ml_source2_nullish_field_receiver42350 = (closure.clone(),);
                            if match (source_ml_source2_nullish_field_receiver42350.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source2_nullish_field_present42351 =
                                        ((source_ml_source2_nullish_field_receiver42350.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source2_nullish_field_present42351.clone() {
                                        self::JavaScriptOrdinaryObject::ClosureObject(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value42352,
                                            _,
                                            _,
                                        ) => {
                                            Some(source_ml_source2_data_property_value42352.clone())
                                        }
                                        _ => {
                                            None::<
                                                std::rc::Rc<
                                                    std::cell::RefCell<
                                                        Vec<self::JavaScriptSemanticValue>,
                                                    >,
                                                >,
                                            >
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 112, 116,
                                        117, 114, 101, 115, 39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        },
                        index,
                    );
                    {
                        let source_ml_source2_computed_property_key42354 =
                            (source_ml_source2_nullish_field_receiver42353.clone()).1;
                        if match (source_ml_source2_nullish_field_receiver42353.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source2_nullish_field_present42355 =
                                    ((source_ml_source2_nullish_field_receiver42353.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                {
                                    let ml_reference_array3246 =
                                        source_ml_source2_nullish_field_present42355.clone();
                                    let ml_reference_position3246 =
                                        source_ml_source2_computed_property_key42354;
                                    let ml_reference_projection3246 = self::ml_array::optional_at(
                                        &*ml_reference_array3246.borrow(),
                                        self::ml_array::number_index(ml_reference_position3246),
                                    );
                                    ml_reference_projection3246
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (((self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39,
                                ]))
                                .concat(
                                    &self::ml_string::Text::from_text(&self::ml_number::js_number(
                                        (source_ml_source2_nullish_field_receiver42353.clone()).1,
                                    )),
                                ))
                                .concat(&self::ml_string::Text::from_units(vec![39, 41])))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    }
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
            )) {
                self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1489Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1489Return(self::ml_string::Text::from_units(vec![97, 110, 32, 97, 114, 114, 111, 119, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 118, 97, 108, 117, 101, 115]))
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1489Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1489Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1486Result {
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1486Done,
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1486Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_frame_error_loop1486(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1483: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1483: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1486Result {
    loop {
        return if (source_ml_source1_temporary_index1483
            < ({
                let ml_reference_array3247 = source_ml_source1_temporary_values1483.clone();
                let ml_reference_projection3247 =
                    self::ml::Big::from_u128(ml_reference_array3247.borrow().len() as u128);
                ml_reference_projection3247
            })) {
            {
                let value = {
                    let ml_reference_array3248 = source_ml_source1_temporary_values1483.clone();
                    let ml_reference_position3248 = source_ml_source1_temporary_index1483.clone();
                    let ml_reference_projection3248 = self::ml_array::at(
                        &*ml_reference_array3248.borrow(),
                        ml_reference_position3248.to_index(),
                    );
                    ml_reference_projection3248
                };
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone()))
                    != (self::ml_string::Text::from_units(vec![])))
                {
                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1486Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1486Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_source1_temporary_index1483_2 =
                            source_ml_source1_temporary_index1483.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index1483 =
                                source_ml_source1_temporary_index1483_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1486Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1486Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1485Result {
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1485Done,
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1485Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_frame_error_loop1485(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1482: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1482: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1485Result {
    loop {
        return if (source_ml_source1_temporary_index1482
            < ({
                let ml_reference_array3249 = source_ml_source1_temporary_values1482.clone();
                let ml_reference_projection3249 =
                    self::ml::Big::from_u128(ml_reference_array3249.borrow().len() as u128);
                ml_reference_projection3249
            })) {
            {
                let value = {
                    let ml_reference_array3250 = source_ml_source1_temporary_values1482.clone();
                    let ml_reference_position3250 = source_ml_source1_temporary_index1482.clone();
                    let ml_reference_projection3250 = self::ml_array::at(
                        &*ml_reference_array3250.borrow(),
                        ml_reference_position3250.to_index(),
                    );
                    ml_reference_projection3250
                };
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone()))
                    != (self::ml_string::Text::from_units(vec![])))
                {
                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1485Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1485Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_source1_temporary_index1482_2 =
                            source_ml_source1_temporary_index1482.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index1482 =
                                source_ml_source1_temporary_index1482_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1485Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1485Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1495 {
    MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1495Next,
    MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1495Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1494 {
    MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1494Next,
    MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1494Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Result {
    MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Done,
    MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_frame_control_error_loop1493(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut fn_: self::JavaScriptCallFunction,
    mut source_ml_source1_temporary_values1490: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallHandler>>,
    >,
    mut source_ml_source1_temporary_index1490: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Result {
    loop {
        return match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(
                _,
                _,
                _,
                _,
                _,
                shared_constructor_field5,
                _,
                shared_constructor_field6,
                _,
                _,
            ) => {
                if (source_ml_source1_temporary_index1490
                    < ({
                        let ml_reference_array3251 = source_ml_source1_temporary_values1490.clone();
                        let ml_reference_projection3251 =
                            self::ml::Big::from_u128(ml_reference_array3251.borrow().len() as u128);
                        ml_reference_projection3251
                    }))
                {
                    {
                        let handler = {
                            let ml_reference_array3252 =
                                source_ml_source1_temporary_values1490.clone();
                            let ml_reference_position3252 =
                                source_ml_source1_temporary_index1490.clone();
                            let ml_reference_projection3252 = self::ml_array::at(
                                &*ml_reference_array3252.borrow(),
                                ml_reference_position3252.to_index(),
                            );
                            ml_reference_projection3252
                        };
                        match handler.clone() {
                            self::JavaScriptCallHandler::CallHandler(target, destination) => {
                                if (!(self::test_java_script_call_coordinate(destination, shared_constructor_field5)) || !(self::test_java_script_call_coordinate(target, { let ml_reference_array3253 = shared_constructor_field6.clone(); let ml_reference_projection3253 = ml_reference_array3253.borrow().len() as f64; ml_reference_projection3253 }))) {
                                    self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Result::MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 104, 97, 110, 100, 108, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                } else {
                                    {
                                        let source_ml_source1_temporary_index1490_2 = source_ml_source1_temporary_index1490.add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_source1_temporary_index1490 = source_ml_source1_temporary_index1490_2.clone();
                                            continue;
                                        }
                                    }
                                }
                            }
                            _ => {
                                match handler.clone() {
                                    self::JavaScriptCallHandler::CallFinallyHandler(target_2) => {
                                        if !(self::test_java_script_call_coordinate(target_2, { let ml_reference_array3254 = shared_constructor_field6.clone(); let ml_reference_projection3254 = ml_reference_array3254.borrow().len() as f64; ml_reference_projection3254 })) {
                                            self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Result::MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Return(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 105, 122, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 101, 110, 116, 114, 121]))
                                        } else {
                                            {
                                                let source_ml_source1_temporary_index1490_3 = source_ml_source1_temporary_index1490.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_source1_temporary_index1490 = source_ml_source1_temporary_index1490_3.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        match handler.clone() {
                                            self::JavaScriptCallHandler::CallDeferredCompletion(completion) => {
                                                let completion = *completion;
                                                match completion.clone() {
                                                    self::JavaScriptCallCompletion::CallCompletion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4) => {
                                                        let shared_constructor_field2 = *shared_constructor_field2;
                                                        if (((((((!(self::ml_array::includes(&{ let ml_reference_value3255 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110]), self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119]), self::ml_string::Text::from_units(vec![106, 117, 109, 112])]))).borrow().clone(); ml_reference_value3255 }, &shared_constructor_field1.clone(), 0f64, false, |a, b| a == b)) || ((self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field2.clone())) != (self::ml_string::Text::from_units(vec![])))) || !(self::ml_math::is_safe_integer(shared_constructor_field4))) || (shared_constructor_field4 < (0f64))) || (shared_constructor_field4 > (1024f64))) || {
                                                            let source_ml_source2_same_value_operand42376 = shared_constructor_field4;
                                                            {
                                                                let source_ml_source2_same_value_operand42377 = (-0f64);
                                                                (((source_ml_source2_same_value_operand42376 == source_ml_source2_same_value_operand42377) && ((source_ml_source2_same_value_operand42376 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand42376)) == ((1f64 / source_ml_source2_same_value_operand42377))))) || ((source_ml_source2_same_value_operand42376 != source_ml_source2_same_value_operand42376) && (source_ml_source2_same_value_operand42377 != source_ml_source2_same_value_operand42377)))
                                                            }
                                                        }) || ((shared_constructor_field1 == (self::ml_string::Text::from_units(vec![106, 117, 109, 112]))) && !(self::test_java_script_call_coordinate(shared_constructor_field3, { let ml_reference_array3256 = shared_constructor_field6.clone(); let ml_reference_projection3256 = ml_reference_array3256.borrow().len() as f64; ml_reference_projection3256 })))) || ((shared_constructor_field1 != (self::ml_string::Text::from_units(vec![106, 117, 109, 112]))) && ((shared_constructor_field3 != ((-1f64))) || (shared_constructor_field4 != (0f64))))) {
                                                            self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Result::MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Return(self::ml_string::Text::from_units(vec![100, 101, 102, 101, 114, 114, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 115, 44, 32, 107, 105, 110, 100, 115, 32, 97, 110, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 111, 110, 116, 114, 111, 108, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                                        } else {
                                                            {
                                                                let source_ml_source1_temporary_index1490_4 = source_ml_source1_temporary_index1490.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index1490 = source_ml_source1_temporary_index1490_4.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Result::MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Return(self::ml_string::Text::from_units(vec![104, 97, 110, 100, 108, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 99, 97, 116, 99, 104, 44, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114, 32, 111, 114, 32, 100, 101, 102, 101, 114, 114, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 107, 105, 110, 100]))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Result::MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Result {
    MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Done(
        Box<self::JavaScriptCallCompletion>,
        Box<self::JavaScriptOrdinaryHeap>,
    ),
    MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Return(
        Box<self::JavaScriptCallCompletionTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1498Result {
    MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1498Done(
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallHandler>>>,
    ),
    MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1498Return(
        Box<self::JavaScriptCallCompletionTransition>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_unwind_java_script_call_completion_loop1498(
    mut completion: self::JavaScriptCallCompletion,
    mut frames: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFrame>>>,
    mut heap: self::JavaScriptOrdinaryHeap,
    mut frame: self::JavaScriptCallFrame,
    mut handlers: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallHandler>>>,
) -> self::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1498Result {
    loop {
        return match frame.clone() {
            self::JavaScriptCallFrame::CallFrame(
                shared_constructor_field15,
                shared_constructor_field16,
                shared_constructor_field17,
                shared_constructor_field18,
                shared_constructor_field19,
                shared_constructor_field20,
                shared_constructor_field21,
                shared_constructor_field22,
                shared_constructor_field23,
                shared_constructor_field24,
            ) => {
                let shared_constructor_field20 = *shared_constructor_field20;
                match completion.clone() {
                    self::JavaScriptCallCompletion::CallCompletion(
                        shared_constructor_field11,
                        shared_constructor_field12,
                        shared_constructor_field13,
                        shared_constructor_field14,
                    ) => {
                        let shared_constructor_field12 = *shared_constructor_field12;
                        if (({
                            let ml_reference_array3257 = handlers.clone();
                            let ml_reference_projection3257 =
                                ml_reference_array3257.borrow().len() as f64;
                            ml_reference_projection3257
                        }) > (if (shared_constructor_field11
                            == (self::ml_string::Text::from_units(vec![106, 117, 109, 112])))
                        {
                            shared_constructor_field14
                        } else {
                            0f64
                        })) {
                            {
                                let handler = {
                                    let source_ml_source2_array_read_index42395 = ({
                                        let ml_reference_array3258 = handlers.clone();
                                        let ml_reference_projection3258 =
                                            ml_reference_array3258.borrow().len() as f64;
                                        ml_reference_projection3258
                                    } - 1f64);
                                    {
                                        let ml_reference_array3259 = handlers.clone();
                                        let ml_reference_position3259 =
                                            source_ml_source2_array_read_index42395;
                                        let ml_reference_projection3259 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array3259.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position3259,
                                                ),
                                            );
                                        ml_reference_projection3259
                                    }
                                };
                                {
                                    let handlers_2 = std::rc::Rc::new(std::cell::RefCell::new(
                                        self::ml_array::slice(
                                            &{
                                                let ml_reference_value3260 =
                                                    handlers.borrow().clone();
                                                ml_reference_value3260
                                            },
                                            0f64,
                                            (-1f64),
                                        ),
                                    ));
                                    if match (handler.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        self::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1498Result::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1498Return(Box::new(self::JavaScriptCallCompletionTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 104, 97, 110, 100, 108, 101, 114, 32, 115, 116, 111, 114, 97, 103, 101, 32, 114, 101, 109, 97, 105, 110, 115, 32, 100, 101, 110, 115, 101]))))
                                    } else {
                                        match handler.clone().expect("a proven present value") {
                                            self::JavaScriptCallHandler::CallFinallyHandler(target) => {
                                                self::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1498Result::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1498Return(Box::new(self::JavaScriptCallCompletionTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap.clone(), frames.clone(), {
                                                    let source_ml_source2_nominal_field42409 = shared_constructor_field15.clone();
                                                    {
                                                        let source_ml_source2_nominal_field42411 = shared_constructor_field16;
                                                        {
                                                            let source_ml_source2_nominal_field42413 = shared_constructor_field17.clone();
                                                            {
                                                                let source_ml_source2_nominal_field42415 = shared_constructor_field18.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field42417 = shared_constructor_field19.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field42419 = shared_constructor_field20.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field42421 = shared_constructor_field21.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field42423 = shared_constructor_field22.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field42425 = shared_constructor_field23.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field42427 = shared_constructor_field24.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_override42429 = self::retain_java_script_call_argument_lists((match frame.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, source_ml_source2_data_property_value42396, _, _) => {
                                                                                                    ((source_ml_source2_data_property_value42396.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>>,))).0).clone()
                                                                                                }
                                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallArgumentList>::new()))), { let ml_reference_array3261 = handlers_2.clone(); let ml_reference_projection3261 = ml_reference_array3261.borrow().len() as f64; ml_reference_projection3261 });
                                                                                            {
                                                                                                let source_ml_source2_nominal_override42431 = self::JavaScriptCallPending::CallIdle;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override42432 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                        let source_ml_source2_concat_value42407 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallHandler::CallDeferredCompletion(Box::new({
                                                                                                            let source_ml_source2_nominal_field42398 = shared_constructor_field11.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field42400 = shared_constructor_field12.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field42402 = shared_constructor_field13;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field42404 = shared_constructor_field14;
                                                                                                                        self::JavaScriptCallCompletion::CallCompletion(source_ml_source2_nominal_field42398.clone(), Box::new(source_ml_source2_nominal_field42400.clone()), source_ml_source2_nominal_field42402, source_ml_source2_nominal_field42404)
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }))]));
                                                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallHandler>::new(), { let ml_reference_value3262 = handlers_2.borrow().clone(); ml_reference_value3262 }), { let ml_reference_value3263 = source_ml_source2_concat_value42407.borrow().clone(); ml_reference_value3263 })
                                                                                                    }));
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field42409.clone(), target, source_ml_source2_nominal_field42413.clone(), source_ml_source2_nominal_field42415.clone(), source_ml_source2_nominal_override42432.clone(), Box::new(source_ml_source2_nominal_override42431.clone()), source_ml_source2_nominal_field42421.clone(), Some((Some(source_ml_source2_nominal_override42429.clone()),)), source_ml_source2_nominal_field42425.clone(), source_ml_source2_nominal_field42427.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                })))))
                                            }
                                            _ => {
                                                match handler.clone().expect("a proven present value") {
                                                    self::JavaScriptCallHandler::CallHandler(target_2, destination) => {
                                                        if (shared_constructor_field11 == (self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119]))) {
                                                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1498Result::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1498Return(Box::new(self::JavaScriptCallCompletionTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap.clone(), frames.clone(), {
                                                                let source_ml_source2_nominal_spread42437 = self::write_java_script_call_register(frame.clone(), destination, shared_constructor_field12.clone());
                                                                match source_ml_source2_nominal_spread42437.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10) => {
                                                                        let shared_constructor_field6 = *shared_constructor_field6;
                                                                        {
                                                                            let source_ml_source2_nominal_field42438 = shared_constructor_field1.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field42440 = shared_constructor_field2;
                                                                                {
                                                                                    let source_ml_source2_nominal_field42442 = shared_constructor_field3.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field42444 = shared_constructor_field4.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field42446 = shared_constructor_field5.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field42448 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field42450 = shared_constructor_field7.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field42452 = shared_constructor_field8.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field42454 = shared_constructor_field9.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field42456 = shared_constructor_field10.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override42458 = self::retain_java_script_call_argument_lists((match frame.clone() {
                                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, source_ml_source2_data_property_value42436, _, _) => {
                                                                                                                            ((source_ml_source2_data_property_value42436.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>>,))).0).clone()
                                                                                                                        }
                                                                                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallArgumentList>::new()))), { let ml_reference_array3264 = handlers_2.clone(); let ml_reference_projection3264 = ml_reference_array3264.borrow().len() as f64; ml_reference_projection3264 });
                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field42438.clone(), target_2, source_ml_source2_nominal_field42442.clone(), source_ml_source2_nominal_field42444.clone(), handlers_2.clone(), Box::new(source_ml_source2_nominal_field42448.clone()), source_ml_source2_nominal_field42450.clone(), Some((Some(source_ml_source2_nominal_override42458.clone()),)), source_ml_source2_nominal_field42454.clone(), source_ml_source2_nominal_field42456.clone())
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            })))))
                                                        } else {
                                                            {
                                                                handlers = handlers_2.clone();
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        {
                                                            handlers = handlers_2.clone();
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1498Result::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1498Done(handlers.clone())
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497 {
    MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Next(
        Box<self::JavaScriptCallCompletion>,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFrame>>>,
        Box<self::JavaScriptOrdinaryHeap>,
        Box<self::JavaScriptCallFrame>,
    ),
    MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(
        Box<self::JavaScriptCallCompletionTransition>,
    ),
}
