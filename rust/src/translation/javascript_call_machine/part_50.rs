// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptObjectCopyCall;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptObjectSpreadTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSemanticValue;
use super::accept_java_script_object_assignment_array;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::finish_java_script_object_assignment_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_object_spread_state_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_unit;
use super::resume_java_script_array_assignment;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_object_spread_property(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectSpreadState,
    key: self::JavaScriptSemanticValue,
) -> self::JavaScriptObjectSpreadTransition {
    match state.clone() {
        self::JavaScriptObjectSpreadState::ObjectSpreadState(
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
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    if {
                        let ml_values639 = {
                            let ml_reference_value640 = shared_constructor_field4.borrow().clone();
                            ml_reference_value640
                        };
                        let ml_callback639 = {
                            let key = key.clone();
                            std::rc::Rc::new(
                                move |excluded: self::JavaScriptSemanticValue| -> bool {
                                    self::test_java_script_semantic_same_value(
                                        excluded.clone(),
                                        key.clone(),
                                    )
                                },
                            )
                        };
                        ml_values639
                            .iter()
                            .cloned()
                            .enumerate()
                            .any(|(ml_index639, ml_value639)| ml_callback639(ml_value639))
                    } {
                        self::JavaScriptObjectSpreadTransition::ObjectSpreadContinue(
                            Box::new(heap.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field8033 =
                                    shared_constructor_field2.clone();
                                {
                                    let source_ml_source2_nominal_field8035 =
                                        shared_constructor_field3.clone();
                                    {
                                        let source_ml_source2_nominal_field8037 =
                                            shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_nominal_field8039 =
                                                shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_nominal_field8041 =
                                                    shared_constructor_field6;
                                                {
                                                    let source_ml_source2_nominal_field8043 =
                                                        shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_nominal_field8045 =
                                                            shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_nominal_field8047 =
                                                                shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_nominal_field8049 =
                                                                    shared_constructor_field10
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field8051 =
                                                                        shared_constructor_field11
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field8053 = shared_constructor_field12.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field8055 = shared_constructor_field13.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_override8057 = (shared_constructor_field6 + 1f64);
                                                                                self::JavaScriptObjectSpreadState::ObjectSpreadState(Box::new(source_ml_source2_nominal_field8033.clone()), Box::new(source_ml_source2_nominal_field8035.clone()), source_ml_source2_nominal_field8037.clone(), source_ml_source2_nominal_field8039.clone(), source_ml_source2_nominal_override8057, source_ml_source2_nominal_field8043.clone(), Box::new(source_ml_source2_nominal_field8045.clone()), Box::new(source_ml_source2_nominal_field8047.clone()), source_ml_source2_nominal_field8049.clone(), source_ml_source2_nominal_field8051.clone(), source_ml_source2_nominal_field8053.clone(), source_ml_source2_nominal_field8055.clone())
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
                        if {
                            let source_ml_source2_s8059 = shared_constructor_field3.clone();
                            match source_ml_source2_s8059.clone() {
                                self::JavaScriptSemanticValue::Object(_, _) => true,
                                _ => false,
                            }
                        } {
                            {
                                let input = {
                                    let source_ml_source2_array_read_value8067 =
                                        shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_array_read_index8068 =
                                            self::find_java_script_ordinary_object(
                                                shared_constructor_field1.clone(),
                                                (((match shared_constructor_field3.clone() {
                                                    self::JavaScriptSemanticValue::Object(
                                                        source_ml_source2_data_property_value8066,
                                                        _,
                                                    ) => Some((
                                                        source_ml_source2_data_property_value8066
                                                            .clone(),
                                                    )),
                                                    _ => None::<(self::ml::Big,)>,
                                                })
                                                .clone()
                                                .expect("a proven present value"))
                                                .0)
                                                    .clone(),
                                            );
                                        {
                                            let ml_reference_array641 =
                                                source_ml_source2_array_read_value8067.clone();
                                            let ml_reference_position641 =
                                                source_ml_source2_array_read_index8068;
                                            let ml_reference_projection641 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array641.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position641,
                                                    ),
                                                );
                                            ml_reference_projection641
                                        }
                                    }
                                };
                                if match (input.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97,
                                            100, 32, 115, 111, 117, 114, 99, 101, 32, 99, 97, 110,
                                            110, 111, 116, 32, 100, 105, 115, 97, 112, 112, 101,
                                            97, 114, 32, 102, 114, 111, 109, 32, 105, 116, 115, 32,
                                            103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101,
                                            120, 116,
                                        ]),
                                    )
                                } else {
                                    if !(self::ml_array::includes(&{ let ml_reference_value642 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value642 }, &match input.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                            self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
                                        }
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                            self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
                                        }
                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                        }
                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                            self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
                                        }
                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                            self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                                        }
                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                            self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
                                        }
                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                            self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
                                        }
                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                            self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
                                        }
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
                                        }
                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
                                        }
                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
                                        }
                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                        }
                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
                                        }
                                    }, 0f64, false, |a, b| a == b)) {
                                        self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 45, 107, 101, 121, 32, 97, 110, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108, 115]))
                                    } else {
                                        {
                                            let property = {
                                                let source_ml_source2_array_read_value8095 = match input.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value8069, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value8069.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value8070, _, _, _) => {
                                                        source_ml_source2_data_property_value8070.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value8071) => {
                                                        source_ml_source2_data_property_value8071.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value8072, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value8072.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value8073, _) => {
                                                        source_ml_source2_data_property_value8073.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value8074, _) => {
                                                        source_ml_source2_data_property_value8074.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value8075, _) => {
                                                        source_ml_source2_data_property_value8075.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value8076, _) => {
                                                        source_ml_source2_data_property_value8076.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value8077) => {
                                                        source_ml_source2_data_property_value8077.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value8078, _, _) => {
                                                        source_ml_source2_data_property_value8078.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value8079, _, _) => {
                                                        source_ml_source2_data_property_value8079.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value8080, _, _) => {
                                                        source_ml_source2_data_property_value8080.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value8081, _, _) => {
                                                        source_ml_source2_data_property_value8081.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_array_read_index8096 = self::find_java_script_ordinary_property(match input.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value8082, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value8082.clone()
                                                        }
                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value8083, _, _, _) => {
                                                            source_ml_source2_data_property_value8083.clone()
                                                        }
                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value8084) => {
                                                            source_ml_source2_data_property_value8084.clone()
                                                        }
                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value8085, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value8085.clone()
                                                        }
                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value8086, _) => {
                                                            source_ml_source2_data_property_value8086.clone()
                                                        }
                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value8087, _) => {
                                                            source_ml_source2_data_property_value8087.clone()
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value8088, _) => {
                                                            source_ml_source2_data_property_value8088.clone()
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value8089, _) => {
                                                            source_ml_source2_data_property_value8089.clone()
                                                        }
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value8090) => {
                                                            source_ml_source2_data_property_value8090.clone()
                                                        }
                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value8091, _, _) => {
                                                            source_ml_source2_data_property_value8091.clone()
                                                        }
                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value8092, _, _) => {
                                                            source_ml_source2_data_property_value8092.clone()
                                                        }
                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value8093, _, _) => {
                                                            source_ml_source2_data_property_value8093.clone()
                                                        }
                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value8094, _, _) => {
                                                            source_ml_source2_data_property_value8094.clone()
                                                        }
                                                    }, key.clone());
                                                    { let ml_reference_array643 = source_ml_source2_array_read_value8095.clone(); let ml_reference_position643 = source_ml_source2_array_read_index8096; let ml_reference_projection643 = self::ml_array::optional_at(&*ml_reference_array643.borrow(), self::ml_array::number_index(ml_reference_position643)); ml_reference_projection643 }
                                                }
                                            };
                                            if (match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !((({ let ml_reference_value644 = (match property.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value8097) => {
                                                    source_ml_source2_data_property_value8097.clone()
                                                }
                                            }).borrow().clone(); ml_reference_value644 }).0).2)) {
                                                self::JavaScriptObjectSpreadTransition::ObjectSpreadContinue(Box::new(heap.clone()), Box::new({
                                                    let source_ml_source2_nominal_field8100 = shared_constructor_field2.clone();
                                                    {
                                                        let source_ml_source2_nominal_field8102 = shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field8104 = shared_constructor_field4.clone();
                                                            {
                                                                let source_ml_source2_nominal_field8106 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field8108 = shared_constructor_field6;
                                                                    {
                                                                        let source_ml_source2_nominal_field8110 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field8112 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field8114 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field8116 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field8118 = shared_constructor_field11.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field8120 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field8122 = shared_constructor_field13.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override8124 = (shared_constructor_field6 + 1f64);
                                                                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(Box::new(source_ml_source2_nominal_field8100.clone()), Box::new(source_ml_source2_nominal_field8102.clone()), source_ml_source2_nominal_field8104.clone(), source_ml_source2_nominal_field8106.clone(), source_ml_source2_nominal_override8124, source_ml_source2_nominal_field8110.clone(), Box::new(source_ml_source2_nominal_field8112.clone()), Box::new(source_ml_source2_nominal_field8114.clone()), source_ml_source2_nominal_field8116.clone(), source_ml_source2_nominal_field8118.clone(), source_ml_source2_nominal_field8120.clone(), source_ml_source2_nominal_field8122.clone())
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
                                                {
                                                    let get = self::plan_java_script_ordinary_get(heap.clone(), (((match shared_constructor_field3.clone() {
                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value8126, _) => {
                                                            Some((source_ml_source2_data_property_value8126.clone(),))
                                                        }
                                                        _ => {
                                                            None::<(self::ml::Big,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0).clone(), key.clone(), shared_constructor_field3.clone());
                                                    if (((({ let ml_reference_value645 = get.borrow().clone(); ml_reference_value645 }).0).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                        self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(Box::new(heap.clone()), (({ let ml_reference_value646 = get.borrow().clone(); ml_reference_value646 }).0).3)
                                                    } else {
                                                        if (((({ let ml_reference_value647 = get.borrow().clone(); ml_reference_value647 }).0).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                            {
                                                                let call = self::JavaScriptObjectCopyCall::ObjectCopyCall(Box::new((({ let ml_reference_value648 = get.borrow().clone(); ml_reference_value648 }).0).2), Box::new((({ let ml_reference_value649 = get.borrow().clone(); ml_reference_value649 }).0).4), (({ let ml_reference_value650 = get.borrow().clone(); ml_reference_value650 }).0).1);
                                                                self::JavaScriptObjectSpreadTransition::ObjectSpreadCall(Box::new(heap.clone()), Box::new(if {
                                                                    let source_ml_source2_comparison_operand8129 = match state.clone() {
                                                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value8128, _, _, _) => {
                                                                            ((source_ml_source2_data_property_value8128.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                        }
                                                                    };
                                                                    match (source_ml_source2_comparison_operand8129.clone(), self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                } {
                                                                    {
                                                                        let source_ml_source2_nominal_field8132 = shared_constructor_field2.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field8134 = shared_constructor_field3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field8136 = shared_constructor_field4.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field8138 = shared_constructor_field5.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field8140 = shared_constructor_field6;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field8142 = shared_constructor_field7.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field8144 = shared_constructor_field8.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field8146 = shared_constructor_field9.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field8148 = shared_constructor_field10.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field8150 = shared_constructor_field11.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field8152 = shared_constructor_field12.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field8154 = shared_constructor_field13.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override8157 = (({ let ml_reference_value651 = get.borrow().clone(); ml_reference_value651 }).0).2;
                                                                                                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(Box::new(source_ml_source2_nominal_field8132.clone()), Box::new(source_ml_source2_nominal_field8134.clone()), source_ml_source2_nominal_field8136.clone(), source_ml_source2_nominal_field8138.clone(), source_ml_source2_nominal_field8140, self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]), Box::new(source_ml_source2_nominal_field8144.clone()), Box::new(source_ml_source2_nominal_override8157.clone()), source_ml_source2_nominal_field8148.clone(), source_ml_source2_nominal_field8150.clone(), source_ml_source2_nominal_field8152.clone(), Some((Some(call.clone()),)))
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
                                                                        let source_ml_source2_nominal_field8160 = shared_constructor_field2.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field8162 = shared_constructor_field3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field8164 = shared_constructor_field4.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field8166 = shared_constructor_field5.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field8168 = shared_constructor_field6;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field8170 = shared_constructor_field7.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field8172 = shared_constructor_field8.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field8174 = shared_constructor_field9.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field8176 = shared_constructor_field10.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field8178 = shared_constructor_field11.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field8180 = shared_constructor_field12.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field8182 = shared_constructor_field13.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override8185 = (({ let ml_reference_value652 = get.borrow().clone(); ml_reference_value652 }).0).2;
                                                                                                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(Box::new(source_ml_source2_nominal_field8160.clone()), Box::new(source_ml_source2_nominal_field8162.clone()), source_ml_source2_nominal_field8164.clone(), source_ml_source2_nominal_field8166.clone(), source_ml_source2_nominal_field8168, self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]), Box::new(source_ml_source2_nominal_field8172.clone()), Box::new(source_ml_source2_nominal_override8185.clone()), source_ml_source2_nominal_field8176.clone(), source_ml_source2_nominal_field8178.clone(), source_ml_source2_nominal_field8180.clone(), source_ml_source2_nominal_field8182.clone())
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
                                                                }), Box::new((({ let ml_reference_value653 = get.borrow().clone(); ml_reference_value653 }).0).2), Box::new((({ let ml_reference_value654 = get.borrow().clone(); ml_reference_value654 }).0).4), (({ let ml_reference_value655 = get.borrow().clone(); ml_reference_value655 }).0).1)
                                                            }
                                                        } else {
                                                            if (((({ let ml_reference_value656 = get.borrow().clone(); ml_reference_value656 }).0).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 71, 101, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                            } else {
                                                                self::JavaScriptObjectSpreadTransition::ObjectSpreadContinue(Box::new(heap.clone()), Box::new({
                                                                    let source_ml_source2_nominal_field8187 = shared_constructor_field2.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field8189 = shared_constructor_field3.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field8191 = shared_constructor_field4.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field8193 = shared_constructor_field5.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field8195 = shared_constructor_field6;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field8197 = shared_constructor_field7.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field8199 = shared_constructor_field8.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field8201 = shared_constructor_field9.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field8203 = shared_constructor_field10.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field8205 = shared_constructor_field11.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field8207 = shared_constructor_field12.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field8209 = shared_constructor_field13.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override8212 = (({ let ml_reference_value657 = get.borrow().clone(); ml_reference_value657 }).0).5;
                                                                                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(Box::new(source_ml_source2_nominal_field8187.clone()), Box::new(source_ml_source2_nominal_field8189.clone()), source_ml_source2_nominal_field8191.clone(), source_ml_source2_nominal_field8193.clone(), source_ml_source2_nominal_field8195, self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), Box::new(source_ml_source2_nominal_override8212.clone()), Box::new(source_ml_source2_nominal_field8201.clone()), source_ml_source2_nominal_field8203.clone(), source_ml_source2_nominal_field8205.clone(), source_ml_source2_nominal_field8207.clone(), source_ml_source2_nominal_field8209.clone())
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
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if ({
                                let source_ml_source2_s8214 = shared_constructor_field3.clone();
                                match source_ml_source2_s8214.clone() {
                                    self::JavaScriptSemanticValue::Text(_) => true,
                                    _ => false,
                                }
                            } || {
                                let source_ml_source2_s8218 = shared_constructor_field3.clone();
                                match source_ml_source2_s8218.clone() {
                                    self::JavaScriptSemanticValue::SourceText(_, _) => true,
                                    _ => false,
                                }
                            }) {
                                self::JavaScriptObjectSpreadTransition::ObjectSpreadContinue(
                                    Box::new(heap.clone()),
                                    Box::new({
                                        let source_ml_source2_nominal_field8225 =
                                            shared_constructor_field2.clone();
                                        {
                                            let source_ml_source2_nominal_field8227 =
                                                shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field8229 =
                                                    shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_nominal_field8231 =
                                                        shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_nominal_field8233 =
                                                            shared_constructor_field6;
                                                        {
                                                            let source_ml_source2_nominal_field8235 =
                                                                shared_constructor_field7.clone();
                                                            {
                                                                let source_ml_source2_nominal_field8237 =
                                                                    shared_constructor_field8
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field8239 =
                                                                        shared_constructor_field9
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field8241 = shared_constructor_field10.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field8243 = shared_constructor_field11.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field8245 = shared_constructor_field12.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field8247 = shared_constructor_field13.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_override8250 = self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![self::read_java_script_semantic_text_unit(shared_constructor_field3.clone(), shared_constructor_field6)])));
                                                                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(Box::new(source_ml_source2_nominal_field8225.clone()), Box::new(source_ml_source2_nominal_field8227.clone()), source_ml_source2_nominal_field8229.clone(), source_ml_source2_nominal_field8231.clone(), source_ml_source2_nominal_field8233, self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), Box::new(source_ml_source2_nominal_override8250.clone()), Box::new(source_ml_source2_nominal_field8239.clone()), source_ml_source2_nominal_field8241.clone(), source_ml_source2_nominal_field8243.clone(), source_ml_source2_nominal_field8245.clone(), source_ml_source2_nominal_field8247.clone())
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
                                self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
                                        119, 105, 116, 104, 111, 117, 116, 32, 111, 119, 110, 32,
                                        107, 101, 121, 115, 32, 99, 97, 110, 110, 111, 116, 32,
                                        114, 101, 116, 97, 105, 110, 32, 115, 101, 108, 101, 99,
                                        116, 101, 100, 32, 115, 112, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115,
                                    ]),
                                )
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
pub fn resume_java_script_object_spread(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectSpreadState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptObjectSpreadTransition {
    match state.clone() {
        self::JavaScriptObjectSpreadState::ObjectSpreadState(
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
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            {
                let error =
                    self::read_java_script_object_spread_state_error(heap.clone(), state.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if (((shared_constructor_field6
                        != (self::ml_string::Text::from_units(vec![
                            103, 101, 116, 116, 101, 114,
                        ])))
                        && (shared_constructor_field6
                            != (self::ml_string::Text::from_units(vec![
                                115, 101, 116, 116, 101, 114,
                            ]))))
                        && (shared_constructor_field6
                            != (self::ml_string::Text::from_units(vec![
                                97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105,
                                111, 110,
                            ]))))
                    {
                        self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                111, 98, 106, 101, 99, 116, 32, 99, 111, 112, 121, 32, 114, 101,
                                115, 117, 109, 101, 115, 32, 111, 110, 108, 121, 32, 97, 110, 32,
                                97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101,
                                100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108,
                            ]),
                        )
                    } else {
                        {
                            let value_error = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source2_data_property_value8257) => {
                                    let source_ml_source2_data_property_value8257 = *source_ml_source2_data_property_value8257;
                                    source_ml_source2_data_property_value8257.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source2_data_property_value8258) => {
                                    let source_ml_source2_data_property_value8258 = *source_ml_source2_data_property_value8258;
                                    source_ml_source2_data_property_value8258.clone()
                                }
                            });
                            if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(
                                    Box::new(heap.clone()),
                                    value_error.clone(),
                                )
                            } else {
                                match completion.clone() {
                                    self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(completion_value) => {
                                        let completion_value = *completion_value;
                                        if (shared_constructor_field6 == (self::ml_string::Text::from_units(vec![115, 101, 116, 116, 101, 114]))) {
                                            self::finish_java_script_object_assignment_property(heap.clone(), state.clone())
                                        } else {
                                            if ((shared_constructor_field6 == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))) && !(match (match state.clone() {
                                                self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8263, _) => {
                                                    ((source_ml_source2_data_property_value8263.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                }
                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                self::accept_java_script_object_assignment_array(heap.clone(), state.clone(), self::resume_java_script_array_assignment(heap.clone(), (match state.clone() {
                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8264, _) => {
                                                        ((source_ml_source2_data_property_value8264.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value"), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(completion_value.clone()))))
                                            } else {
                                                self::JavaScriptObjectSpreadTransition::ObjectSpreadContinue(Box::new(heap.clone()), Box::new({
                                                    let source_ml_source2_nominal_field8266 = shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_nominal_field8268 = shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_nominal_field8270 = shared_constructor_field3.clone();
                                                            {
                                                                let source_ml_source2_nominal_field8272 = shared_constructor_field4.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field8274 = shared_constructor_field5;
                                                                    {
                                                                        let source_ml_source2_nominal_field8276 = shared_constructor_field6.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field8278 = shared_constructor_field7.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field8280 = shared_constructor_field8.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field8282 = shared_constructor_field9.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field8284 = shared_constructor_field10.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field8286 = shared_constructor_field11.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field8288 = shared_constructor_field12.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override8292 = self::JavaScriptSemanticValue::Undefined;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override8293 = None::<self::JavaScriptObjectCopyCall>;
                                                                                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(Box::new(source_ml_source2_nominal_field8266.clone()), Box::new(source_ml_source2_nominal_field8268.clone()), source_ml_source2_nominal_field8270.clone(), source_ml_source2_nominal_field8272.clone(), source_ml_source2_nominal_field8274, self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), Box::new(completion_value.clone()), Box::new(source_ml_source2_nominal_override8292.clone()), source_ml_source2_nominal_field8282.clone(), source_ml_source2_nominal_field8284.clone(), source_ml_source2_nominal_field8286.clone(), Some((source_ml_source2_nominal_override8293.clone(),)))
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
                                        }
                                    }
                                    self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value) => {
                                        let value = *value;
                                        self::JavaScriptObjectSpreadTransition::ObjectSpreadThrow(Box::new(heap.clone()), Box::new(value.clone()))
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
pub fn begin_java_script_object_assignment(
    heap: self::JavaScriptOrdinaryHeap,
    target: self::JavaScriptSemanticValue,
    input: self::JavaScriptSemanticValue,
    exotic_key: self::JavaScriptSemanticValue,
) -> self::JavaScriptObjectSpreadTransition {
    {
        let state = {
            let source_ml_source2_constructor_field8296 = std::rc::Rc::new(
                std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()),
            );
            {
                let source_ml_source2_constructor_field8297 = std::rc::Rc::new(
                    std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()),
                );
                {
                    let source_ml_source2_constructor_field8298 = 0f64;
                    {
                        let source_ml_source2_constructor_field8300 =
                            self::JavaScriptSemanticValue::Undefined;
                        {
                            let source_ml_source2_constructor_field8301 =
                                self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source2_constructor_field8302 =
                                    Some(self::ml_string::Text::from_units(vec![
                                        97, 115, 115, 105, 103, 110, 109, 101, 110, 116,
                                    ]));
                                {
                                    let source_ml_source2_constructor_field8303 =
                                        Some(exotic_key.clone());
                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(
                                        Box::new(target.clone()),
                                        Box::new(input.clone()),
                                        source_ml_source2_constructor_field8296.clone(),
                                        source_ml_source2_constructor_field8297.clone(),
                                        source_ml_source2_constructor_field8298,
                                        self::ml_string::Text::from_units(vec![107, 101, 121, 115]),
                                        Box::new(source_ml_source2_constructor_field8300.clone()),
                                        Box::new(source_ml_source2_constructor_field8301.clone()),
                                        Some((source_ml_source2_constructor_field8302.clone(),)),
                                        Some((source_ml_source2_constructor_field8303.clone(),)),
                                        None::<(Option<self::JavaScriptArrayAssignmentState>,)>,
                                        None::<(Option<self::JavaScriptObjectCopyCall>,)>,
                                    )
                                }
                            }
                        }
                    }
                }
            }
        };
        {
            let invalid =
                self::read_java_script_object_spread_state_error(heap.clone(), state.clone());
            if (invalid == (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptObjectSpreadTransition::ObjectSpreadContinue(
                    Box::new(heap.clone()),
                    Box::new(state.clone()),
                )
            } else {
                self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(
                    Box::new(heap.clone()),
                    invalid.clone(),
                )
            }
        }
    }
}
