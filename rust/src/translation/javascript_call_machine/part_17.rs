// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayPropertyPlan;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::find_java_script_ordinary_object;
use super::locate_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_array_property_definition;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_source1_temporary_define_java_script_array_own_property_loop120;
use super::source_ml_source3_record_order;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_ordinary_get(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
) -> std::rc::Rc<
    std::cell::RefCell<(
        (
            self::ml_string::Text,
            std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
            self::JavaScriptSemanticValue,
            self::ml_string::Text,
            self::JavaScriptSemanticValue,
            self::JavaScriptSemanticValue,
        ),
        Vec<f64>,
    )>,
> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
            {
                let key_error =
                    self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
                {
                    let receiver_error =
                        self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                    {
                        let error = if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                            heap_error.clone()
                        } else {
                            if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                key_error.clone()
                            } else {
                                if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                    receiver_error.clone()
                                } else {
                                    if ((self::find_java_script_ordinary_object(
                                        shared_constructor_field1.clone(),
                                        identity.clone(),
                                    )) < (0f64))
                                    {
                                        self::ml_string::Text::from_units(vec![
                                            97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32,
                                            71, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98,
                                            106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105,
                                            116, 121,
                                        ])
                                    } else {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                }
                            }
                        };
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_field1438 =
                                    self::JavaScriptSemanticValue::Undefined;
                                {
                                    let source_ml_source2_field1439 =
                                        self::JavaScriptSemanticValue::Undefined;
                                    {
                                        let source_ml_source2_field1441 =
                                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                                self::JavaScriptSemanticValue,
                                            >::new(
                                            )));
                                        {
                                            let source_ml_source3_record_field61 =
                                                self::ml_string::Text::from_units(vec![
                                                    101, 114, 114, 111, 114,
                                                ]);
                                            {
                                                let source_ml_source3_record_field62 =
                                                    source_ml_source2_field1441.clone();
                                                {
                                                    let source_ml_source3_record_field63 =
                                                        source_ml_source2_field1439.clone();
                                                    {
                                                        let source_ml_source3_record_field64 =
                                                            error.clone();
                                                        {
                                                            let source_ml_source3_record_field65 =
                                                                receiver.clone();
                                                            {
                                                                let source_ml_source3_record_field66 =
                                                                    source_ml_source2_field1438
                                                                        .clone();
                                                                ((source_ml_source3_record_field61.clone(), source_ml_source3_record_field62.clone(), source_ml_source3_record_field63.clone(), source_ml_source3_record_field64.clone(), source_ml_source3_record_field65.clone(), source_ml_source3_record_field66.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![97f64, 99f64, 116f64, 105f64, 111f64, 110f64], vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![99f64, 97f64, 108f64, 108f64, 101f64, 101f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64], vec![118f64, 97f64, 108f64, 117f64, 101f64]], vec![0f64, 3f64, 5f64, 2f64, 4f64, 1f64]))
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
                                let target = {
                                    let source_ml_source2_array_read_value1444 =
                                        shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_array_read_index1445 =
                                            self::find_java_script_ordinary_object(
                                                shared_constructor_field1.clone(),
                                                identity.clone(),
                                            );
                                        {
                                            let ml_reference_array214 =
                                                source_ml_source2_array_read_value1444.clone();
                                            let ml_reference_position214 =
                                                source_ml_source2_array_read_index1445;
                                            let ml_reference_projection214 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array214.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position214,
                                                    ),
                                                );
                                            ml_reference_projection214
                                        }
                                    }
                                };
                                if (((((((((!(match (target.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) && match target
                                    .clone()
                                    .expect("a proven present value")
                                {
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => false,
                                    _ => true,
                                }) && match target
                                    .clone()
                                    .expect("a proven present value")
                                {
                                    self::JavaScriptOrdinaryObject::ClosureObject(
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
                                    ) => false,
                                    _ => true,
                                }) && match target
                                    .clone()
                                    .expect("a proven present value")
                                {
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => false,
                                    _ => true,
                                }) && match target.clone().expect("a proven present value")
                                {
                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                        false
                                    }
                                    _ => true,
                                }) && match target.clone().expect("a proven present value")
                                {
                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                        false
                                    }
                                    _ => true,
                                }) && match target.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => false,
                                    _ => true,
                                }) && match target.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => false,
                                    _ => true,
                                }) && match target.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
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
                                    ) => false,
                                    _ => true,
                                }) && match target.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                        false
                                    }
                                    _ => true,
                                }) {
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_field1515 =
                                            self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source2_field1520 = (((match target.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, source_ml_source2_data_property_value1516, _) => {
                                                        let source_ml_source2_data_property_value1516 = *source_ml_source2_data_property_value1516;
                                                        Some((source_ml_source2_data_property_value1516.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, source_ml_source2_data_property_value1517, _) => {
                                                        let source_ml_source2_data_property_value1517 = *source_ml_source2_data_property_value1517;
                                                        Some((source_ml_source2_data_property_value1517.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, source_ml_source2_data_property_value1518, _) => {
                                                        let source_ml_source2_data_property_value1518 = *source_ml_source2_data_property_value1518;
                                                        Some((source_ml_source2_data_property_value1518.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, source_ml_source2_data_property_value1519, _) => {
                                                        let source_ml_source2_data_property_value1519 = *source_ml_source2_data_property_value1519;
                                                        Some((source_ml_source2_data_property_value1519.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone();
                                            {
                                                let source_ml_source2_field1521 =
                                                    self::JavaScriptSemanticValue::Object(
                                                        identity.clone(),
                                                        false,
                                                    );
                                                {
                                                    let source_ml_source2_field1522 =
                                                        std::rc::Rc::new(std::cell::RefCell::new(
                                                            vec![key.clone(), receiver.clone()],
                                                        ));
                                                    {
                                                        let source_ml_source3_record_field67 =
                                                            self::ml_string::Text::from_units(
                                                                vec![99, 97, 108, 108],
                                                            );
                                                        {
                                                            let source_ml_source3_record_field68 =
                                                                source_ml_source2_field1522.clone();
                                                            {
                                                                let source_ml_source3_record_field69 =
                                                                    source_ml_source2_field1520
                                                                        .clone();
                                                                {
                                                                    let source_ml_source3_record_field70 = self::ml_string::Text::from_units(vec![]);
                                                                    {
                                                                        let source_ml_source3_record_field71 = source_ml_source2_field1521.clone();
                                                                        {
                                                                            let source_ml_source3_record_field72 = source_ml_source2_field1515.clone();
                                                                            ((source_ml_source3_record_field67.clone(), source_ml_source3_record_field68.clone(), source_ml_source3_record_field69.clone(), source_ml_source3_record_field70.clone(), source_ml_source3_record_field71.clone(), source_ml_source3_record_field72.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![97f64, 99f64, 116f64, 105f64, 111f64, 110f64], vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![99f64, 97f64, 108f64, 108f64, 101f64, 101f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64], vec![118f64, 97f64, 108f64, 117f64, 101f64]], vec![0f64, 3f64, 5f64, 2f64, 4f64, 1f64]))
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
                                        let property = self::locate_java_script_ordinary_property(
                                            heap.clone(),
                                            identity.clone(),
                                            key.clone(),
                                        );
                                        if !((({
                                            let ml_reference_value215 = property.borrow().clone();
                                            ml_reference_value215
                                        })
                                        .0)
                                            .1)
                                        {
                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source2_field1525 =
                                                    self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source2_field1526 =
                                                        self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source2_field1528 =
                                                            std::rc::Rc::new(
                                                                std::cell::RefCell::new(Vec::<
                                                                    self::JavaScriptSemanticValue,
                                                                >::new(
                                                                )),
                                                            );
                                                        {
                                                            let source_ml_source3_record_field73 =
                                                                self::ml_string::Text::from_units(
                                                                    vec![118, 97, 108, 117, 101],
                                                                );
                                                            {
                                                                let source_ml_source3_record_field74 =
                                                                    source_ml_source2_field1528
                                                                        .clone();
                                                                {
                                                                    let source_ml_source3_record_field75 =
                                                                        source_ml_source2_field1526
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source3_record_field76 = self::ml_string::Text::from_units(vec![]);
                                                                        {
                                                                            let source_ml_source3_record_field77 =
                                                                                receiver.clone();
                                                                            {
                                                                                let source_ml_source3_record_field78 = source_ml_source2_field1525.clone();
                                                                                ((source_ml_source3_record_field73.clone(), source_ml_source3_record_field74.clone(), source_ml_source3_record_field75.clone(), source_ml_source3_record_field76.clone(), source_ml_source3_record_field77.clone(), source_ml_source3_record_field78.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![97f64, 99f64, 116f64, 105f64, 111f64, 110f64], vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![99f64, 97f64, 108f64, 108f64, 101f64, 101f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64], vec![118f64, 97f64, 108f64, 117f64, 101f64]], vec![0f64, 3f64, 5f64, 2f64, 4f64, 1f64]))
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
                                            if (({
                                                let ml_reference_value216 = ((({
                                                    let ml_reference_value217 =
                                                        property.borrow().clone();
                                                    ml_reference_value217
                                                })
                                                .0)
                                                    .0)
                                                    .borrow()
                                                    .clone();
                                                ml_reference_value216
                                            })
                                            .0)
                                                .9
                                            {
                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source2_field1531 = (({
                                                        let ml_reference_value218 = ((({
                                                            let ml_reference_value219 =
                                                                property.borrow().clone();
                                                            ml_reference_value219
                                                        })
                                                        .0)
                                                            .0)
                                                            .borrow()
                                                            .clone();
                                                        ml_reference_value218
                                                    })
                                                    .0)
                                                        .8;
                                                    {
                                                        let source_ml_source2_field1532 = self::JavaScriptSemanticValue::Undefined;
                                                        {
                                                            let source_ml_source2_field1534 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                            {
                                                                let source_ml_source3_record_field79 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                                                {
                                                                    let source_ml_source3_record_field80 =
                                                                        source_ml_source2_field1534
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source3_record_field81 = source_ml_source2_field1532.clone();
                                                                        {
                                                                            let source_ml_source3_record_field82 = self::ml_string::Text::from_units(vec![]);
                                                                            {
                                                                                let source_ml_source3_record_field83 =
                                                                                    receiver.clone();
                                                                                {
                                                                                    let source_ml_source3_record_field84 = source_ml_source2_field1531.clone();
                                                                                    ((source_ml_source3_record_field79.clone(), source_ml_source3_record_field80.clone(), source_ml_source3_record_field81.clone(), source_ml_source3_record_field82.clone(), source_ml_source3_record_field83.clone(), source_ml_source3_record_field84.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![97f64, 99f64, 116f64, 105f64, 111f64, 110f64], vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![99f64, 97f64, 108f64, 108f64, 101f64, 101f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64], vec![118f64, 97f64, 108f64, 117f64, 101f64]], vec![0f64, 3f64, 5f64, 2f64, 4f64, 1f64]))
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
                                                    let getter = (({
                                                        let ml_reference_value220 = ((({
                                                            let ml_reference_value221 =
                                                                property.borrow().clone();
                                                            ml_reference_value221
                                                        })
                                                        .0)
                                                            .0)
                                                            .borrow()
                                                            .clone();
                                                        ml_reference_value220
                                                    })
                                                    .0)
                                                        .4;
                                                    match getter.clone() {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source2_field1537 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source2_field1538 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field1540 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            {
                                                                                let source_ml_source3_record_field85 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                                                                {
                                                                                    let source_ml_source3_record_field86 = source_ml_source2_field1540.clone();
                                                                                    {
                                                                                        let source_ml_source3_record_field87 = source_ml_source2_field1538.clone();
                                                                                        {
                                                                                            let source_ml_source3_record_field88 = self::ml_string::Text::from_units(vec![]);
                                                                                            {
                                                                                                let source_ml_source3_record_field89 = receiver.clone();
                                                                                                {
                                                                                                    let source_ml_source3_record_field90 = source_ml_source2_field1537.clone();
                                                                                                    ((source_ml_source3_record_field85.clone(), source_ml_source3_record_field86.clone(), source_ml_source3_record_field87.clone(), source_ml_source3_record_field88.clone(), source_ml_source3_record_field89.clone(), source_ml_source3_record_field90.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![97f64, 99f64, 116f64, 105f64, 111f64, 110f64], vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![99f64, 97f64, 108f64, 108f64, 101f64, 101f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64], vec![118f64, 97f64, 108f64, 117f64, 101f64]], vec![0f64, 3f64, 5f64, 2f64, 4f64, 1f64]))
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
                                                            _ => {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source2_field1543 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source2_field1546 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                        {
                                                                            let source_ml_source3_record_field91 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                                            {
                                                                                let source_ml_source3_record_field92 = source_ml_source2_field1546.clone();
                                                                                {
                                                                                    let source_ml_source3_record_field93 = getter.clone();
                                                                                    {
                                                                                        let source_ml_source3_record_field94 = self::ml_string::Text::from_units(vec![]);
                                                                                        {
                                                                                            let source_ml_source3_record_field95 = receiver.clone();
                                                                                            {
                                                                                                let source_ml_source3_record_field96 = source_ml_source2_field1543.clone();
                                                                                                ((source_ml_source3_record_field91.clone(), source_ml_source3_record_field92.clone(), source_ml_source3_record_field93.clone(), source_ml_source3_record_field94.clone(), source_ml_source3_record_field95.clone(), source_ml_source3_record_field96.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![97f64, 99f64, 116f64, 105f64, 111f64, 110f64], vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![99f64, 97f64, 108f64, 108f64, 101f64, 101f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64], vec![118f64, 97f64, 108f64, 117f64, 101f64]], vec![0f64, 3f64, 5f64, 2f64, 4f64, 1f64]))
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
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accepts_java_script_ordinary_own_property_definition(
    object: self::JavaScriptOrdinaryObject,
) -> bool {
    ((((((((match object.clone() {
        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => true,
        _ => false,
    } || match object.clone() {
        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => true,
        _ => false,
    }) || match object.clone() {
        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => true,
        _ => false,
    }) || match object.clone() {
        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => true,
        _ => false,
    }) || match object.clone() {
        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => true,
        _ => false,
    }) || match object.clone() {
        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => true,
        _ => false,
    }) || match object.clone() {
        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => true,
        _ => false,
    }) || match object.clone() {
        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
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
        ) => true,
        _ => false,
    }) || match object.clone() {
        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => true,
        _ => false,
    })
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn define_java_script_array_own_property(
    heap: self::JavaScriptOrdinaryHeap,
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
) -> std::rc::Rc<
    std::cell::RefCell<(
        (
            self::ml_string::Text,
            self::JavaScriptOrdinaryHeap,
            Option<(Option<bool>,)>,
            bool,
            std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
        ),
        Vec<f64>,
    )>,
> {
    {
        let plan = self::plan_java_script_array_property_definition(
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
                if (shared_constructor_field4
                    == (self::ml_string::Text::from_units(vec![
                        114, 101, 115, 111, 117, 114, 99, 101,
                    ])))
                {
                    std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_field1619 = shared_constructor_field3.clone();
                        {
                            let source_ml_source2_field1620 =
                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                    self::ml_string::Text,
                                >::new(
                                )));
                            {
                                let source_ml_source2_field1621 = Some((Some(true),));
                                {
                                    let source_ml_source3_record_field97 =
                                        source_ml_source2_field1619.clone();
                                    {
                                        let source_ml_source3_record_field98 = heap.clone();
                                        {
                                            let source_ml_source3_record_field99 =
                                                source_ml_source2_field1621.clone();
                                            {
                                                let source_ml_source3_record_field100 = false;
                                                {
                                                    let source_ml_source3_record_field101 =
                                                        source_ml_source2_field1620.clone();
                                                    ((source_ml_source3_record_field97.clone(), source_ml_source3_record_field98.clone(), source_ml_source3_record_field99.clone(), source_ml_source3_record_field100, source_ml_source3_record_field101.clone()), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64, 76f64, 105f64, 109f64, 105f64, 116f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64], vec![119f64, 114f64, 105f64, 116f64, 101f64, 115f64]], vec![1f64, 3f64, 0f64, 4f64, 2f64], vec![true, true, match source_ml_source3_record_field99.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true]))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }))
                } else {
                    if (shared_constructor_field4 != (self::ml_string::Text::from_units(vec![]))) {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_field1627 = shared_constructor_field3.clone();
                            {
                                let source_ml_source2_field1628 =
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                        self::ml_string::Text,
                                    >::new(
                                    )));
                                {
                                    let source_ml_source3_record_field102 =
                                        source_ml_source2_field1627.clone();
                                    {
                                        let source_ml_source3_record_field103 = heap.clone();
                                        {
                                            let source_ml_source3_record_field104 =
                                                None::<(Option<bool>,)>;
                                            {
                                                let source_ml_source3_record_field105 = false;
                                                {
                                                    let source_ml_source3_record_field106 =
                                                        source_ml_source2_field1628.clone();
                                                    ((source_ml_source3_record_field102.clone(), source_ml_source3_record_field103.clone(), source_ml_source3_record_field104.clone(), source_ml_source3_record_field105, source_ml_source3_record_field106.clone()), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64, 76f64, 105f64, 109f64, 105f64, 116f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64], vec![119f64, 114f64, 105f64, 116f64, 101f64, 115f64]], vec![1f64, 3f64, 0f64, 4f64], vec![true, true, match source_ml_source3_record_field104.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true]))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }))
                    } else {
                        {
                            let count = 0f64;
                            {
                                let source_ml_source1_temporary_values119 = match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                        source_ml_source2_data_property_value1629,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value1629.clone(),
                                };
                                {
                                    let source_ml_source1_temporary_index119 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let count_2 = self::source_ml_source1_temporary_define_java_script_array_own_property_loop120(count, source_ml_source1_temporary_values119.clone(), source_ml_source1_temporary_index119.clone());
                                        if (((count_2
                                            - {
                                                let ml_reference_array222 = match object.clone() {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value1657, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value1657.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value1658, _, _, _) => {
                                                source_ml_source2_data_property_value1658.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value1659) => {
                                                source_ml_source2_data_property_value1659.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value1660, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value1660.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value1661, _) => {
                                                source_ml_source2_data_property_value1661.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value1662, _) => {
                                                source_ml_source2_data_property_value1662.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value1663, _) => {
                                                source_ml_source2_data_property_value1663.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value1664, _) => {
                                                source_ml_source2_data_property_value1664.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value1665) => {
                                                source_ml_source2_data_property_value1665.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value1666, _, _) => {
                                                source_ml_source2_data_property_value1666.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value1667, _, _) => {
                                                source_ml_source2_data_property_value1667.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value1668, _, _) => {
                                                source_ml_source2_data_property_value1668.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value1669, _, _) => {
                                                source_ml_source2_data_property_value1669.clone()
                                            }
                                        };
                                                let ml_reference_projection222 =
                                                    ml_reference_array222.borrow().len() as f64;
                                                ml_reference_projection222
                                            })
                                            + {
                                                let ml_reference_array223 = match shared_constructor_field1.clone() {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value1671, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value1671.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value1672, _, _, _) => {
                                                source_ml_source2_data_property_value1672.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value1673) => {
                                                source_ml_source2_data_property_value1673.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value1674, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value1674.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value1675, _) => {
                                                source_ml_source2_data_property_value1675.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value1676, _) => {
                                                source_ml_source2_data_property_value1676.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value1677, _) => {
                                                source_ml_source2_data_property_value1677.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value1678, _) => {
                                                source_ml_source2_data_property_value1678.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value1679) => {
                                                source_ml_source2_data_property_value1679.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value1680, _, _) => {
                                                source_ml_source2_data_property_value1680.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value1681, _, _) => {
                                                source_ml_source2_data_property_value1681.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value1682, _, _) => {
                                                source_ml_source2_data_property_value1682.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value1683, _, _) => {
                                                source_ml_source2_data_property_value1683.clone()
                                            }
                                        };
                                                let ml_reference_projection223 =
                                                    ml_reference_array223.borrow().len() as f64;
                                                ml_reference_projection223
                                            })
                                            > (16384f64))
                                        {
                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source2_field1687 =
                                                    std::rc::Rc::new(std::cell::RefCell::new(
                                                        Vec::<self::ml_string::Text>::new(),
                                                    ));
                                                {
                                                    let source_ml_source2_field1688 =
                                                        Some((Some(true),));
                                                    {
                                                        let source_ml_source3_record_field107 =
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    111, 114, 100, 105, 110, 97,
                                                                    114, 121, 32, 112, 114, 111,
                                                                    112, 101, 114, 116, 105, 101,
                                                                    115, 32, 101, 120, 99, 101,
                                                                    101, 100, 32, 116, 104, 101,
                                                                    105, 114, 32, 101, 120, 112,
                                                                    108, 105, 99, 105, 116, 32,
                                                                    111, 114, 100, 101, 114, 101,
                                                                    100, 32, 115, 116, 111, 114,
                                                                    97, 103, 101, 32, 98, 117, 100,
                                                                    103, 101, 116,
                                                                ],
                                                            );
                                                        {
                                                            let source_ml_source3_record_field108 =
                                                                heap.clone();
                                                            {
                                                                let source_ml_source3_record_field109 =
                                                                    source_ml_source2_field1688
                                                                        .clone();
                                                                {
                                                                    let source_ml_source3_record_field110 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source3_record_field111 = source_ml_source2_field1687.clone();
                                                                        ((source_ml_source3_record_field107.clone(), source_ml_source3_record_field108.clone(), source_ml_source3_record_field109.clone(), source_ml_source3_record_field110, source_ml_source3_record_field111.clone()), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64, 76f64, 105f64, 109f64, 105f64, 116f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64], vec![119f64, 114f64, 105f64, 116f64, 101f64, 115f64]], vec![1f64, 3f64, 0f64, 4f64, 2f64], vec![true, true, match source_ml_source3_record_field109.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true]))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }))
                                        } else {
                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source2_field1690 =
                                                    self::replace_java_script_ordinary_object(
                                                        heap.clone(),
                                                        shared_constructor_field1.clone(),
                                                    );
                                                {
                                                    let source_ml_source2_field1692 =
                                                        shared_constructor_field2;
                                                    {
                                                        let source_ml_source2_field1694 =
                                                            shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_field1696 =
                                                                shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source3_record_field112 =
                                                                    source_ml_source2_field1694
                                                                        .clone();
                                                                {
                                                                    let source_ml_source3_record_field113 =
                                                                        source_ml_source2_field1690
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source3_record_field114 =
                                                                            None::<(Option<bool>,)>;
                                                                        {
                                                                            let source_ml_source3_record_field115 = source_ml_source2_field1692;
                                                                            {
                                                                                let source_ml_source3_record_field116 = source_ml_source2_field1696.clone();
                                                                                ((source_ml_source3_record_field112.clone(), source_ml_source3_record_field113.clone(), source_ml_source3_record_field114.clone(), source_ml_source3_record_field115, source_ml_source3_record_field116.clone()), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64, 76f64, 105f64, 109f64, 105f64, 116f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64], vec![119f64, 114f64, 105f64, 116f64, 101f64, 115f64]], vec![1f64, 3f64, 0f64, 4f64], vec![true, true, match source_ml_source3_record_field114.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true]))
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
    }
}
