// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayPropertyPlan;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34;
use super::MlSource1TemporaryCompareJavaScriptCollectionKeysLoop35Result;
use super::MlSource1TemporaryNormalizeJavaScriptCollectionKeyJoin33;
use super::MlSource1TemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Result;
use super::MlSource1TemporaryTestJavaScriptCollectionKeyJoin32;
use super::find_java_script_array_property;
use super::find_java_script_heap_directory_object;
use super::find_java_script_heap_directory_property;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::plan_java_script_array_length_definition;
use super::plan_java_script_array_ordinary_definition;
use super::read_java_script_array_index;
use super::read_java_script_array_length;
use super::read_java_script_array_length_key;
use super::read_java_script_heap_directory_key_error;
use super::read_java_script_heap_directory_value_error;
use super::read_java_script_semantic_text_length;
use super::read_java_script_weak_collection_method_owner;
use super::source_ml_source1_temporary_compare_java_script_collection_keys_loop35;
use super::source_ml_source1_temporary_order_java_script_collection_keys_loop36;
use super::source_ml_source1_temporary_plan_java_script_array_property_definition_loop30;
use super::test_java_script_array_length_key;
use super::test_java_script_heap_directory_symbol;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_array_property_definition(
    object: self::JavaScriptOrdinaryObject,
    key: self::JavaScriptSemanticValue,
    proposed: std::rc::Rc<
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
) -> self::JavaScriptArrayPropertyPlan {
    if self::test_java_script_array_length_key(key.clone()) {
        if !((({
            let ml_reference_value163 = proposed.borrow().clone();
            ml_reference_value163
        })
        .0)
            .9)
        {
            self::plan_java_script_array_ordinary_definition(
                object.clone(),
                key.clone(),
                proposed.clone(),
            )
        } else {
            {
                let value = (({
                    let ml_reference_value164 = proposed.borrow().clone();
                    ml_reference_value164
                })
                .0)
                    .8;
                match value.clone() {
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        self::plan_java_script_array_length_definition(
                            object.clone(),
                            proposed.clone(),
                            self::convert_java_script_array_unsigned_length(value_value),
                            value_value,
                        )
                    }
                    _ => self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                        Box::new(object.clone()),
                        false,
                        self::ml_string::Text::from_units(vec![
                            97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101,
                            113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                            97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32,
                            99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116,
                            105, 110, 117, 97, 116, 105, 111, 110, 115,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                        ]),
                        std::rc::Rc::new(std::cell::RefCell::new(
                            Vec::<self::ml_string::Text>::new(),
                        )),
                    ),
                }
            }
        }
    } else {
        {
            let index = self::read_java_script_array_index(key.clone());
            {
                let length = self::read_java_script_array_length(object.clone());
                {
                    let length_property = {
                        let source_ml_source2_array_read_value644 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value618,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value618.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value619,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value619.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value620,
                            ) => source_ml_source2_data_property_value620.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value621,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value621.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value622,
                                _,
                            ) => source_ml_source2_data_property_value622.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value623,
                                _,
                            ) => source_ml_source2_data_property_value623.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value624,
                                _,
                            ) => source_ml_source2_data_property_value624.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value625,
                                _,
                            ) => source_ml_source2_data_property_value625.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value626,
                            ) => source_ml_source2_data_property_value626.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value627,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value627.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value628,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value628.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value629,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value629.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value630,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value630.clone(),
                        };
                        {
                            let source_ml_source2_array_read_index645 = self::find_java_script_array_property(match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value631, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value631.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value632, _, _, _) => {
                                    source_ml_source2_data_property_value632.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value633) => {
                                    source_ml_source2_data_property_value633.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value634, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value634.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value635, _) => {
                                    source_ml_source2_data_property_value635.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value636, _) => {
                                    source_ml_source2_data_property_value636.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value637, _) => {
                                    source_ml_source2_data_property_value637.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value638, _) => {
                                    source_ml_source2_data_property_value638.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value639) => {
                                    source_ml_source2_data_property_value639.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value640, _, _) => {
                                    source_ml_source2_data_property_value640.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value641, _, _) => {
                                    source_ml_source2_data_property_value641.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value642, _, _) => {
                                    source_ml_source2_data_property_value642.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value643, _, _) => {
                                    source_ml_source2_data_property_value643.clone()
                                }
                            }, self::read_java_script_array_length_key());
                            {
                                let ml_reference_array165 =
                                    source_ml_source2_array_read_value644.clone();
                                let ml_reference_position165 =
                                    source_ml_source2_array_read_index645;
                                let ml_reference_projection165 = self::ml_array::optional_at(
                                    &*ml_reference_array165.borrow(),
                                    self::ml_array::number_index(ml_reference_position165),
                                );
                                ml_reference_projection165
                            }
                        }
                    };
                    if ((index >= length)
                        && (match (length_property.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } || !((({
                            let ml_reference_value166 =
                                (match length_property.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        _,
                                        source_ml_source2_data_property_value646,
                                    ) => source_ml_source2_data_property_value646.clone(),
                                })
                                .borrow()
                                .clone();
                            ml_reference_value166
                        })
                        .0)
                            .10)))
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
                            match plan.clone() {
                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                                    shared_constructor_field1,
                                    shared_constructor_field2,
                                    shared_constructor_field3,
                                    shared_constructor_field4,
                                    shared_constructor_field5,
                                ) => {
                                    let shared_constructor_field1 = *shared_constructor_field1;
                                    if ((!shared_constructor_field2 || (index < length))
                                        || (index < (0f64)))
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
                                                let source_ml_source1_temporary_values29 = match shared_constructor_field1.clone() {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value649, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value649.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value650, _, _, _) => {
                                                        source_ml_source2_data_property_value650.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value651) => {
                                                        source_ml_source2_data_property_value651.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value652, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value652.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value653, _) => {
                                                        source_ml_source2_data_property_value653.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value654, _) => {
                                                        source_ml_source2_data_property_value654.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value655, _) => {
                                                        source_ml_source2_data_property_value655.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value656, _) => {
                                                        source_ml_source2_data_property_value656.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value657) => {
                                                        source_ml_source2_data_property_value657.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value658, _, _) => {
                                                        source_ml_source2_data_property_value658.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value659, _, _) => {
                                                        source_ml_source2_data_property_value659.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value660, _, _) => {
                                                        source_ml_source2_data_property_value660.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value661, _, _) => {
                                                        source_ml_source2_data_property_value661.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_temporary_index29 =
                                                        self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s662 = self::source_ml_source1_temporary_plan_java_script_array_property_definition_loop30(index, properties.clone(), source_ml_source1_temporary_values29.clone(), source_ml_source1_temporary_index29.clone());
                                                        match source_ml_source2_s662.clone() {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Result::MlSource1TemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Done => {
                                                                {
                                                                    let source_ml_source2_nominal_field776 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field778 = shared_constructor_field2;
                                                                        {
                                                                            let source_ml_source2_nominal_field780 = shared_constructor_field3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field782 = shared_constructor_field4.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field784 = shared_constructor_field5.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_override786 = match shared_constructor_field1.clone() {
                                                                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field664, source_ml_source2_nominal_field665, source_ml_source2_nominal_field666, source_ml_source2_nominal_field667, _, source_ml_source2_nominal_field669, source_ml_source2_nominal_field670, source_ml_source2_nominal_field671, source_ml_source2_nominal_field672, source_ml_source2_nominal_field673) => {
                                                                                                let source_ml_source2_nominal_field665 = *source_ml_source2_nominal_field665;
                                                                                                let source_ml_source2_nominal_field672 = *source_ml_source2_nominal_field672;
                                                                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field664.clone(), Box::new(source_ml_source2_nominal_field665.clone()), source_ml_source2_nominal_field666, source_ml_source2_nominal_field667, properties.clone(), source_ml_source2_nominal_field669.clone(), source_ml_source2_nominal_field670, source_ml_source2_nominal_field671.clone(), Box::new(source_ml_source2_nominal_field672.clone()), source_ml_source2_nominal_field673.clone())
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field675, source_ml_source2_nominal_field676, source_ml_source2_nominal_field677, source_ml_source2_nominal_field678, _, source_ml_source2_nominal_field680, source_ml_source2_nominal_field681, source_ml_source2_nominal_field682) => {
                                                                                                let source_ml_source2_nominal_field676 = *source_ml_source2_nominal_field676;
                                                                                                let source_ml_source2_nominal_field680 = *source_ml_source2_nominal_field680;
                                                                                                let source_ml_source2_nominal_field681 = *source_ml_source2_nominal_field681;
                                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field675.clone(), Box::new(source_ml_source2_nominal_field676.clone()), source_ml_source2_nominal_field677, source_ml_source2_nominal_field678, properties.clone(), Box::new(source_ml_source2_nominal_field680.clone()), Box::new(source_ml_source2_nominal_field681.clone()), source_ml_source2_nominal_field682.clone())
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field684, source_ml_source2_nominal_field685, source_ml_source2_nominal_field686, source_ml_source2_nominal_field687, _) => {
                                                                                                let source_ml_source2_nominal_field685 = *source_ml_source2_nominal_field685;
                                                                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field684.clone(), Box::new(source_ml_source2_nominal_field685.clone()), source_ml_source2_nominal_field686, source_ml_source2_nominal_field687, properties.clone())
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field690, source_ml_source2_nominal_field691, source_ml_source2_nominal_field692, source_ml_source2_nominal_field693, _, source_ml_source2_nominal_field695, source_ml_source2_nominal_field696, source_ml_source2_nominal_field697, source_ml_source2_nominal_field698, source_ml_source2_nominal_field699, source_ml_source2_nominal_field700) => {
                                                                                                let source_ml_source2_nominal_field691 = *source_ml_source2_nominal_field691;
                                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field690.clone(), Box::new(source_ml_source2_nominal_field691.clone()), source_ml_source2_nominal_field692, source_ml_source2_nominal_field693, properties.clone(), source_ml_source2_nominal_field695.clone(), source_ml_source2_nominal_field696, source_ml_source2_nominal_field697.clone(), source_ml_source2_nominal_field698.clone(), source_ml_source2_nominal_field699.clone(), source_ml_source2_nominal_field700.clone())
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field702, source_ml_source2_nominal_field703, source_ml_source2_nominal_field704, source_ml_source2_nominal_field705, _, source_ml_source2_nominal_field707) => {
                                                                                                let source_ml_source2_nominal_field703 = *source_ml_source2_nominal_field703;
                                                                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field702.clone(), Box::new(source_ml_source2_nominal_field703.clone()), source_ml_source2_nominal_field704, source_ml_source2_nominal_field705, properties.clone(), source_ml_source2_nominal_field707.clone())
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field709, source_ml_source2_nominal_field710, source_ml_source2_nominal_field711, source_ml_source2_nominal_field712, _, source_ml_source2_nominal_field714) => {
                                                                                                let source_ml_source2_nominal_field710 = *source_ml_source2_nominal_field710;
                                                                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field709.clone(), Box::new(source_ml_source2_nominal_field710.clone()), source_ml_source2_nominal_field711, source_ml_source2_nominal_field712, properties.clone(), source_ml_source2_nominal_field714.clone())
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field716, source_ml_source2_nominal_field717, source_ml_source2_nominal_field718, source_ml_source2_nominal_field719, _, source_ml_source2_nominal_field721) => {
                                                                                                let source_ml_source2_nominal_field717 = *source_ml_source2_nominal_field717;
                                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field716.clone(), Box::new(source_ml_source2_nominal_field717.clone()), source_ml_source2_nominal_field718, source_ml_source2_nominal_field719, properties.clone(), source_ml_source2_nominal_field721.clone())
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field723, source_ml_source2_nominal_field724, source_ml_source2_nominal_field725, source_ml_source2_nominal_field726, _, source_ml_source2_nominal_field728) => {
                                                                                                let source_ml_source2_nominal_field724 = *source_ml_source2_nominal_field724;
                                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field723.clone(), Box::new(source_ml_source2_nominal_field724.clone()), source_ml_source2_nominal_field725, source_ml_source2_nominal_field726, properties.clone(), source_ml_source2_nominal_field728.clone())
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field730, source_ml_source2_nominal_field731, source_ml_source2_nominal_field732, source_ml_source2_nominal_field733, source_ml_source2_nominal_field734, source_ml_source2_nominal_field735, source_ml_source2_nominal_field736, source_ml_source2_nominal_field737, _) => {
                                                                                                let source_ml_source2_nominal_field731 = *source_ml_source2_nominal_field731;
                                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field730.clone(), Box::new(source_ml_source2_nominal_field731.clone()), source_ml_source2_nominal_field732, source_ml_source2_nominal_field733.clone(), source_ml_source2_nominal_field734.clone(), source_ml_source2_nominal_field735.clone(), source_ml_source2_nominal_field736.clone(), source_ml_source2_nominal_field737, properties.clone())
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field740, source_ml_source2_nominal_field741, source_ml_source2_nominal_field742, source_ml_source2_nominal_field743, _, source_ml_source2_nominal_field745, source_ml_source2_nominal_field746) => {
                                                                                                let source_ml_source2_nominal_field741 = *source_ml_source2_nominal_field741;
                                                                                                let source_ml_source2_nominal_field745 = *source_ml_source2_nominal_field745;
                                                                                                let source_ml_source2_nominal_field746 = *source_ml_source2_nominal_field746;
                                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field740.clone(), Box::new(source_ml_source2_nominal_field741.clone()), source_ml_source2_nominal_field742, source_ml_source2_nominal_field743, properties.clone(), Box::new(source_ml_source2_nominal_field745.clone()), Box::new(source_ml_source2_nominal_field746.clone()))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field748, source_ml_source2_nominal_field749, source_ml_source2_nominal_field750, source_ml_source2_nominal_field751, _, source_ml_source2_nominal_field753, source_ml_source2_nominal_field754) => {
                                                                                                let source_ml_source2_nominal_field749 = *source_ml_source2_nominal_field749;
                                                                                                let source_ml_source2_nominal_field753 = *source_ml_source2_nominal_field753;
                                                                                                let source_ml_source2_nominal_field754 = *source_ml_source2_nominal_field754;
                                                                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field748.clone(), Box::new(source_ml_source2_nominal_field749.clone()), source_ml_source2_nominal_field750, source_ml_source2_nominal_field751, properties.clone(), Box::new(source_ml_source2_nominal_field753.clone()), Box::new(source_ml_source2_nominal_field754.clone()))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field756, source_ml_source2_nominal_field757, source_ml_source2_nominal_field758, source_ml_source2_nominal_field759, _, source_ml_source2_nominal_field761, source_ml_source2_nominal_field762) => {
                                                                                                let source_ml_source2_nominal_field757 = *source_ml_source2_nominal_field757;
                                                                                                let source_ml_source2_nominal_field761 = *source_ml_source2_nominal_field761;
                                                                                                let source_ml_source2_nominal_field762 = *source_ml_source2_nominal_field762;
                                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field756.clone(), Box::new(source_ml_source2_nominal_field757.clone()), source_ml_source2_nominal_field758, source_ml_source2_nominal_field759, properties.clone(), Box::new(source_ml_source2_nominal_field761.clone()), Box::new(source_ml_source2_nominal_field762.clone()))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field764, source_ml_source2_nominal_field765, source_ml_source2_nominal_field766, source_ml_source2_nominal_field767, _, source_ml_source2_nominal_field769, source_ml_source2_nominal_field770) => {
                                                                                                let source_ml_source2_nominal_field765 = *source_ml_source2_nominal_field765;
                                                                                                let source_ml_source2_nominal_field769 = *source_ml_source2_nominal_field769;
                                                                                                let source_ml_source2_nominal_field770 = *source_ml_source2_nominal_field770;
                                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field764.clone(), Box::new(source_ml_source2_nominal_field765.clone()), source_ml_source2_nominal_field766, source_ml_source2_nominal_field767, properties.clone(), Box::new(source_ml_source2_nominal_field769.clone()), Box::new(source_ml_source2_nominal_field770.clone()))
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override787 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                let source_ml_source2_concat_value773 = shared_constructor_field5.clone();
                                                                                                {
                                                                                                    let source_ml_source2_concat_value774 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])]));
                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value167 = source_ml_source2_concat_value773.borrow().clone(); ml_reference_value167 }), { let ml_reference_value168 = source_ml_source2_concat_value774.borrow().clone(); ml_reference_value168 })
                                                                                                }
                                                                                            }));
                                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(source_ml_source2_nominal_override786.clone()), source_ml_source2_nominal_field778, source_ml_source2_nominal_field780.clone(), source_ml_source2_nominal_field782.clone(), source_ml_source2_nominal_override787.clone())
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_array_unsigned_length(value: f64) -> f64 {
    if (!(value.is_finite()) || (value == (0f64))) {
        0f64
    } else {
        {
            let integer = value.trunc();
            {
                let remainder = (integer % 4294967296f64);
                if (remainder < (0f64)) {
                    (remainder + 4294967296f64)
                } else {
                    if (remainder == (0f64)) {
                        0f64
                    } else {
                        remainder
                    }
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
        let source_ml_source2_s788 = match left.clone() {
            self::JavaScriptSemanticValue::Number(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Number(value) => {
                        self::MlSource1TemporaryTestJavaScriptCollectionKeyJoin32::MlSource1TemporaryTestJavaScriptCollectionKeyJoin32Return(((left_value == value) || (left_value.is_nan() && value.is_nan())))
                    }
                    _ => {
                        self::MlSource1TemporaryTestJavaScriptCollectionKeyJoin32::MlSource1TemporaryTestJavaScriptCollectionKeyJoin32Next
                    }
                }
            }
            _ => {
                self::MlSource1TemporaryTestJavaScriptCollectionKeyJoin32::MlSource1TemporaryTestJavaScriptCollectionKeyJoin32Next
            }
        };
        match source_ml_source2_s788.clone() {
            self::MlSource1TemporaryTestJavaScriptCollectionKeyJoin32::MlSource1TemporaryTestJavaScriptCollectionKeyJoin32Next => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::MlSource1TemporaryTestJavaScriptCollectionKeyJoin32::MlSource1TemporaryTestJavaScriptCollectionKeyJoin32Return(source_ml_source1_temporary_result32) => {
                source_ml_source1_temporary_result32
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn normalize_java_script_collection_key(
    key: self::JavaScriptSemanticValue,
) -> self::JavaScriptSemanticValue {
    {
        let source_ml_source2_s789 = match key.clone() {
            self::JavaScriptSemanticValue::Number(value) => {
                if (value == (0f64)) {
                    self::MlSource1TemporaryNormalizeJavaScriptCollectionKeyJoin33::MlSource1TemporaryNormalizeJavaScriptCollectionKeyJoin33Return(Box::new(self::JavaScriptSemanticValue::Number(0f64)))
                } else {
                    self::MlSource1TemporaryNormalizeJavaScriptCollectionKeyJoin33::MlSource1TemporaryNormalizeJavaScriptCollectionKeyJoin33Next
                }
            }
            _ => {
                self::MlSource1TemporaryNormalizeJavaScriptCollectionKeyJoin33::MlSource1TemporaryNormalizeJavaScriptCollectionKeyJoin33Next
            }
        };
        match source_ml_source2_s789.clone() {
            self::MlSource1TemporaryNormalizeJavaScriptCollectionKeyJoin33::MlSource1TemporaryNormalizeJavaScriptCollectionKeyJoin33Next => {
                key.clone()
            }
            self::MlSource1TemporaryNormalizeJavaScriptCollectionKeyJoin33::MlSource1TemporaryNormalizeJavaScriptCollectionKeyJoin33Return(source_ml_source1_temporary_result33) => {
                let source_ml_source1_temporary_result33 = *source_ml_source1_temporary_result33;
                source_ml_source1_temporary_result33.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_operation_kind(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if self::ml_array::includes(
        &{
            let ml_reference_value169 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                self::ml_string::Text::from_units(vec![77, 97, 112, 46, 103, 101, 116]),
                self::ml_string::Text::from_units(vec![77, 97, 112, 46, 115, 101, 116]),
                self::ml_string::Text::from_units(vec![77, 97, 112, 46, 104, 97, 115]),
                self::ml_string::Text::from_units(vec![
                    77, 97, 112, 46, 100, 101, 108, 101, 116, 101,
                ]),
                self::ml_string::Text::from_units(vec![77, 97, 112, 46, 99, 108, 101, 97, 114]),
                self::ml_string::Text::from_units(vec![77, 97, 112, 46, 115, 105, 122, 101]),
            ])))
            .borrow()
            .clone();
            ml_reference_value169
        },
        operation,
        0f64,
        false,
        |a, b| a == b,
    ) {
        self::ml_string::Text::from_units(vec![77, 97, 112])
    } else {
        if self::ml_array::includes(
            &{
                let ml_reference_value170 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                    self::ml_string::Text::from_units(vec![83, 101, 116, 46, 97, 100, 100]),
                    self::ml_string::Text::from_units(vec![83, 101, 116, 46, 104, 97, 115]),
                    self::ml_string::Text::from_units(vec![
                        83, 101, 116, 46, 100, 101, 108, 101, 116, 101,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        83, 101, 116, 46, 99, 108, 101, 97, 114,
                    ]),
                    self::ml_string::Text::from_units(vec![83, 101, 116, 46, 115, 105, 122, 101]),
                ])))
                .borrow()
                .clone();
                ml_reference_value170
            },
            operation,
            0f64,
            false,
            |a, b| a == b,
        ) {
            self::ml_string::Text::from_units(vec![83, 101, 116])
        } else {
            self::read_java_script_weak_collection_method_owner(operation)
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_java_script_collection_keys(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> f64 {
    {
        let a = self::read_java_script_collection_key_rank(left.clone());
        {
            let b = self::read_java_script_collection_key_rank(right.clone());
            if (a != b) {
                if (a < b) { (-1f64) } else { 1f64 }
            } else {
                {
                    let source_ml_source2_s803 = match left.clone() {
                        self::JavaScriptSemanticValue::Boolean(left_value) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Boolean(value) => {
                                    self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Return(if (left_value == value) {
                                        0f64
                                    } else {
                                        if left_value {
                                            1f64
                                        } else {
                                            (-1f64)
                                        }
                                    })
                                }
                                _ => {
                                    self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Number(left_value_2) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Number(value_2) => {
                                    if self::test_java_script_collection_key(left.clone(), right.clone()) {
                                        self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                    } else {
                                        if left_value_2.is_nan() {
                                            self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Return(1f64)
                                        } else {
                                            if value_2.is_nan() {
                                                self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Return((-1f64))
                                            } else {
                                                self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Return(if (left_value_2 < value_2) {
                                                    (-1f64)
                                                } else {
                                                    1f64
                                                })
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(left_value_3) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Integer(value_3) => {
                                    self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Return(if (left_value_3 == value_3) {
                                        0f64
                                    } else {
                                        if (left_value_3 < value_3) {
                                            (-1f64)
                                        } else {
                                            1f64
                                        }
                                    })
                                }
                                _ => {
                                    self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::SourceText(_, left_text) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::SourceText(_, text) => {
                                    self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Return(if (left_text == text) {
                                        0f64
                                    } else {
                                        if <self::ml_string::Text as std::cmp::PartialOrd>::lt(&left_text, &text) {
                                            (-1f64)
                                        } else {
                                            1f64
                                        }
                                    })
                                }
                                _ => {
                                    self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Next
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Symbol(left_identity) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Symbol(identity) => {
                                    self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Return(if (left_identity == identity) {
                                        0f64
                                    } else {
                                        if (left_identity < identity) {
                                            (-1f64)
                                        } else {
                                            1f64
                                        }
                                    })
                                }
                                _ => {
                                    self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Object(left_identity_2, _) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                    self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Return(if (left_identity_2 == identity_2) {
                                        0f64
                                    } else {
                                        if (left_identity_2 < identity_2) {
                                            (-1f64)
                                        } else {
                                            1f64
                                        }
                                    })
                                }
                                _ => {
                                    self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                            }
                        }
                        _ => {
                            self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Next
                        }
                    };
                    match source_ml_source2_s803.clone() {
                        self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Next => {
                            if (a == (5f64)) {
                                {
                                    let first = self::read_java_script_semantic_text_length(left.clone());
                                    {
                                        let second = self::read_java_script_semantic_text_length(right.clone());
                                        {
                                            let length = self::ml_math::min(first, second);
                                            {
                                                let index = 0f64;
                                                {
                                                    let source_ml_source2_s808 = self::source_ml_source1_temporary_compare_java_script_collection_keys_loop35(left.clone(), right.clone(), length, index);
                                                    match source_ml_source2_s808.clone() {
                                                        self::MlSource1TemporaryCompareJavaScriptCollectionKeysLoop35Result::MlSource1TemporaryCompareJavaScriptCollectionKeysLoop35Done => {
                                                            if (first == second) {
                                                                0f64
                                                            } else {
                                                                if (first < second) {
                                                                    (-1f64)
                                                                } else {
                                                                    1f64
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryCompareJavaScriptCollectionKeysLoop35Result::MlSource1TemporaryCompareJavaScriptCollectionKeysLoop35Return(source_ml_source1_temporary_result35) => {
                                                            source_ml_source1_temporary_result35
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                0f64
                            }
                        }
                        self::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34::MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Return(source_ml_source1_temporary_result35_2) => {
                            source_ml_source1_temporary_result35_2
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn order_java_script_collection_keys(
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>> {
    {
        let source = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(
            &{
                let ml_reference_value171 = values.borrow().clone();
                ml_reference_value171
            },
            0f64,
            f64::INFINITY,
        )));
        {
            let width = 1f64;
            {
                let source_2 =
                    self::source_ml_source1_temporary_order_java_script_collection_keys_loop36(
                        source.clone(),
                        width,
                    );
                source_2.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_ordinary_object(
    objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>,
    identity: self::ml::Big,
) -> f64 {
    self::find_java_script_heap_directory_object(objects.clone(), identity.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_ordinary_symbol(
    symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    identity: self::ml::Big,
) -> bool {
    self::test_java_script_heap_directory_symbol(symbols.clone(), identity.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_value_error(
    heap: self::JavaScriptOrdinaryHeap,
    value: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    self::read_java_script_heap_directory_value_error(heap.clone(), value.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_key_error(
    heap: self::JavaScriptOrdinaryHeap,
    key: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    self::read_java_script_heap_directory_key_error(heap.clone(), key.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_ordinary_property(
    properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    key: self::JavaScriptSemanticValue,
) -> f64 {
    self::find_java_script_heap_directory_property(properties.clone(), key.clone())
}
