// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAuthorityUpdate;
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructorRealm;
use super::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Result;
use super::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1423Result;
use super::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1424Result;
use super::find_java_script_array_constructor_fact;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::source_ml_source1_temporary_extend_java_script_call_array_authority_loop1424;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_extend_java_script_call_array_authority_loop1423(
    mut program: self::JavaScriptCallProgram,
    mut machine: self::JavaScriptCallMachine,
    mut directory: self::JavaScriptArraySpeciesDirectory,
    mut source_ml_source1_temporary_values1419: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptConstructorRealm>>,
    >,
    mut source_ml_source1_temporary_index1419: self::ml::Big,
) -> self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1423Result {
    loop {
        return if (source_ml_source1_temporary_index1419
            < ({
                let ml_reference_array3187 = source_ml_source1_temporary_values1419.clone();
                let ml_reference_projection3187 =
                    self::ml::Big::from_u128(ml_reference_array3187.borrow().len() as u128);
                ml_reference_projection3187
            })) {
            {
                let root = {
                    let ml_reference_array3188 = source_ml_source1_temporary_values1419.clone();
                    let ml_reference_position3188 = source_ml_source1_temporary_index1419.clone();
                    let ml_reference_projection3188 = self::ml_array::at(
                        &*ml_reference_array3188.borrow(),
                        ml_reference_position3188.to_index(),
                    );
                    ml_reference_projection3188
                };
                match root.clone() {
                    self::JavaScriptConstructorRealm::ConstructorRealm(
                        _,
                        shared_constructor_field1,
                        shared_constructor_field2,
                        shared_constructor_field3,
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
                        let shared_constructor_field1 = *shared_constructor_field1;
                        let shared_constructor_field2 = *shared_constructor_field2;
                        let shared_constructor_field3 = *shared_constructor_field3;
                        {
                            let next = {
                                let ml_values3189 = {
                                    let ml_reference_value3190 = ((match directory.clone() {
                                self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(_, _, _, _, _, source_ml_source2_data_property_value42139) => {
                                    ((source_ml_source2_data_property_value42139.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptConstructorRealm>>>>,))).0).clone()
                                }
                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptConstructorRealm>::new())))).borrow().clone();
                                    ml_reference_value3190
                                };
                                let ml_callback3189 = {
                                    let root = root.clone();
                                    std::rc::Rc::new(
                                        move |row: self::JavaScriptConstructorRealm| -> bool {
                                            ((match row.clone() {
                                self::JavaScriptConstructorRealm::ConstructorRealm(source_ml_source2_data_property_value42142, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value42142.clone()
                                }
                            }) == (match root.clone() {
                                self::JavaScriptConstructorRealm::ConstructorRealm(source_ml_source2_data_property_value42143, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value42143.clone()
                                }
                            }))
                                        },
                                    )
                                };
                                ml_values3189.iter().cloned().enumerate().find_map(
                                    |(ml_index3189, ml_value3189)| {
                                        if ml_callback3189(ml_value3189.clone()) {
                                            Some(ml_value3189)
                                        } else {
                                            None
                                        }
                                    },
                                )
                            };
                            if (((match (next.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } || !(self::test_java_script_semantic_same_value(
                                match next.clone().expect("a proven present value") {
                                    self::JavaScriptConstructorRealm::ConstructorRealm(
                                        _,
                                        source_ml_source2_data_property_value42144,
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
                                    ) => {
                                        let source_ml_source2_data_property_value42144 =
                                            *source_ml_source2_data_property_value42144;
                                        source_ml_source2_data_property_value42144.clone()
                                    }
                                },
                                shared_constructor_field1.clone(),
                            ))) || !(self::test_java_script_semantic_same_value(
                                match next.clone().expect("a proven present value") {
                                    self::JavaScriptConstructorRealm::ConstructorRealm(
                                        _,
                                        _,
                                        source_ml_source2_data_property_value42146,
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
                                    ) => {
                                        let source_ml_source2_data_property_value42146 =
                                            *source_ml_source2_data_property_value42146;
                                        source_ml_source2_data_property_value42146.clone()
                                    }
                                },
                                shared_constructor_field2.clone(),
                            ))) || !(self::test_java_script_semantic_same_value(
                                match next.clone().expect("a proven present value") {
                                    self::JavaScriptConstructorRealm::ConstructorRealm(
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value42148,
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
                                        let source_ml_source2_data_property_value42148 =
                                            *source_ml_source2_data_property_value42148;
                                        source_ml_source2_data_property_value42148.clone()
                                    }
                                },
                                shared_constructor_field3.clone(),
                            ))) {
                                self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1423Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1423Return(Box::new(self::JavaScriptArrayAuthorityUpdate::ArrayAuthorityUpdate(Box::new(program.clone()), Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 32, 112, 114, 101, 115, 101, 114, 118, 101, 115, 32, 101, 118, 101, 114, 121, 32, 101, 115, 116, 97, 98, 108, 105, 115, 104, 101, 100, 32, 114, 101, 97, 108, 109, 32, 114, 111, 111, 116]))))
                            } else {
                                {
                                    let index = 0f64;
                                    {
                                        let source_ml_source2_s42150 = self::source_ml_source1_temporary_extend_java_script_call_array_authority_loop1424(program.clone(), machine.clone(), root.clone(), next.clone().expect("a proven present value"), index);
                                        match source_ml_source2_s42150.clone() {
                                            self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1424Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1424Done => {
                                                {
                                                    let source_ml_source1_temporary_index1419_2 = source_ml_source1_temporary_index1419.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_source1_temporary_index1419 = source_ml_source1_temporary_index1419_2.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1424Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1424Return(source_ml_source1_temporary_result1424) => {
                                                let source_ml_source1_temporary_result1424 = *source_ml_source1_temporary_result1424;
                                                self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1423Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1423Return(Box::new(source_ml_source1_temporary_result1424.clone()))
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
            self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1423Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1423Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1422Result {
    MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1422Done,
    MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1422Return(
        Box<self::JavaScriptArrayAuthorityUpdate>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_extend_java_script_call_array_authority_loop1422(
    mut program: self::JavaScriptCallProgram,
    mut machine: self::JavaScriptCallMachine,
    mut directory: self::JavaScriptArraySpeciesDirectory,
    mut source_ml_source1_temporary_values1418: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptArrayConstructorFact>>,
    >,
    mut source_ml_source1_temporary_index1418: self::ml::Big,
) -> self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1422Result {
    loop {
        return match directory.clone() {
            self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(
                _,
                _,
                _,
                _,
                shared_constructor_field5,
                _,
            ) => {
                if (source_ml_source1_temporary_index1418
                    < ({
                        let ml_reference_array3191 = source_ml_source1_temporary_values1418.clone();
                        let ml_reference_projection3191 =
                            self::ml::Big::from_u128(ml_reference_array3191.borrow().len() as u128);
                        ml_reference_projection3191
                    }))
                {
                    {
                        let fact = {
                            let ml_reference_array3192 =
                                source_ml_source1_temporary_values1418.clone();
                            let ml_reference_position3192 =
                                source_ml_source1_temporary_index1418.clone();
                            let ml_reference_projection3192 = self::ml_array::at(
                                &*ml_reference_array3192.borrow(),
                                ml_reference_position3192.to_index(),
                            );
                            ml_reference_projection3192
                        };
                        match fact.clone() {
                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                shared_constructor_field1,
                                shared_constructor_field2,
                                shared_constructor_field3,
                                shared_constructor_field4,
                                _,
                            ) => {
                                let next = {
                                    let source_ml_source2_array_read_value42156 =
                                        shared_constructor_field5.clone();
                                    {
                                        let source_ml_source2_array_read_index42157 =
                                            self::find_java_script_array_constructor_fact(
                                                shared_constructor_field5.clone(),
                                                shared_constructor_field1.clone(),
                                            );
                                        {
                                            let ml_reference_array3193 =
                                                source_ml_source2_array_read_value42156.clone();
                                            let ml_reference_position3193 =
                                                source_ml_source2_array_read_index42157;
                                            let ml_reference_projection3193 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array3193.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position3193,
                                                    ),
                                                );
                                            ml_reference_projection3193
                                        }
                                    }
                                };
                                if ((((match (next.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match next.clone().expect("a proven present value") {
                                        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value42160, _, _, _) => {
                                            source_ml_source2_data_property_value42160
                                        }
                                    }) != shared_constructor_field2)) || ((match next.clone().expect("a proven present value") {
                                        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value42164, _, _) => {
                                            source_ml_source2_data_property_value42164.clone()
                                        }
                                    }) != shared_constructor_field3)) || ((match next.clone().expect("a proven present value") {
                                        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, source_ml_source2_data_property_value42168, _) => {
                                            source_ml_source2_data_property_value42168
                                        }
                                    }) != shared_constructor_field4)) || {
                                        let source_ml_source2_comparison_operand42172 = match next.clone().expect("a proven present value") {
                                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, _, source_ml_source2_data_property_value42170) => {
                                                ((source_ml_source2_data_property_value42170.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_comparison_operand42173 = match fact.clone() {
                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, _, source_ml_source2_data_property_value42171) => {
                                                    ((source_ml_source2_data_property_value42171.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                }
                                            };
                                            !(match (source_ml_source2_comparison_operand42172.clone(), source_ml_source2_comparison_operand42173.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                                        }
                                    }) {
                                        self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1422Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1422Return(Box::new(self::JavaScriptArrayAuthorityUpdate::ArrayAuthorityUpdate(Box::new(program.clone()), Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 32, 112, 114, 101, 115, 101, 114, 118, 101, 115, 32, 101, 118, 101, 114, 121, 32, 101, 115, 116, 97, 98, 108, 105, 115, 104, 101, 100, 32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110]))))
                                    } else {
                                        {
                                            let source_ml_source1_temporary_index1418_2 = source_ml_source1_temporary_index1418.add(&self::ml::Big::from_i128(1));
                                            {
                                                source_ml_source1_temporary_index1418 = source_ml_source1_temporary_index1418_2.clone();
                                                continue;
                                            }
                                        }
                                    }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1422Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1422Done
                }
            }
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_extend_java_script_call_array_authority_loop1421(
    mut program: self::JavaScriptCallProgram,
    mut machine: self::JavaScriptCallMachine,
    mut directory: self::JavaScriptArraySpeciesDirectory,
    mut source_ml_source1_temporary_values1417: std::rc::Rc<
        std::cell::RefCell<Vec<Option<self::JavaScriptArraySpeciesDirectory>>>,
    >,
    mut source_ml_source1_temporary_index1417: self::ml::Big,
) -> self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Result {
    loop {
        return match directory.clone() {
            self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(
                shared_constructor_field1,
                shared_constructor_field2,
                shared_constructor_field3,
                shared_constructor_field4,
                _,
                _,
            ) => {
                let shared_constructor_field2 = *shared_constructor_field2;
                let shared_constructor_field3 = *shared_constructor_field3;
                let shared_constructor_field4 = *shared_constructor_field4;
                if (source_ml_source1_temporary_index1417
                    < ({
                        let ml_reference_array3194 = source_ml_source1_temporary_values1417.clone();
                        let ml_reference_projection3194 =
                            self::ml::Big::from_u128(ml_reference_array3194.borrow().len() as u128);
                        ml_reference_projection3194
                    }))
                {
                    {
                        let retained = {
                            let ml_reference_array3195 =
                                source_ml_source1_temporary_values1417.clone();
                            let ml_reference_position3195 =
                                source_ml_source1_temporary_index1417.clone();
                            let ml_reference_projection3195 = self::ml_array::at(
                                &*ml_reference_array3195.borrow(),
                                ml_reference_position3195.to_index(),
                            );
                            ml_reference_projection3195
                        };
                        if ((((shared_constructor_field1
                            != ({
                                let source_ml_source2_nullish_field_receiver42181 =
                                    (retained.clone(),);
                                if match (source_ml_source2_nullish_field_receiver42181.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present.clone();
                                        true
                                    }
                                    None => false,
                                } {
                                    {
                                        let source_ml_source2_nullish_field_present42182 =
                                            ((source_ml_source2_nullish_field_receiver42181
                                                .clone())
                                            .0)
                                                .clone()
                                                .expect("a proven present value");
                                        match source_ml_source2_nullish_field_present42182.clone() {
                                        self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(source_ml_source2_data_property_value42183, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value42183.clone()
                                        }
                                    }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException {
                                        class_name: String::from("TypeError"),
                                        message_units: (self::ml_string::Text::from_units(vec![
                                            67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32,
                                            112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32,
                                            111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101,
                                            100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39,
                                            114, 101, 97, 108, 109, 39, 41,
                                        ]))
                                        .0,
                                        message_present: true,
                                    })
                                    .raise()
                                }
                            }))
                            || !(self::test_java_script_semantic_same_value(
                                shared_constructor_field2.clone(),
                                {
                                    let source_ml_source2_nullish_field_receiver42185 =
                                        (retained.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver42185.clone())
                                        .0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source2_nullish_field_present42186 =
                                                ((source_ml_source2_nullish_field_receiver42185
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source2_nullish_field_present42186.clone() {
                                        self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(_, source_ml_source2_data_property_value42187, _, _, _, _) => {
                                            let source_ml_source2_data_property_value42187 = *source_ml_source2_data_property_value42187;
                                            source_ml_source2_data_property_value42187.clone()
                                        }
                                    }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException {
                                            class_name: String::from("TypeError"),
                                            message_units: (self::ml_string::Text::from_units(
                                                vec![
                                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97,
                                                    100, 32, 112, 114, 111, 112, 101, 114, 116,
                                                    105, 101, 115, 32, 111, 102, 32, 117, 110, 100,
                                                    101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                                    97, 100, 105, 110, 103, 32, 39, 97, 114, 114,
                                                    97, 121, 67, 111, 110, 115, 116, 114, 117, 99,
                                                    116, 111, 114, 39, 41,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    }
                                },
                            )))
                            || !(self::test_java_script_semantic_same_value(
                                shared_constructor_field3.clone(),
                                {
                                    let source_ml_source2_nullish_field_receiver42189 =
                                        (retained.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver42189.clone())
                                        .0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source2_nullish_field_present42190 =
                                                ((source_ml_source2_nullish_field_receiver42189
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source2_nullish_field_present42190.clone() {
                                        self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(_, _, source_ml_source2_data_property_value42191, _, _, _) => {
                                            let source_ml_source2_data_property_value42191 = *source_ml_source2_data_property_value42191;
                                            source_ml_source2_data_property_value42191.clone()
                                        }
                                    }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException {
                                            class_name: String::from("TypeError"),
                                            message_units: (self::ml_string::Text::from_units(
                                                vec![
                                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97,
                                                    100, 32, 112, 114, 111, 112, 101, 114, 116,
                                                    105, 101, 115, 32, 111, 102, 32, 117, 110, 100,
                                                    101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                                    97, 100, 105, 110, 103, 32, 39, 97, 114, 114,
                                                    97, 121, 80, 114, 111, 116, 111, 116, 121, 112,
                                                    101, 39, 41,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    }
                                },
                            )))
                            || !(self::test_java_script_semantic_same_value(
                                shared_constructor_field4.clone(),
                                {
                                    let source_ml_source2_nullish_field_receiver42193 =
                                        (retained.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver42193.clone())
                                        .0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source2_nullish_field_present42194 =
                                                ((source_ml_source2_nullish_field_receiver42193
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source2_nullish_field_present42194.clone() {
                                        self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(_, _, _, source_ml_source2_data_property_value42195, _, _) => {
                                            let source_ml_source2_data_property_value42195 = *source_ml_source2_data_property_value42195;
                                            source_ml_source2_data_property_value42195.clone()
                                        }
                                    }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException {
                                            class_name: String::from("TypeError"),
                                            message_units: (self::ml_string::Text::from_units(
                                                vec![
                                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97,
                                                    100, 32, 112, 114, 111, 112, 101, 114, 116,
                                                    105, 101, 115, 32, 111, 102, 32, 117, 110, 100,
                                                    101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                                    97, 100, 105, 110, 103, 32, 39, 115, 112, 101,
                                                    99, 105, 101, 115, 75, 101, 121, 39, 41,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    }
                                },
                            )))
                        {
                            self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Return(Box::new(self::JavaScriptArrayAuthorityUpdate::ArrayAuthorityUpdate(Box::new(program.clone()), Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 101, 120, 105, 115, 116, 105, 110, 103, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 111, 111, 116, 115]))))
                        } else {
                            {
                                let source_ml_source1_temporary_values1418 = {
                                    let source_ml_source2_nullish_field_receiver42196 =
                                        (retained.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver42196.clone())
                                        .0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source2_nullish_field_present42197 =
                                                ((source_ml_source2_nullish_field_receiver42196
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source2_nullish_field_present42197.clone() {
                                                self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(_, _, _, _, source_ml_source2_data_property_value42198, _) => {
                                                    source_ml_source2_data_property_value42198.clone()
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException {
                                            class_name: String::from("TypeError"),
                                            message_units: (self::ml_string::Text::from_units(
                                                vec![
                                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97,
                                                    100, 32, 112, 114, 111, 112, 101, 114, 116,
                                                    105, 101, 115, 32, 111, 102, 32, 117, 110, 100,
                                                    101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                                    97, 100, 105, 110, 103, 32, 39, 99, 111, 110,
                                                    115, 116, 114, 117, 99, 116, 111, 114, 115, 39,
                                                    41,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    }
                                };
                                {
                                    let source_ml_source1_temporary_index1418 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s42199 = self::source_ml_source1_temporary_extend_java_script_call_array_authority_loop1422(program.clone(), machine.clone(), directory.clone(), source_ml_source1_temporary_values1418.clone(), source_ml_source1_temporary_index1418.clone());
                                        match source_ml_source2_s42199.clone() {
                                            self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1422Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1422Done => {
                                                {
                                                    let source_ml_source1_temporary_values1419 = ({
                                                        let source_ml_source2_nullish_field_receiver42200 = (retained.clone(),);
                                                        if match (source_ml_source2_nullish_field_receiver42200.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                            {
                                                                let source_ml_source2_nullish_field_present42201 = ((source_ml_source2_nullish_field_receiver42200.clone()).0).clone().expect("a proven present value");
                                                                match source_ml_source2_nullish_field_present42201.clone() {
                                                                    self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(_, _, _, _, _, source_ml_source2_data_property_value42202) => {
                                                                        ((source_ml_source2_data_property_value42202.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptConstructorRealm>>>>,))).0).clone()
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 82, 101, 97, 108, 109, 115, 39, 41])).0, message_present: true }).raise()
                                                        }
                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptConstructorRealm>::new())));
                                                    {
                                                        let source_ml_source1_temporary_index1419 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source2_s42203 = self::source_ml_source1_temporary_extend_java_script_call_array_authority_loop1423(program.clone(), machine.clone(), directory.clone(), source_ml_source1_temporary_values1419.clone(), source_ml_source1_temporary_index1419.clone());
                                                            match source_ml_source2_s42203.clone() {
                                                                self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1423Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1423Done => {
                                                                    {
                                                                        let source_ml_source1_temporary_index1417_2 = source_ml_source1_temporary_index1417.add(&self::ml::Big::from_i128(1));
                                                                        {
                                                                            source_ml_source1_temporary_index1417 = source_ml_source1_temporary_index1417_2.clone();
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                                self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1423Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1423Return(source_ml_source1_temporary_result1423) => {
                                                                    let source_ml_source1_temporary_result1423 = *source_ml_source1_temporary_result1423;
                                                                    self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Return(Box::new(source_ml_source1_temporary_result1423.clone()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1422Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1422Return(source_ml_source1_temporary_result1424) => {
                                                let source_ml_source1_temporary_result1424 = *source_ml_source1_temporary_result1424;
                                                self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Return(Box::new(source_ml_source1_temporary_result1424.clone()))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryExtendJavaScriptCallArrayAuthorityJoin1420 {
    MlSource1TemporaryExtendJavaScriptCallArrayAuthorityJoin1420Next,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyLoop1435Result {
    MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyLoop1435Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_push_java_script_species_constructor_body_loop1435(
    mut slots: f64,
    mut arguments_count: f64,
    mut source_ml_source1_temporary_values1433: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallFrame>>,
    >,
    mut source_ml_source1_temporary_index1433: self::ml::Big,
) -> self::MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyLoop1435Result {
    loop {
        return if (source_ml_source1_temporary_index1433
            < ({
                let ml_reference_array3196 = source_ml_source1_temporary_values1433.clone();
                let ml_reference_projection3196 =
                    self::ml::Big::from_u128(ml_reference_array3196.borrow().len() as u128);
                ml_reference_projection3196
            })) {
            {
                let frame = {
                    let ml_reference_array3197 = source_ml_source1_temporary_values1433.clone();
                    let ml_reference_position3197 = source_ml_source1_temporary_index1433.clone();
                    let ml_reference_projection3197 = self::ml_array::at(
                        &*ml_reference_array3197.borrow(),
                        ml_reference_position3197.to_index(),
                    );
                    ml_reference_projection3197
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
                            let ml_reference_array3198 = shared_constructor_field1.clone();
                            let ml_reference_projection3198 =
                                ml_reference_array3198.borrow().len() as f64;
                            ml_reference_projection3198
                        });
                        {
                            let arguments_count_2 = (arguments_count + {
                                let ml_reference_array3199 = shared_constructor_field2.clone();
                                let ml_reference_projection3199 =
                                    ml_reference_array3199.borrow().len() as f64;
                                ml_reference_projection3199
                            });
                            {
                                let source_ml_source1_temporary_index1433_2 =
                                    source_ml_source1_temporary_index1433
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (
                                        slots,
                                        arguments_count,
                                        source_ml_source1_temporary_index1433,
                                    ) = (
                                        slots_2,
                                        arguments_count_2,
                                        source_ml_source1_temporary_index1433_2.clone(),
                                    );
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyLoop1435Result::MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyLoop1435Done(slots, arguments_count)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyJoin1434 {
    MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyJoin1434Next,
    MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyJoin1434Return(
        Box<self::JavaScriptCallMachineTransition>,
    ),
}
