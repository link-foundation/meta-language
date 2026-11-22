// Generated from complete js/src/translation/javascript-collection-iterator-results.js. Do not edit by hand.
use super::JavaScriptArrayPropertyPlan;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Result;
use super::MlTemporaryTestJavaScriptCollectionKeyJoin32;
use super::find_java_script_array_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_array_length_definition;
use super::plan_java_script_array_ordinary_definition;
use super::read_java_script_array_index;
use super::read_java_script_array_length;
use super::read_java_script_array_length_key;
use super::source_ml_temporary_plan_java_script_array_property_definition_loop30;
use super::test_java_script_array_length_key;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_java_script_array_property_definition(
    object: self::JavaScriptOrdinaryObject,
    key: self::JavaScriptSemanticValue,
    proposed: std::rc::Rc<
        std::cell::RefCell<(
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
        )>,
    >,
) -> self::JavaScriptArrayPropertyPlan {
    if self::test_java_script_array_length_key(key.clone()) {
        if (({ proposed.borrow().clone() }).9) {
            {
                let value = ({ proposed.borrow().clone() }).8;
                match value {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            self::ml_string::Text::from_units(vec![
                                97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114,
                                101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114,
                                105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                                99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                            ]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            self::ml_string::Text::from_units(vec![
                                97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114,
                                101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114,
                                105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                                99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                            ]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            self::ml_string::Text::from_units(vec![
                                97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114,
                                101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114,
                                105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                                99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                            ]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    }
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        self::plan_java_script_array_length_definition(
                            object.clone(),
                            proposed.clone(),
                            self::convert_java_script_array_unsigned_length(value_value),
                            value_value,
                        )
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            self::ml_string::Text::from_units(vec![
                                97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114,
                                101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114,
                                105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                                99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                            ]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            self::ml_string::Text::from_units(vec![
                                97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114,
                                101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114,
                                105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                                99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                            ]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    }
                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            self::ml_string::Text::from_units(vec![
                                97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114,
                                101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114,
                                105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                                99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                            ]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            self::ml_string::Text::from_units(vec![
                                97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114,
                                101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114,
                                105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                                99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                            ]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            self::ml_string::Text::from_units(vec![
                                97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114,
                                101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114,
                                105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                                99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                            ]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    }
                }
            }
        } else {
            self::plan_java_script_array_ordinary_definition(
                object.clone(),
                key.clone(),
                proposed.clone(),
            )
        }
    } else {
        {
            let index = self::read_java_script_array_index(key.clone());
            {
                let length = self::read_java_script_array_length(object.clone());
                {
                    let length_property = {
                        let source_ml_source1_array_read_value1189 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value1163,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value1163.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value1164,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value1164.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value1165,
                            ) => source_ml_source1_data_property_value1165.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value1166,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value1166.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value1167,
                                _,
                            ) => source_ml_source1_data_property_value1167.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value1168,
                                _,
                            ) => source_ml_source1_data_property_value1168.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value1169,
                                _,
                            ) => source_ml_source1_data_property_value1169.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value1170,
                                _,
                            ) => source_ml_source1_data_property_value1170.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value1171,
                            ) => source_ml_source1_data_property_value1171.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value1172,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value1172.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value1173,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value1173.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value1174,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value1174.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value1175,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value1175.clone(),
                        };
                        {
                            let source_ml_source1_array_read_index1190 = self::find_java_script_array_property(match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value1176, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1176.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value1177, _, _, _) => {
                                    source_ml_source1_data_property_value1177.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value1178) => {
                                    source_ml_source1_data_property_value1178.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value1179, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1179.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value1180, _) => {
                                    source_ml_source1_data_property_value1180.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value1181, _) => {
                                    source_ml_source1_data_property_value1181.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value1182, _) => {
                                    source_ml_source1_data_property_value1182.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value1183, _) => {
                                    source_ml_source1_data_property_value1183.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value1184) => {
                                    source_ml_source1_data_property_value1184.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value1185, _, _) => {
                                    source_ml_source1_data_property_value1185.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value1186, _, _) => {
                                    source_ml_source1_data_property_value1186.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value1187, _, _) => {
                                    source_ml_source1_data_property_value1187.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value1188, _, _) => {
                                    source_ml_source1_data_property_value1188.clone()
                                }
                            }, self::read_java_script_array_length_key());
                            {
                                let ml_reference_array133 = source_ml_source1_array_read_value1189;
                                let ml_reference_position133 =
                                    source_ml_source1_array_read_index1190;
                                self::ml_array::optional_at(
                                    &*ml_reference_array133.borrow(),
                                    self::ml_array::number_index(ml_reference_position133),
                                )
                            }
                        }
                    };
                    if (index >= length)
                        && (match (length_property.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } || !(({
                            (match length_property.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    _,
                                    source_ml_source1_data_property_value1191,
                                ) => source_ml_source1_data_property_value1191.clone(),
                            })
                            .borrow()
                            .clone()
                        })
                        .10))
                    {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            self::ml_string::Text::from_units(vec![]),
                            self::ml_string::Text::from_units(vec![]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    } else {
                        {
                            let plan = self::plan_java_script_array_ordinary_definition(
                                object.clone(),
                                key.clone(),
                                proposed.clone(),
                            );
                            if (!(match plan.clone() {
                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                                    _,
                                    source_ml_source1_data_property_value1192,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value1192,
                            }) || (index < length))
                                || (index < 0f64)
                            {
                                plan.clone()
                            } else {
                                {
                                    let properties =
                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                            self::JavaScriptOrdinaryProperty,
                                        >::new(
                                        )));
                                    {
                                        let source_ml_temporary_values29 = match match plan.clone() {
                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value1193, _, _, _, _) => {
                                                let source_ml_source1_data_property_value1193 = *source_ml_source1_data_property_value1193;
                                                source_ml_source1_data_property_value1193.clone()
                                            }
                                        } {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value1194, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value1194.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value1195, _, _, _) => {
                                                source_ml_source1_data_property_value1195.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value1196) => {
                                                source_ml_source1_data_property_value1196.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value1197, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value1197.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value1198, _) => {
                                                source_ml_source1_data_property_value1198.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value1199, _) => {
                                                source_ml_source1_data_property_value1199.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value1200, _) => {
                                                source_ml_source1_data_property_value1200.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value1201, _) => {
                                                source_ml_source1_data_property_value1201.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value1202) => {
                                                source_ml_source1_data_property_value1202.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value1203, _, _) => {
                                                source_ml_source1_data_property_value1203.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value1204, _, _) => {
                                                source_ml_source1_data_property_value1204.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value1205, _, _) => {
                                                source_ml_source1_data_property_value1205.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value1206, _, _) => {
                                                source_ml_source1_data_property_value1206.clone()
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index29 =
                                                self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source1_s1207 = self::source_ml_temporary_plan_java_script_array_property_definition_loop30(index, properties.clone(), source_ml_temporary_values29, source_ml_temporary_index29);
                                                match source_ml_source1_s1207 {
                                                    self::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Result::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Done => {
                                                        {
                                                            let source_ml_source1_nominal_field1321 = match plan.clone() {
                                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value1322, _, _, _, _) => {
                                                                    let source_ml_source1_data_property_value1322 = *source_ml_source1_data_property_value1322;
                                                                    source_ml_source1_data_property_value1322.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field1323 = match plan.clone() {
                                                                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, source_ml_source1_data_property_value1324, _, _, _) => {
                                                                        source_ml_source1_data_property_value1324
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_field1325 = match plan.clone() {
                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, source_ml_source1_data_property_value1326, _, _) => {
                                                                            source_ml_source1_data_property_value1326.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_nominal_field1327 = match plan.clone() {
                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, source_ml_source1_data_property_value1328, _) => {
                                                                                source_ml_source1_data_property_value1328.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_nominal_field1329 = match plan.clone() {
                                                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value1330) => {
                                                                                    source_ml_source1_data_property_value1330.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_override1331 = match match plan.clone() {
                                                                                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value1208, _, _, _, _) => {
                                                                                        let source_ml_source1_data_property_value1208 = *source_ml_source1_data_property_value1208;
                                                                                        source_ml_source1_data_property_value1208.clone()
                                                                                    }
                                                                                } {
                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field1209, source_ml_source1_nominal_field1210, source_ml_source1_nominal_field1211, source_ml_source1_nominal_field1212, _, source_ml_source1_nominal_field1214, source_ml_source1_nominal_field1215, source_ml_source1_nominal_field1216, source_ml_source1_nominal_field1217, source_ml_source1_nominal_field1218) => {
                                                                                        let source_ml_source1_nominal_field1210 = *source_ml_source1_nominal_field1210;
                                                                                        let source_ml_source1_nominal_field1217 = *source_ml_source1_nominal_field1217;
                                                                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field1209.clone(), Box::new(source_ml_source1_nominal_field1210.clone()), source_ml_source1_nominal_field1211, source_ml_source1_nominal_field1212, properties.clone(), source_ml_source1_nominal_field1214.clone(), source_ml_source1_nominal_field1215, source_ml_source1_nominal_field1216.clone(), Box::new(source_ml_source1_nominal_field1217.clone()), source_ml_source1_nominal_field1218.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field1220, source_ml_source1_nominal_field1221, source_ml_source1_nominal_field1222, source_ml_source1_nominal_field1223, _, source_ml_source1_nominal_field1225, source_ml_source1_nominal_field1226, source_ml_source1_nominal_field1227) => {
                                                                                        let source_ml_source1_nominal_field1221 = *source_ml_source1_nominal_field1221;
                                                                                        let source_ml_source1_nominal_field1225 = *source_ml_source1_nominal_field1225;
                                                                                        let source_ml_source1_nominal_field1226 = *source_ml_source1_nominal_field1226;
                                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field1220.clone(), Box::new(source_ml_source1_nominal_field1221.clone()), source_ml_source1_nominal_field1222, source_ml_source1_nominal_field1223, properties.clone(), Box::new(source_ml_source1_nominal_field1225.clone()), Box::new(source_ml_source1_nominal_field1226.clone()), source_ml_source1_nominal_field1227.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field1229, source_ml_source1_nominal_field1230, source_ml_source1_nominal_field1231, source_ml_source1_nominal_field1232, _) => {
                                                                                        let source_ml_source1_nominal_field1230 = *source_ml_source1_nominal_field1230;
                                                                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field1229.clone(), Box::new(source_ml_source1_nominal_field1230.clone()), source_ml_source1_nominal_field1231, source_ml_source1_nominal_field1232, properties.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field1235, source_ml_source1_nominal_field1236, source_ml_source1_nominal_field1237, source_ml_source1_nominal_field1238, _, source_ml_source1_nominal_field1240, source_ml_source1_nominal_field1241, source_ml_source1_nominal_field1242, source_ml_source1_nominal_field1243, source_ml_source1_nominal_field1244, source_ml_source1_nominal_field1245) => {
                                                                                        let source_ml_source1_nominal_field1236 = *source_ml_source1_nominal_field1236;
                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field1235.clone(), Box::new(source_ml_source1_nominal_field1236.clone()), source_ml_source1_nominal_field1237, source_ml_source1_nominal_field1238, properties.clone(), source_ml_source1_nominal_field1240.clone(), source_ml_source1_nominal_field1241, source_ml_source1_nominal_field1242.clone(), source_ml_source1_nominal_field1243.clone(), source_ml_source1_nominal_field1244.clone(), source_ml_source1_nominal_field1245.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field1247, source_ml_source1_nominal_field1248, source_ml_source1_nominal_field1249, source_ml_source1_nominal_field1250, _, source_ml_source1_nominal_field1252) => {
                                                                                        let source_ml_source1_nominal_field1248 = *source_ml_source1_nominal_field1248;
                                                                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field1247.clone(), Box::new(source_ml_source1_nominal_field1248.clone()), source_ml_source1_nominal_field1249, source_ml_source1_nominal_field1250, properties.clone(), source_ml_source1_nominal_field1252.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field1254, source_ml_source1_nominal_field1255, source_ml_source1_nominal_field1256, source_ml_source1_nominal_field1257, _, source_ml_source1_nominal_field1259) => {
                                                                                        let source_ml_source1_nominal_field1255 = *source_ml_source1_nominal_field1255;
                                                                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field1254.clone(), Box::new(source_ml_source1_nominal_field1255.clone()), source_ml_source1_nominal_field1256, source_ml_source1_nominal_field1257, properties.clone(), source_ml_source1_nominal_field1259.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field1261, source_ml_source1_nominal_field1262, source_ml_source1_nominal_field1263, source_ml_source1_nominal_field1264, _, source_ml_source1_nominal_field1266) => {
                                                                                        let source_ml_source1_nominal_field1262 = *source_ml_source1_nominal_field1262;
                                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field1261.clone(), Box::new(source_ml_source1_nominal_field1262.clone()), source_ml_source1_nominal_field1263, source_ml_source1_nominal_field1264, properties.clone(), source_ml_source1_nominal_field1266.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field1268, source_ml_source1_nominal_field1269, source_ml_source1_nominal_field1270, source_ml_source1_nominal_field1271, _, source_ml_source1_nominal_field1273) => {
                                                                                        let source_ml_source1_nominal_field1269 = *source_ml_source1_nominal_field1269;
                                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field1268.clone(), Box::new(source_ml_source1_nominal_field1269.clone()), source_ml_source1_nominal_field1270, source_ml_source1_nominal_field1271, properties.clone(), source_ml_source1_nominal_field1273.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field1275, source_ml_source1_nominal_field1276, source_ml_source1_nominal_field1277, source_ml_source1_nominal_field1278, source_ml_source1_nominal_field1279, source_ml_source1_nominal_field1280, source_ml_source1_nominal_field1281, source_ml_source1_nominal_field1282, _) => {
                                                                                        let source_ml_source1_nominal_field1276 = *source_ml_source1_nominal_field1276;
                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field1275.clone(), Box::new(source_ml_source1_nominal_field1276.clone()), source_ml_source1_nominal_field1277, source_ml_source1_nominal_field1278.clone(), source_ml_source1_nominal_field1279.clone(), source_ml_source1_nominal_field1280.clone(), source_ml_source1_nominal_field1281.clone(), source_ml_source1_nominal_field1282, properties.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field1285, source_ml_source1_nominal_field1286, source_ml_source1_nominal_field1287, source_ml_source1_nominal_field1288, _, source_ml_source1_nominal_field1290, source_ml_source1_nominal_field1291) => {
                                                                                        let source_ml_source1_nominal_field1286 = *source_ml_source1_nominal_field1286;
                                                                                        let source_ml_source1_nominal_field1290 = *source_ml_source1_nominal_field1290;
                                                                                        let source_ml_source1_nominal_field1291 = *source_ml_source1_nominal_field1291;
                                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field1285.clone(), Box::new(source_ml_source1_nominal_field1286.clone()), source_ml_source1_nominal_field1287, source_ml_source1_nominal_field1288, properties.clone(), Box::new(source_ml_source1_nominal_field1290.clone()), Box::new(source_ml_source1_nominal_field1291.clone()))
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field1293, source_ml_source1_nominal_field1294, source_ml_source1_nominal_field1295, source_ml_source1_nominal_field1296, _, source_ml_source1_nominal_field1298, source_ml_source1_nominal_field1299) => {
                                                                                        let source_ml_source1_nominal_field1294 = *source_ml_source1_nominal_field1294;
                                                                                        let source_ml_source1_nominal_field1298 = *source_ml_source1_nominal_field1298;
                                                                                        let source_ml_source1_nominal_field1299 = *source_ml_source1_nominal_field1299;
                                                                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field1293.clone(), Box::new(source_ml_source1_nominal_field1294.clone()), source_ml_source1_nominal_field1295, source_ml_source1_nominal_field1296, properties.clone(), Box::new(source_ml_source1_nominal_field1298.clone()), Box::new(source_ml_source1_nominal_field1299.clone()))
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field1301, source_ml_source1_nominal_field1302, source_ml_source1_nominal_field1303, source_ml_source1_nominal_field1304, _, source_ml_source1_nominal_field1306, source_ml_source1_nominal_field1307) => {
                                                                                        let source_ml_source1_nominal_field1302 = *source_ml_source1_nominal_field1302;
                                                                                        let source_ml_source1_nominal_field1306 = *source_ml_source1_nominal_field1306;
                                                                                        let source_ml_source1_nominal_field1307 = *source_ml_source1_nominal_field1307;
                                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field1301.clone(), Box::new(source_ml_source1_nominal_field1302.clone()), source_ml_source1_nominal_field1303, source_ml_source1_nominal_field1304, properties.clone(), Box::new(source_ml_source1_nominal_field1306.clone()), Box::new(source_ml_source1_nominal_field1307.clone()))
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field1309, source_ml_source1_nominal_field1310, source_ml_source1_nominal_field1311, source_ml_source1_nominal_field1312, _, source_ml_source1_nominal_field1314, source_ml_source1_nominal_field1315) => {
                                                                                        let source_ml_source1_nominal_field1310 = *source_ml_source1_nominal_field1310;
                                                                                        let source_ml_source1_nominal_field1314 = *source_ml_source1_nominal_field1314;
                                                                                        let source_ml_source1_nominal_field1315 = *source_ml_source1_nominal_field1315;
                                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field1309.clone(), Box::new(source_ml_source1_nominal_field1310.clone()), source_ml_source1_nominal_field1311, source_ml_source1_nominal_field1312, properties.clone(), Box::new(source_ml_source1_nominal_field1314.clone()), Box::new(source_ml_source1_nominal_field1315.clone()))
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_override1332 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                        let source_ml_source1_concat_value1318 = match plan.clone() {
                                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value1317) => {
                                                                                                source_ml_source1_data_property_value1317.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_concat_value1319 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])]));
                                                                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { source_ml_source1_concat_value1318.borrow().clone() }), { source_ml_source1_concat_value1319.borrow().clone() })
                                                                                        }
                                                                                    }));
                                                                                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(source_ml_source1_nominal_override1331), source_ml_source1_nominal_field1323, source_ml_source1_nominal_field1325, source_ml_source1_nominal_field1327, source_ml_source1_nominal_override1332)
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_array_unsigned_length(value: f64) -> f64 {
    if !(value.is_finite()) || (value == 0f64) {
        0f64
    } else {
        {
            let integer = value.trunc();
            {
                let remainder = integer % 4_294_967_296f64;
                if remainder < 0f64 {
                    remainder + 4_294_967_296f64
                } else if remainder == 0f64 {
                    0f64
                } else {
                    remainder
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_collection_key(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> bool {
    {
        let source_ml_source1_s1349 = match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Next
            }
            self::JavaScriptSemanticValue::Null => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Next
            }
            self::JavaScriptSemanticValue::Boolean(_) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Next
            }
            self::JavaScriptSemanticValue::Number(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Next
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Next
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Next
                    }
                    self::JavaScriptSemanticValue::Number(value) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Return((left_value == value) || (left_value.is_nan() && value.is_nan()))
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Next
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Next
                    }
                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Next
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Next
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Next
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(_) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Next
            }
            self::JavaScriptSemanticValue::Text(_) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Next
            }
            self::JavaScriptSemanticValue::SourceText(_, _) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Next
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Next
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Next
            }
        };
        match source_ml_source1_s1349 {
            self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Next => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::MlTemporaryTestJavaScriptCollectionKeyJoin32::MlTemporaryTestJavaScriptCollectionKeyJoin32Return(source_ml_temporary_result32) => {
                source_ml_temporary_result32
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_key_rank(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => 0f64,
        self::JavaScriptSemanticValue::Null => 1f64,
        self::JavaScriptSemanticValue::Boolean(_) => 2f64,
        self::JavaScriptSemanticValue::Number(_) => 3f64,
        self::JavaScriptSemanticValue::Integer(_) => 4f64,
        self::JavaScriptSemanticValue::Symbol(_) => 6f64,
        self::JavaScriptSemanticValue::Object(_, _) => 7f64,
        _ => 5f64,
    }
}
