// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallArgumentList;
use super::JavaScriptCallFrame;
use super::MlSource1TemporaryPushJavaScriptConstructionBodyLoop1537Result;
use super::ml;
use super::ml_array;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_push_java_script_construction_body_loop1538(
    mut count: f64,
    mut source_ml_source1_temporary_values1536: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>,
    >,
    mut source_ml_source1_temporary_index1536: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index1536
            < ({
                let ml_reference_array3310 = source_ml_source1_temporary_values1536.clone();
                let ml_reference_projection3310 =
                    self::ml::Big::from_u128(ml_reference_array3310.borrow().len() as u128);
                ml_reference_projection3310
            })) {
            {
                let list = {
                    let ml_reference_array3311 = source_ml_source1_temporary_values1536.clone();
                    let ml_reference_position3311 = source_ml_source1_temporary_index1536.clone();
                    let ml_reference_projection3311 = self::ml_array::at(
                        &*ml_reference_array3311.borrow(),
                        ml_reference_position3311.to_index(),
                    );
                    ml_reference_projection3311
                };
                {
                    let count_2 = (count + {
                        let ml_reference_array3312 = match list.clone() {
                            self::JavaScriptCallArgumentList::CallArgumentList(
                                _,
                                _,
                                source_ml_source2_data_property_value42967,
                            ) => source_ml_source2_data_property_value42967.clone(),
                        };
                        let ml_reference_projection3312 =
                            ml_reference_array3312.borrow().len() as f64;
                        ml_reference_projection3312
                    });
                    {
                        let source_ml_source1_temporary_index1536_2 =
                            source_ml_source1_temporary_index1536.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_source1_temporary_index1536) =
                                (count_2, source_ml_source1_temporary_index1536_2.clone());
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_push_java_script_construction_body_loop1537(
    mut slots: f64,
    mut count: f64,
    mut source_ml_source1_temporary_values1535: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallFrame>>,
    >,
    mut source_ml_source1_temporary_index1535: self::ml::Big,
) -> self::MlSource1TemporaryPushJavaScriptConstructionBodyLoop1537Result {
    loop {
        return if (source_ml_source1_temporary_index1535
            < ({
                let ml_reference_array3313 = source_ml_source1_temporary_values1535.clone();
                let ml_reference_projection3313 =
                    self::ml::Big::from_u128(ml_reference_array3313.borrow().len() as u128);
                ml_reference_projection3313
            })) {
            {
                let frame = {
                    let ml_reference_array3314 = source_ml_source1_temporary_values1535.clone();
                    let ml_reference_position3314 = source_ml_source1_temporary_index1535.clone();
                    let ml_reference_projection3314 = self::ml_array::at(
                        &*ml_reference_array3314.borrow(),
                        ml_reference_position3314.to_index(),
                    );
                    ml_reference_projection3314
                };
                match frame.clone() {
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
                            let ml_reference_array3315 = shared_constructor_field1.clone();
                            let ml_reference_projection3315 =
                                ml_reference_array3315.borrow().len() as f64;
                            ml_reference_projection3315
                        });
                        {
                            let count_2 = (count + {
                                let ml_reference_array3316 = shared_constructor_field2.clone();
                                let ml_reference_projection3316 =
                                    ml_reference_array3316.borrow().len() as f64;
                                ml_reference_projection3316
                            });
                            {
                                let source_ml_source1_temporary_values1536 = (match frame.clone() {
                                    self::JavaScriptCallFrame::CallFrame(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value42972,
                                        _,
                                        _,
                                    ) => ((source_ml_source2_data_property_value42972
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
                                    let source_ml_source1_temporary_index1536 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let count_3 = self::source_ml_source1_temporary_push_java_script_construction_body_loop1538(count_2, source_ml_source1_temporary_values1536.clone(), source_ml_source1_temporary_index1536.clone());
                                        {
                                            let source_ml_source1_temporary_index1535_2 =
                                                source_ml_source1_temporary_index1535
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (
                                                    slots,
                                                    count,
                                                    source_ml_source1_temporary_index1535,
                                                ) = (
                                                    slots_2,
                                                    count_3,
                                                    source_ml_source1_temporary_index1535_2.clone(),
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
            self::MlSource1TemporaryPushJavaScriptConstructionBodyLoop1537Result::MlSource1TemporaryPushJavaScriptConstructionBodyLoop1537Done(slots, count)
        };
    }
}

#[path = "source_ml_source3_record_order.rs"]
pub mod source_ml_source3_record_order;
