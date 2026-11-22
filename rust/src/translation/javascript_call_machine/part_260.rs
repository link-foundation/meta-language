// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallArgumentList;
use super::JavaScriptCallCompletion;
use super::JavaScriptCallFrame;
use super::JavaScriptCallHandler;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptIteratorCompletion;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_value_error;

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510 {
    MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Next,
    MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Return(
        Box<self::JavaScriptCallMachineTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryBeginJavaScriptCallMachineLoop1514Result {
    MlSource1TemporaryBeginJavaScriptCallMachineLoop1514Done,
    MlSource1TemporaryBeginJavaScriptCallMachineLoop1514Return(
        Box<self::JavaScriptCallMachineTransition>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_begin_java_script_call_machine_loop1514(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1513: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1513: self::ml::Big,
) -> self::MlSource1TemporaryBeginJavaScriptCallMachineLoop1514Result {
    loop {
        return if (source_ml_source1_temporary_index1513
            < ({
                let ml_reference_array3286 = source_ml_source1_temporary_values1513.clone();
                let ml_reference_projection3286 =
                    self::ml::Big::from_u128(ml_reference_array3286.borrow().len() as u128);
                ml_reference_projection3286
            })) {
            {
                let value = {
                    let ml_reference_array3287 = source_ml_source1_temporary_values1513.clone();
                    let ml_reference_position3287 = source_ml_source1_temporary_index1513.clone();
                    let ml_reference_projection3287 = self::ml_array::at(
                        &*ml_reference_array3287.borrow(),
                        ml_reference_position3287.to_index(),
                    );
                    ml_reference_projection3287
                };
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone()))
                    != (self::ml_string::Text::from_units(vec![])))
                {
                    self::MlSource1TemporaryBeginJavaScriptCallMachineLoop1514Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop1514Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))))
                } else {
                    {
                        let source_ml_source1_temporary_index1513_2 =
                            source_ml_source1_temporary_index1513.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index1513 =
                                source_ml_source1_temporary_index1513_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryBeginJavaScriptCallMachineLoop1514Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop1514Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPushJavaScriptErrorConstructorLoop1519Result {
    MlSource1TemporaryPushJavaScriptErrorConstructorLoop1519Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_push_java_script_error_constructor_loop1520(
    mut argument_slots: f64,
    mut source_ml_source1_temporary_values1518: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>,
    >,
    mut source_ml_source1_temporary_index1518: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index1518
            < ({
                let ml_reference_array3288 = source_ml_source1_temporary_values1518.clone();
                let ml_reference_projection3288 =
                    self::ml::Big::from_u128(ml_reference_array3288.borrow().len() as u128);
                ml_reference_projection3288
            })) {
            {
                let list = {
                    let ml_reference_array3289 = source_ml_source1_temporary_values1518.clone();
                    let ml_reference_position3289 = source_ml_source1_temporary_index1518.clone();
                    let ml_reference_projection3289 = self::ml_array::at(
                        &*ml_reference_array3289.borrow(),
                        ml_reference_position3289.to_index(),
                    );
                    ml_reference_projection3289
                };
                {
                    let argument_slots_2 = (argument_slots + {
                        let ml_reference_array3290 = match list.clone() {
                            self::JavaScriptCallArgumentList::CallArgumentList(
                                _,
                                _,
                                source_ml_source2_data_property_value42922,
                            ) => source_ml_source2_data_property_value42922.clone(),
                        };
                        let ml_reference_projection3290 =
                            ml_reference_array3290.borrow().len() as f64;
                        ml_reference_projection3290
                    });
                    {
                        let source_ml_source1_temporary_index1518_2 =
                            source_ml_source1_temporary_index1518.add(&self::ml::Big::from_i128(1));
                        {
                            (argument_slots, source_ml_source1_temporary_index1518) = (
                                argument_slots_2,
                                source_ml_source1_temporary_index1518_2.clone(),
                            );
                            continue;
                        }
                    }
                }
            }
        } else {
            argument_slots
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_push_java_script_error_constructor_loop1519(
    mut slots: f64,
    mut argument_slots: f64,
    mut source_ml_source1_temporary_values1517: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallFrame>>,
    >,
    mut source_ml_source1_temporary_index1517: self::ml::Big,
) -> self::MlSource1TemporaryPushJavaScriptErrorConstructorLoop1519Result {
    loop {
        return if (source_ml_source1_temporary_index1517
            < ({
                let ml_reference_array3291 = source_ml_source1_temporary_values1517.clone();
                let ml_reference_projection3291 =
                    self::ml::Big::from_u128(ml_reference_array3291.borrow().len() as u128);
                ml_reference_projection3291
            })) {
            {
                let active = {
                    let ml_reference_array3292 = source_ml_source1_temporary_values1517.clone();
                    let ml_reference_position3292 = source_ml_source1_temporary_index1517.clone();
                    let ml_reference_projection3292 = self::ml_array::at(
                        &*ml_reference_array3292.borrow(),
                        ml_reference_position3292.to_index(),
                    );
                    ml_reference_projection3292
                };
                match active.clone() {
                    self::JavaScriptCallFrame::CallFrame(
                        _,
                        _,
                        shared_constructor_field1,
                        shared_constructor_field2,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let slots_2 = (slots + {
                            let ml_reference_array3293 = shared_constructor_field1.clone();
                            let ml_reference_projection3293 =
                                ml_reference_array3293.borrow().len() as f64;
                            ml_reference_projection3293
                        });
                        {
                            let argument_slots_2 = (argument_slots + {
                                let ml_reference_array3294 = shared_constructor_field2.clone();
                                let ml_reference_projection3294 =
                                    ml_reference_array3294.borrow().len() as f64;
                                ml_reference_projection3294
                            });
                            {
                                let source_ml_source1_temporary_values1518 =
                                    (match active.clone() {
                                        self::JavaScriptCallFrame::CallFrame(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value42927,
                                            _,
                                            _,
                                        ) => ((source_ml_source2_data_property_value42927
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<
                                                    std::rc::Rc<
                                                        std::cell::RefCell<
                                                            Vec<self::JavaScriptCallArgumentList>,
                                                        >,
                                                    >,
                                                >,)
                                            }))
                                        .0)
                                            .clone(),
                                    })
                                    .clone()
                                    .unwrap_or_else(|| {
                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                            self::JavaScriptCallArgumentList,
                                        >::new(
                                        )))
                                    });
                                {
                                    let source_ml_source1_temporary_index1518 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let argument_slots_3 = self::source_ml_source1_temporary_push_java_script_error_constructor_loop1520(argument_slots_2, source_ml_source1_temporary_values1518.clone(), source_ml_source1_temporary_index1518.clone());
                                        {
                                            let source_ml_source1_temporary_index1517_2 =
                                                source_ml_source1_temporary_index1517
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (
                                                    slots,
                                                    argument_slots,
                                                    source_ml_source1_temporary_index1517,
                                                ) = (
                                                    slots_2,
                                                    argument_slots_3,
                                                    source_ml_source1_temporary_index1517_2.clone(),
                                                );
                                                continue;
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
            self::MlSource1TemporaryPushJavaScriptErrorConstructorLoop1519Result::MlSource1TemporaryPushJavaScriptErrorConstructorLoop1519Done(slots, argument_slots)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPushJavaScriptOrdinaryClassConstructorLoop1523Result {
    MlSource1TemporaryPushJavaScriptOrdinaryClassConstructorLoop1523Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_push_java_script_ordinary_class_constructor_loop1524(
    mut argument_slots: f64,
    mut source_ml_source1_temporary_values1522: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>,
    >,
    mut source_ml_source1_temporary_index1522: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index1522
            < ({
                let ml_reference_array3295 = source_ml_source1_temporary_values1522.clone();
                let ml_reference_projection3295 =
                    self::ml::Big::from_u128(ml_reference_array3295.borrow().len() as u128);
                ml_reference_projection3295
            })) {
            {
                let list = {
                    let ml_reference_array3296 = source_ml_source1_temporary_values1522.clone();
                    let ml_reference_position3296 = source_ml_source1_temporary_index1522.clone();
                    let ml_reference_projection3296 = self::ml_array::at(
                        &*ml_reference_array3296.borrow(),
                        ml_reference_position3296.to_index(),
                    );
                    ml_reference_projection3296
                };
                {
                    let argument_slots_2 = (argument_slots + {
                        let ml_reference_array3297 = match list.clone() {
                            self::JavaScriptCallArgumentList::CallArgumentList(
                                _,
                                _,
                                source_ml_source2_data_property_value42930,
                            ) => source_ml_source2_data_property_value42930.clone(),
                        };
                        let ml_reference_projection3297 =
                            ml_reference_array3297.borrow().len() as f64;
                        ml_reference_projection3297
                    });
                    {
                        let source_ml_source1_temporary_index1522_2 =
                            source_ml_source1_temporary_index1522.add(&self::ml::Big::from_i128(1));
                        {
                            (argument_slots, source_ml_source1_temporary_index1522) = (
                                argument_slots_2,
                                source_ml_source1_temporary_index1522_2.clone(),
                            );
                            continue;
                        }
                    }
                }
            }
        } else {
            argument_slots
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_push_java_script_ordinary_class_constructor_loop1523(
    mut slots: f64,
    mut argument_slots: f64,
    mut source_ml_source1_temporary_values1521: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallFrame>>,
    >,
    mut source_ml_source1_temporary_index1521: self::ml::Big,
) -> self::MlSource1TemporaryPushJavaScriptOrdinaryClassConstructorLoop1523Result {
    loop {
        return if (source_ml_source1_temporary_index1521
            < ({
                let ml_reference_array3298 = source_ml_source1_temporary_values1521.clone();
                let ml_reference_projection3298 =
                    self::ml::Big::from_u128(ml_reference_array3298.borrow().len() as u128);
                ml_reference_projection3298
            })) {
            {
                let active = {
                    let ml_reference_array3299 = source_ml_source1_temporary_values1521.clone();
                    let ml_reference_position3299 = source_ml_source1_temporary_index1521.clone();
                    let ml_reference_projection3299 = self::ml_array::at(
                        &*ml_reference_array3299.borrow(),
                        ml_reference_position3299.to_index(),
                    );
                    ml_reference_projection3299
                };
                match active.clone() {
                    self::JavaScriptCallFrame::CallFrame(
                        _,
                        _,
                        shared_constructor_field1,
                        shared_constructor_field2,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let slots_2 = (slots + {
                            let ml_reference_array3300 = shared_constructor_field1.clone();
                            let ml_reference_projection3300 =
                                ml_reference_array3300.borrow().len() as f64;
                            ml_reference_projection3300
                        });
                        {
                            let argument_slots_2 = (argument_slots + {
                                let ml_reference_array3301 = shared_constructor_field2.clone();
                                let ml_reference_projection3301 =
                                    ml_reference_array3301.borrow().len() as f64;
                                ml_reference_projection3301
                            });
                            {
                                let source_ml_source1_temporary_values1522 =
                                    (match active.clone() {
                                        self::JavaScriptCallFrame::CallFrame(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value42935,
                                            _,
                                            _,
                                        ) => ((source_ml_source2_data_property_value42935
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<
                                                    std::rc::Rc<
                                                        std::cell::RefCell<
                                                            Vec<self::JavaScriptCallArgumentList>,
                                                        >,
                                                    >,
                                                >,)
                                            }))
                                        .0)
                                            .clone(),
                                    })
                                    .clone()
                                    .unwrap_or_else(|| {
                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                            self::JavaScriptCallArgumentList,
                                        >::new(
                                        )))
                                    });
                                {
                                    let source_ml_source1_temporary_index1522 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let argument_slots_3 = self::source_ml_source1_temporary_push_java_script_ordinary_class_constructor_loop1524(argument_slots_2, source_ml_source1_temporary_values1522.clone(), source_ml_source1_temporary_index1522.clone());
                                        {
                                            let source_ml_source1_temporary_index1521_2 =
                                                source_ml_source1_temporary_index1521
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (
                                                    slots,
                                                    argument_slots,
                                                    source_ml_source1_temporary_index1521,
                                                ) = (
                                                    slots_2,
                                                    argument_slots_3,
                                                    source_ml_source1_temporary_index1521_2.clone(),
                                                );
                                                continue;
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
            self::MlSource1TemporaryPushJavaScriptOrdinaryClassConstructorLoop1523Result::MlSource1TemporaryPushJavaScriptOrdinaryClassConstructorLoop1523Done(slots, argument_slots)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPushJavaScriptCollectionConstructorLoop1527Result {
    MlSource1TemporaryPushJavaScriptCollectionConstructorLoop1527Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_push_java_script_collection_constructor_loop1528(
    mut argument_slots: f64,
    mut source_ml_source1_temporary_values1526: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>,
    >,
    mut source_ml_source1_temporary_index1526: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index1526
            < ({
                let ml_reference_array3302 = source_ml_source1_temporary_values1526.clone();
                let ml_reference_projection3302 =
                    self::ml::Big::from_u128(ml_reference_array3302.borrow().len() as u128);
                ml_reference_projection3302
            })) {
            {
                let list = {
                    let ml_reference_array3303 = source_ml_source1_temporary_values1526.clone();
                    let ml_reference_position3303 = source_ml_source1_temporary_index1526.clone();
                    let ml_reference_projection3303 = self::ml_array::at(
                        &*ml_reference_array3303.borrow(),
                        ml_reference_position3303.to_index(),
                    );
                    ml_reference_projection3303
                };
                {
                    let argument_slots_2 = (argument_slots + {
                        let ml_reference_array3304 = match list.clone() {
                            self::JavaScriptCallArgumentList::CallArgumentList(
                                _,
                                _,
                                source_ml_source2_data_property_value42938,
                            ) => source_ml_source2_data_property_value42938.clone(),
                        };
                        let ml_reference_projection3304 =
                            ml_reference_array3304.borrow().len() as f64;
                        ml_reference_projection3304
                    });
                    {
                        let source_ml_source1_temporary_index1526_2 =
                            source_ml_source1_temporary_index1526.add(&self::ml::Big::from_i128(1));
                        {
                            (argument_slots, source_ml_source1_temporary_index1526) = (
                                argument_slots_2,
                                source_ml_source1_temporary_index1526_2.clone(),
                            );
                            continue;
                        }
                    }
                }
            }
        } else {
            argument_slots
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_push_java_script_collection_constructor_loop1527(
    mut slots: f64,
    mut argument_slots: f64,
    mut source_ml_source1_temporary_values1525: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallFrame>>,
    >,
    mut source_ml_source1_temporary_index1525: self::ml::Big,
) -> self::MlSource1TemporaryPushJavaScriptCollectionConstructorLoop1527Result {
    loop {
        return if (source_ml_source1_temporary_index1525
            < ({
                let ml_reference_array3305 = source_ml_source1_temporary_values1525.clone();
                let ml_reference_projection3305 =
                    self::ml::Big::from_u128(ml_reference_array3305.borrow().len() as u128);
                ml_reference_projection3305
            })) {
            {
                let active = {
                    let ml_reference_array3306 = source_ml_source1_temporary_values1525.clone();
                    let ml_reference_position3306 = source_ml_source1_temporary_index1525.clone();
                    let ml_reference_projection3306 = self::ml_array::at(
                        &*ml_reference_array3306.borrow(),
                        ml_reference_position3306.to_index(),
                    );
                    ml_reference_projection3306
                };
                match active.clone() {
                    self::JavaScriptCallFrame::CallFrame(
                        _,
                        _,
                        shared_constructor_field1,
                        shared_constructor_field2,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let slots_2 = (slots + {
                            let ml_reference_array3307 = shared_constructor_field1.clone();
                            let ml_reference_projection3307 =
                                ml_reference_array3307.borrow().len() as f64;
                            ml_reference_projection3307
                        });
                        {
                            let argument_slots_2 = (argument_slots + {
                                let ml_reference_array3308 = shared_constructor_field2.clone();
                                let ml_reference_projection3308 =
                                    ml_reference_array3308.borrow().len() as f64;
                                ml_reference_projection3308
                            });
                            {
                                let source_ml_source1_temporary_values1526 =
                                    (match active.clone() {
                                        self::JavaScriptCallFrame::CallFrame(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value42943,
                                            _,
                                            _,
                                        ) => ((source_ml_source2_data_property_value42943
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<
                                                    std::rc::Rc<
                                                        std::cell::RefCell<
                                                            Vec<self::JavaScriptCallArgumentList>,
                                                        >,
                                                    >,
                                                >,)
                                            }))
                                        .0)
                                            .clone(),
                                    })
                                    .clone()
                                    .unwrap_or_else(|| {
                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                            self::JavaScriptCallArgumentList,
                                        >::new(
                                        )))
                                    });
                                {
                                    let source_ml_source1_temporary_index1526 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let argument_slots_3 = self::source_ml_source1_temporary_push_java_script_collection_constructor_loop1528(argument_slots_2, source_ml_source1_temporary_values1526.clone(), source_ml_source1_temporary_index1526.clone());
                                        {
                                            let source_ml_source1_temporary_index1525_2 =
                                                source_ml_source1_temporary_index1525
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (
                                                    slots,
                                                    argument_slots,
                                                    source_ml_source1_temporary_index1525,
                                                ) = (
                                                    slots_2,
                                                    argument_slots_3,
                                                    source_ml_source1_temporary_index1525_2.clone(),
                                                );
                                                continue;
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
            self::MlSource1TemporaryPushJavaScriptCollectionConstructorLoop1527Result::MlSource1TemporaryPushJavaScriptCollectionConstructorLoop1527Done(slots, argument_slots)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryResumeJavaScriptErrorValueAllocationJoin1529 {
    MlSource1TemporaryResumeJavaScriptErrorValueAllocationJoin1529Next(
        Box<self::JavaScriptCallFrame>,
    ),
    MlSource1TemporaryResumeJavaScriptErrorValueAllocationJoin1529Return(
        Box<self::JavaScriptCallMachineTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptCallIteratorCloseCompletionLoop1533Result {
    MlSource1TemporaryReadJavaScriptCallIteratorCloseCompletionLoop1533Done,
    MlSource1TemporaryReadJavaScriptCallIteratorCloseCompletionLoop1533Return(
        Box<self::JavaScriptIteratorCompletion>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_iterator_close_completion_loop1533(
    mut frame: self::JavaScriptCallFrame,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptCallIteratorCloseCompletionLoop1533Result {
    loop {
        return if (at >= (0f64)) {
            {
                let handler = {
                    let source_ml_source2_array_read_value42945 = match frame.clone() {
                        self::JavaScriptCallFrame::CallFrame(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value42944,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value42944.clone(),
                    };
                    {
                        let ml_reference_array3309 =
                            source_ml_source2_array_read_value42945.clone();
                        let ml_reference_position3309 = at;
                        let ml_reference_projection3309 = self::ml_array::optional_at(
                            &*ml_reference_array3309.borrow(),
                            self::ml_array::number_index(ml_reference_position3309),
                        );
                        ml_reference_projection3309
                    }
                };
                if {
                    let source_ml_source2_comparison_operand42948 = {
                        let source_ml_source2_optional_chain_receiver42947 = (handler.clone(),);
                        if match (source_ml_source2_optional_chain_receiver42947.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            Some({
                                let source_ml_source1_temporary_optional_chain1532 =
                                    ((source_ml_source2_optional_chain_receiver42947.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_temporary_optional_chain1532.clone() {
                                    self::JavaScriptCallHandler::CallHandler(_, _) => {
                                        self::ml_string::Text::from_units(vec![
                                            99, 97, 108, 108, 72, 97, 110, 100, 108, 101, 114,
                                        ])
                                    }
                                    self::JavaScriptCallHandler::CallFinallyHandler(_) => {
                                        self::ml_string::Text::from_units(vec![
                                            99, 97, 108, 108, 70, 105, 110, 97, 108, 108, 121, 72,
                                            97, 110, 100, 108, 101, 114,
                                        ])
                                    }
                                    self::JavaScriptCallHandler::CallDeferredCompletion(_) => {
                                        self::ml_string::Text::from_units(vec![
                                            99, 97, 108, 108, 68, 101, 102, 101, 114, 114, 101,
                                            100, 67, 111, 109, 112, 108, 101, 116, 105, 111, 110,
                                        ])
                                    }
                                }
                            })
                        } else {
                            None::<self::ml_string::Text>
                        }
                    };
                    match (
                        source_ml_source2_comparison_operand42948.clone(),
                        self::ml_string::Text::from_units(vec![
                            99, 97, 108, 108, 68, 101, 102, 101, 114, 114, 101, 100, 67, 111, 109,
                            112, 108, 101, 116, 105, 111, 110,
                        ]),
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                } {
                    {
                        let original = {
                            let source_ml_source2_nullish_field_receiver42950 = (handler.clone(),);
                            if match (source_ml_source2_nullish_field_receiver42950.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source2_nullish_field_present42951 =
                                        ((source_ml_source2_nullish_field_receiver42950.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source2_nullish_field_present42951.clone() {
                                        self::JavaScriptCallHandler::CallDeferredCompletion(
                                            source_ml_source2_data_property_value42952,
                                        ) => {
                                            let source_ml_source2_data_property_value42952 =
                                                *source_ml_source2_data_property_value42952;
                                            Some(source_ml_source2_data_property_value42952.clone())
                                        }
                                        _ => None::<self::JavaScriptCallCompletion>,
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 111, 109,
                                        112, 108, 101, 116, 105, 111, 110, 39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        };
                        self::MlSource1TemporaryReadJavaScriptCallIteratorCloseCompletionLoop1533Result::MlSource1TemporaryReadJavaScriptCallIteratorCloseCompletionLoop1533Return(Box::new(self::JavaScriptIteratorCompletion::IteratorCompletion(if (({
                            let source_ml_source2_nullish_field_receiver42956 = (original.clone(),);
                            if match (source_ml_source2_nullish_field_receiver42956.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                {
                                    let source_ml_source2_nullish_field_present42957 = ((source_ml_source2_nullish_field_receiver42956.clone()).0).clone().expect("a proven present value");
                                    match source_ml_source2_nullish_field_present42957.clone() {
                                        self::JavaScriptCallCompletion::CallCompletion(source_ml_source2_data_property_value42958, _, _, _) => {
                                            source_ml_source2_data_property_value42958.clone()
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 105, 110, 100, 39, 41])).0, message_present: true }).raise()
                            }
                        }) == (self::ml_string::Text::from_units(vec![106, 117, 109, 112]))) {
                            self::ml_string::Text::from_units(vec![110, 111, 114, 109, 97, 108])
                        } else {
                            {
                                let source_ml_source2_nullish_field_receiver42959 = (original.clone(),);
                                if match (source_ml_source2_nullish_field_receiver42959.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source2_nullish_field_present42960 = ((source_ml_source2_nullish_field_receiver42959.clone()).0).clone().expect("a proven present value");
                                        match source_ml_source2_nullish_field_present42960.clone() {
                                            self::JavaScriptCallCompletion::CallCompletion(source_ml_source2_data_property_value42961, _, _, _) => {
                                                source_ml_source2_data_property_value42961.clone()
                                            }
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 105, 110, 100, 39, 41])).0, message_present: true }).raise()
                                }
                            }
                        }, Box::new({
                            let source_ml_source2_nullish_field_receiver42962 = (original.clone(),);
                            if match (source_ml_source2_nullish_field_receiver42962.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                {
                                    let source_ml_source2_nullish_field_present42963 = ((source_ml_source2_nullish_field_receiver42962.clone()).0).clone().expect("a proven present value");
                                    match source_ml_source2_nullish_field_present42963.clone() {
                                        self::JavaScriptCallCompletion::CallCompletion(_, source_ml_source2_data_property_value42964, _, _) => {
                                            let source_ml_source2_data_property_value42964 = *source_ml_source2_data_property_value42964;
                                            source_ml_source2_data_property_value42964.clone()
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 118, 97, 108, 117, 101, 39, 41])).0, message_present: true }).raise()
                            }
                        }), (-1f64))))
                    }
                } else {
                    {
                        let at_2 = (at - 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallIteratorCloseCompletionLoop1533Result::MlSource1TemporaryReadJavaScriptCallIteratorCloseCompletionLoop1533Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPushJavaScriptConstructionBodyLoop1537Result {
    MlSource1TemporaryPushJavaScriptConstructionBodyLoop1537Done(f64, f64),
}
