// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::source_ml_source3_record_order;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_species_construction_loop1112(
    mut properties: f64,
    mut source_ml_source1_temporary_values1111: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index1111: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index1111
            < ({
                let ml_reference_array2951 = source_ml_source1_temporary_values1111.clone();
                let ml_reference_projection2951 =
                    self::ml::Big::from_u128(ml_reference_array2951.borrow().len() as u128);
                ml_reference_projection2951
            })) {
            {
                let object = {
                    let ml_reference_array2952 = source_ml_source1_temporary_values1111.clone();
                    let ml_reference_position2952 = source_ml_source1_temporary_index1111.clone();
                    let ml_reference_projection2952 = self::ml_array::at(
                        &*ml_reference_array2952.borrow(),
                        ml_reference_position2952.to_index(),
                    );
                    ml_reference_projection2952
                };
                {
                    let properties_2 = (properties + {
                        let ml_reference_array2953 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41501,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41501.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41502,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41502.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41503,
                            ) => source_ml_source2_data_property_value41503.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41504,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41504.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41505,
                                _,
                            ) => source_ml_source2_data_property_value41505.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41506,
                                _,
                            ) => source_ml_source2_data_property_value41506.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41507,
                                _,
                            ) => source_ml_source2_data_property_value41507.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41508,
                                _,
                            ) => source_ml_source2_data_property_value41508.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41509,
                            ) => source_ml_source2_data_property_value41509.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41510,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41510.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41511,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41511.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41512,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41512.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41513,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41513.clone(),
                        };
                        let ml_reference_projection2953 =
                            ml_reference_array2953.borrow().len() as f64;
                        ml_reference_projection2953
                    });
                    {
                        let source_ml_source1_temporary_index1111_2 =
                            source_ml_source1_temporary_index1111.add(&self::ml::Big::from_i128(1));
                        {
                            (properties, source_ml_source1_temporary_index1111) = (
                                properties_2,
                                source_ml_source1_temporary_index1111_2.clone(),
                            );
                            continue;
                        }
                    }
                }
            }
        } else {
            properties
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorJoin1121 {
    MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorJoin1121Next,
    MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorJoin1121Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorLoop1122Result {
    MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorLoop1122Done,
    MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorLoop1122Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_constructor_frame_operands_error_loop1122(
    mut parent: self::JavaScriptCallFrame,
    mut frame: self::JavaScriptCallFrame,
    mut argument_registers: Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorLoop1122Result {
    loop {
        return if (at
            < ({
                let source_ml_source2_nullish_field_receiver41516 = (argument_registers.clone(),);
                if match (source_ml_source2_nullish_field_receiver41516.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    {
                        let source_ml_source2_nullish_field_present41517 =
                            ((source_ml_source2_nullish_field_receiver41516.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        {
                            let ml_reference_array2954 =
                                source_ml_source2_nullish_field_present41517.clone();
                            let ml_reference_projection2954 =
                                ml_reference_array2954.borrow().len() as f64;
                            ml_reference_projection2954
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
                    let source_ml_source2_array_read_value41524 = match parent.clone() {
                        self::JavaScriptCallFrame::CallFrame(
                            _,
                            _,
                            source_ml_source2_data_property_value41518,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value41518.clone(),
                    };
                    {
                        let source_ml_source2_array_read_index41525 = ({
                            let source_ml_source2_nullish_field_receiver41519 =
                                (argument_registers.clone(), at);
                            {
                                let source_ml_source2_computed_property_key41520 =
                                    (source_ml_source2_nullish_field_receiver41519.clone()).1;
                                if match (source_ml_source2_nullish_field_receiver41519.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present.clone();
                                        true
                                    }
                                    None => false,
                                } {
                                    {
                                        let source_ml_source2_nullish_field_present41521 =
                                            ((source_ml_source2_nullish_field_receiver41519
                                                .clone())
                                            .0)
                                                .clone()
                                                .expect("a proven present value");
                                        {
                                            let ml_reference_array2955 =
                                                source_ml_source2_nullish_field_present41521
                                                    .clone();
                                            let ml_reference_position2955 =
                                                source_ml_source2_computed_property_key41520;
                                            let ml_reference_projection2955 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array2955.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position2955,
                                                    ),
                                                );
                                            ml_reference_projection2955
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException {
                                        class_name: String::from("TypeError"),
                                        message_units: (((self::ml_string::Text::from_units(
                                            vec![
                                                67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100,
                                                32, 112, 114, 111, 112, 101, 114, 116, 105, 101,
                                                115, 32, 111, 102, 32, 117, 110, 100, 101, 102,
                                                105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105,
                                                110, 103, 32, 39,
                                            ],
                                        ))
                                        .concat(&self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                (source_ml_source2_nullish_field_receiver41519
                                                    .clone())
                                                .1,
                                            ),
                                        )))
                                        .concat(&self::ml_string::Text::from_units(vec![39, 41])))
                                        .0,
                                        message_present: true,
                                    })
                                    .raise()
                                }
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| (-1f64));
                        {
                            let ml_reference_array2956 =
                                source_ml_source2_array_read_value41524.clone();
                            let ml_reference_position2956 = source_ml_source2_array_read_index41525;
                            let ml_reference_projection2956 = self::ml_array::optional_at(
                                &*ml_reference_array2956.borrow(),
                                self::ml_array::number_index(ml_reference_position2956),
                            );
                            ml_reference_projection2956
                        }
                    }
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
                ({
                    let source_ml_source2_array_read_value41527 = match frame.clone() {
                        self::JavaScriptCallFrame::CallFrame(
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value41526,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value41526.clone(),
                    };
                    {
                        let ml_reference_array2957 =
                            source_ml_source2_array_read_value41527.clone();
                        let ml_reference_position2957 = at;
                        let ml_reference_projection2957 = self::ml_array::optional_at(
                            &*ml_reference_array2957.borrow(),
                            self::ml_array::number_index(ml_reference_position2957),
                        );
                        ml_reference_projection2957
                    }
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
            )) {
                self::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorLoop1122Result::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorLoop1122Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 102, 114, 97, 109, 101, 115, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 101, 114, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
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
            self::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorLoop1122Result::MlSource1TemporaryReadJavaScriptConstructorFrameOperandsErrorLoop1122Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptDescriptorOperandsLoop1123Result {
    MlSource1TemporaryReadJavaScriptDescriptorOperandsLoop1123Done,
    MlSource1TemporaryReadJavaScriptDescriptorOperandsLoop1123Return(
        std::rc::Rc<
            std::cell::RefCell<(
                (
                    std::rc::Rc<
                        std::cell::RefCell<(
                            (
                                bool,
                                bool,
                                bool,
                                bool,
                                self::JavaScriptSemanticValue,
                                bool,
                                self::JavaScriptSemanticValue,
                                bool,
                                self::JavaScriptSemanticValue,
                                bool,
                                bool,
                                bool,
                            ),
                            Vec<f64>,
                        )>,
                    >,
                    self::ml_string::Text,
                ),
                Vec<f64>,
            )>,
        >,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124 {
    MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1127 {
    MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1127Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1130 {
    MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1130Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1133 {
    MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1133Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1136 {
    MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1136Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1139 {
    MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1139Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1142 {
    MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1142Next,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_descriptor_operands_loop1123(
    mut operands: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut descriptor: std::rc::Rc<
        std::cell::RefCell<(
            (
                bool,
                bool,
                bool,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                bool,
                bool,
            ),
            Vec<f64>,
        )>,
    >,
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptDescriptorOperandsLoop1123Result {
    loop {
        return if (index < (6f64)) {
            {
                let presence = {
                    let source_ml_source2_array_read_index41530 = (index * 2f64);
                    {
                        let ml_reference_array2958 = operands.clone();
                        let ml_reference_position2958 = source_ml_source2_array_read_index41530;
                        let ml_reference_projection2958 = self::ml_array::optional_at(
                            &*ml_reference_array2958.borrow(),
                            self::ml_array::number_index(ml_reference_position2958),
                        );
                        ml_reference_projection2958
                    }
                };
                {
                    let value = {
                        let source_ml_source2_array_read_index41532 = ((index * 2f64) + 1f64);
                        {
                            let ml_reference_array2959 = operands.clone();
                            let ml_reference_position2959 = source_ml_source2_array_read_index41532;
                            let ml_reference_projection2959 = self::ml_array::optional_at(
                                &*ml_reference_array2959.borrow(),
                                self::ml_array::number_index(ml_reference_position2959),
                            );
                            ml_reference_projection2959
                        }
                    };
                    if ((match (presence.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } || match presence.clone().expect("a proven present value") {
                        self::JavaScriptSemanticValue::Boolean(_) => false,
                        _ => true,
                    }) || match (value.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        self::MlSource1TemporaryReadJavaScriptDescriptorOperandsLoop1123Result::MlSource1TemporaryReadJavaScriptDescriptorOperandsLoop1123Return(std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source3_record_field558 = descriptor.clone();
                            {
                                let source_ml_source3_record_field559 = self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 111, 112, 101, 114, 97, 110, 100, 32, 112, 114, 101, 115, 101, 110, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 66, 111, 111, 108, 101, 97, 110, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 112, 97, 121, 108, 111, 97, 100]);
                                ((source_ml_source3_record_field558.clone(), source_ml_source3_record_field559.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![100f64, 101f64, 115f64, 99f64, 114f64, 105f64, 112f64, 116f64, 111f64, 114f64], vec![101f64, 114f64, 114f64, 111f64, 114f64]], vec![0f64, 1f64]))
                            }
                        })))
                    } else {
                        {
                            let present =
                                ((match presence.clone().expect("a proven present value") {
                                    self::JavaScriptSemanticValue::Boolean(
                                        source_ml_source2_data_property_value41536,
                                    ) => Some((source_ml_source2_data_property_value41536,)),
                                    _ => None::<(bool,)>,
                                })
                                .clone()
                                .expect("a proven present value"))
                                .0;
                            {
                                let source_ml_source2_s41555 = if (index == (0f64)) {
                                    {
                                        let source_ml_source1_temporary_read_java_script_descriptor_operands_written1141 = {
                                            let ml_field_owner2960 = descriptor.clone();
                                            let ml_field_value2960 = present;
                                            ml_field_owner2960.borrow_mut().0.1 =
                                                ml_field_value2960;
                                            ()
                                        };
                                        {
                                            let source_ml_source2_s41558 = if (present
                                                && match value
                                                    .clone()
                                                    .expect("a proven present value")
                                                {
                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                        true
                                                    }
                                                    _ => false,
                                                }) {
                                                {
                                                    let source_ml_source1_temporary_read_java_script_descriptor_operands_written1143 = {
                                                        let ml_field_owner2961 = descriptor.clone();
                                                        let ml_field_value2961 = ((match value.clone().expect("a proven present value") {
                                                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_data_property_value41560) => {
                                                            Some((source_ml_source2_data_property_value41560,))
                                                        }
                                                        _ => {
                                                            None::<(bool,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0;
                                                        ml_field_owner2961.borrow_mut().0.0 =
                                                            ml_field_value2961;
                                                        ()
                                                    };
                                                    self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1142::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1142Next
                                                }
                                            } else {
                                                self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1142::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1142Next
                                            };
                                            match source_ml_source2_s41558.clone() {
                                                self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1142::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1142Next => {
                                                    self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124Next
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if (index == (1f64)) {
                                        {
                                            let source_ml_source1_temporary_read_java_script_descriptor_operands_written1138 = {
                                                let ml_field_owner2962 = descriptor.clone();
                                                let ml_field_value2962 = present;
                                                ml_field_owner2962.borrow_mut().0.3 =
                                                    ml_field_value2962;
                                                ()
                                            };
                                            {
                                                let source_ml_source2_s41563 = if (present
                                                    && match value
                                                        .clone()
                                                        .expect("a proven present value")
                                                    {
                                                        self::JavaScriptSemanticValue::Boolean(
                                                            _,
                                                        ) => true,
                                                        _ => false,
                                                    }) {
                                                    {
                                                        let source_ml_source1_temporary_read_java_script_descriptor_operands_written1140 = {
                                                            let ml_field_owner2963 =
                                                                descriptor.clone();
                                                            let ml_field_value2963 = ((match value.clone().expect("a proven present value") {
                                                            self::JavaScriptSemanticValue::Boolean(source_ml_source2_data_property_value41565) => {
                                                                Some((source_ml_source2_data_property_value41565,))
                                                            }
                                                            _ => {
                                                                None::<(bool,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0;
                                                            ml_field_owner2963.borrow_mut().0.2 =
                                                                ml_field_value2963;
                                                            ()
                                                        };
                                                        self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1139::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1139Next
                                                    }
                                                } else {
                                                    self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1139::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1139Next
                                                };
                                                match source_ml_source2_s41563.clone() {
                                                    self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1139::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1139Next => {
                                                        self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124Next
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if (index == (2f64)) {
                                            {
                                                let source_ml_source1_temporary_read_java_script_descriptor_operands_written1135 = {
                                                    let ml_field_owner2964 = descriptor.clone();
                                                    let ml_field_value2964 = present;
                                                    ml_field_owner2964.borrow_mut().0.9 =
                                                        ml_field_value2964;
                                                    ()
                                                };
                                                {
                                                    let source_ml_source2_s41566 = if present {
                                                        {
                                                            let source_ml_source1_temporary_read_java_script_descriptor_operands_written1137 = {
                                                                let ml_field_owner2965 =
                                                                    descriptor.clone();
                                                                let ml_field_value2965 =
                                                                    value.clone().expect(
                                                                        "a proven present value",
                                                                    );
                                                                ml_field_owner2965
                                                                    .borrow_mut()
                                                                    .0
                                                                    .8 = ml_field_value2965;
                                                                ()
                                                            };
                                                            self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1136::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1136Next
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1136::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1136Next
                                                    };
                                                    match source_ml_source2_s41566.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1136::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1136Next => {
                                                            self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124Next
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if (index == (3f64)) {
                                                {
                                                    let source_ml_source1_temporary_read_java_script_descriptor_operands_written1132 = {
                                                        let ml_field_owner2966 = descriptor.clone();
                                                        let ml_field_value2966 = present;
                                                        ml_field_owner2966.borrow_mut().0.11 =
                                                            ml_field_value2966;
                                                        ()
                                                    };
                                                    {
                                                        let source_ml_source2_s41569 = if (present && match value.clone().expect("a proven present value") {
                                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                                true
                                                            }
                                                            _ => {
                                                                false
                                                            }
                                                        }) {
                                                            {
                                                                let source_ml_source1_temporary_read_java_script_descriptor_operands_written1134 = { let ml_field_owner2967 = descriptor.clone(); let ml_field_value2967 = ((match value.clone().expect("a proven present value") {
                                                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_data_property_value41571) => {
                                                                        Some((source_ml_source2_data_property_value41571,))
                                                                    }
                                                                    _ => {
                                                                        None::<(bool,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0; ml_field_owner2967.borrow_mut().0.10 = ml_field_value2967; () };
                                                                self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1133::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1133Next
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1133::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1133Next
                                                        };
                                                        match source_ml_source2_s41569.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1133::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1133Next => {
                                                                self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124Next
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if (index == (4f64)) {
                                                    {
                                                        let source_ml_source1_temporary_read_java_script_descriptor_operands_written1129 = {
                                                            let ml_field_owner2968 =
                                                                descriptor.clone();
                                                            let ml_field_value2968 = present;
                                                            ml_field_owner2968.borrow_mut().0.5 =
                                                                ml_field_value2968;
                                                            ()
                                                        };
                                                        {
                                                            let source_ml_source2_s41572 =
                                                                if present {
                                                                    {
                                                                        let source_ml_source1_temporary_read_java_script_descriptor_operands_written1131 = {
                                                                            let ml_field_owner2969 =
                                                                                descriptor.clone();
                                                                            let ml_field_value2969 = value.clone().expect("a proven present value");
                                                                            ml_field_owner2969
                                                                                .borrow_mut()
                                                                                .0
                                                                                .4 =
                                                                                ml_field_value2969;
                                                                            ()
                                                                        };
                                                                        self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1130::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1130Next
                                                                    }
                                                                } else {
                                                                    self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1130::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1130Next
                                                                };
                                                            match source_ml_source2_s41572.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1130::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1130Next => {
                                                                    self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124Next
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if (index == (5f64)) {
                                                        {
                                                            let source_ml_source1_temporary_read_java_script_descriptor_operands_written1126 = {
                                                                let ml_field_owner2970 =
                                                                    descriptor.clone();
                                                                let ml_field_value2970 = present;
                                                                ml_field_owner2970
                                                                    .borrow_mut()
                                                                    .0
                                                                    .7 = ml_field_value2970;
                                                                ()
                                                            };
                                                            {
                                                                let source_ml_source2_s41573 =
                                                                    if present {
                                                                        {
                                                                            let source_ml_source1_temporary_read_java_script_descriptor_operands_written1128 = {
                                                                                let ml_field_owner2971 =
                                                                                    descriptor
                                                                                        .clone();
                                                                                let ml_field_value2971 = value.clone().expect("a proven present value");
                                                                                ml_field_owner2971.borrow_mut().0.6 = ml_field_value2971;
                                                                                ()
                                                                            };
                                                                            self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1127::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1127Next
                                                                        }
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1127::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1127Next
                                                                    };
                                                                match source_ml_source2_s41573.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1127::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1127Next => {
                                                                        self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124Next
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124Next
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                                match source_ml_source2_s41555.clone() {
                                    self::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124::MlSource1TemporaryReadJavaScriptDescriptorOperandsJoin1124Next => {
                                        if ((present && (((index == (0f64)) || (index == (1f64))) || (index == (3f64)))) && match value.clone().expect("a proven present value") {
                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) {
                                            self::MlSource1TemporaryReadJavaScriptDescriptorOperandsLoop1123Result::MlSource1TemporaryReadJavaScriptDescriptorOperandsLoop1123Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source3_record_field560 = descriptor.clone();
                                                {
                                                    let source_ml_source3_record_field561 = self::ml_string::Text::from_units(vec![112, 114, 101, 115, 101, 110, 116, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 66, 111, 111, 108, 101, 97, 110, 115]);
                                                    ((source_ml_source3_record_field560.clone(), source_ml_source3_record_field561.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![100f64, 101f64, 115f64, 99f64, 114f64, 105f64, 112f64, 116f64, 111f64, 114f64], vec![101f64, 114f64, 114f64, 111f64, 114f64]], vec![0f64, 1f64]))
                                                }
                                            })))
                                        } else {
                                            {
                                                let index_2 = (index + 1f64);
                                                {
                                                    index = index_2;
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
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptDescriptorOperandsLoop1123Result::MlSource1TemporaryReadJavaScriptDescriptorOperandsLoop1123Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1149Result {
    MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1149Done,
    MlSource1TemporaryReadJavaScriptFunctionSourceDirectoryEntriesErrorLoop1149Return(
        self::ml_string::Text,
    ),
}
