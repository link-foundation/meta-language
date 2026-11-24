pub fn plan_java_script_ordinary_get(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, self::JavaScriptSemanticValue, self::ml_string::Text, self::JavaScriptSemanticValue, self::JavaScriptSemanticValue)>> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                {
                    let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
                    {
                        let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
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
                                        if ((self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone())) < (0f64)) {
                                            self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121])
                                        } else {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                    }
                                }
                            };
                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source1_field5691 = self::JavaScriptSemanticValue::Undefined;
                                    {
                                        let source_ml_source1_field5692 = self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source1_field5694 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                            (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), source_ml_source1_field5694.clone(), source_ml_source1_field5692.clone(), error.clone(), receiver.clone(), source_ml_source1_field5691.clone())
                                        }
                                    }
                                }))
                            } else {
                                {
                                    let target = {
                                        let source_ml_source1_array_read_value5697 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source1_array_read_index5698 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                            { let ml_reference_array214 = source_ml_source1_array_read_value5697.clone(); let ml_reference_position214 = source_ml_source1_array_read_index5698; let ml_reference_projection214 = self::ml_array::optional_at(&*ml_reference_array214.borrow(), self::ml_array::number_index(ml_reference_position214)); ml_reference_projection214 }
                                        }
                                    };
                                    if (((((((((!(match (target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && match target.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }) && match target.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }) && match target.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }) && match target.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }) && match target.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }) && match target.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }) && match target.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }) && match target.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }) && match target.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }) {
                                        std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source1_field6556 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field6561 = (((match target.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, source_ml_source1_data_property_value6557, _) => {
                                                        let source_ml_source1_data_property_value6557 = *source_ml_source1_data_property_value6557;
                                                        Some((source_ml_source1_data_property_value6557.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, source_ml_source1_data_property_value6558, _) => {
                                                        let source_ml_source1_data_property_value6558 = *source_ml_source1_data_property_value6558;
                                                        Some((source_ml_source1_data_property_value6558.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, source_ml_source1_data_property_value6559, _) => {
                                                        let source_ml_source1_data_property_value6559 = *source_ml_source1_data_property_value6559;
                                                        Some((source_ml_source1_data_property_value6559.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, source_ml_source1_data_property_value6560, _) => {
                                                        let source_ml_source1_data_property_value6560 = *source_ml_source1_data_property_value6560;
                                                        Some((source_ml_source1_data_property_value6560.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone();
                                                {
                                                    let source_ml_source1_field6562 = self::JavaScriptSemanticValue::Object(identity.clone(), false);
                                                    {
                                                        let source_ml_source1_field6563 = std::rc::Rc::new(std::cell::RefCell::new(vec![key.clone(), receiver.clone()]));
                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source1_field6563.clone(), source_ml_source1_field6561.clone(), self::ml_string::Text::from_units(vec![]), source_ml_source1_field6562.clone(), source_ml_source1_field6556.clone())
                                                    }
                                                }
                                            }
                                        }))
                                    } else {
                                        {
                                            let property = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                                            if !(({ let ml_reference_value215 = property.borrow().clone(); ml_reference_value215 }).1) {
                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source1_field6566 = self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source1_field6567 = self::JavaScriptSemanticValue::Undefined;
                                                        {
                                                            let source_ml_source1_field6569 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                            (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]), source_ml_source1_field6569.clone(), source_ml_source1_field6567.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source1_field6566.clone())
                                                        }
                                                    }
                                                }))
                                            } else {
                                                if ({ let ml_reference_value216 = (({ let ml_reference_value217 = property.borrow().clone(); ml_reference_value217 }).0).borrow().clone(); ml_reference_value216 }).9 {
                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                        let source_ml_source1_field6572 = ({ let ml_reference_value218 = (({ let ml_reference_value219 = property.borrow().clone(); ml_reference_value219 }).0).borrow().clone(); ml_reference_value218 }).8;
                                                        {
                                                            let source_ml_source1_field6573 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field6575 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]), source_ml_source1_field6575.clone(), source_ml_source1_field6573.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source1_field6572.clone())
                                                            }
                                                        }
                                                    }))
                                                } else {
                                                    {
                                                        let getter = ({ let ml_reference_value220 = (({ let ml_reference_value221 = property.borrow().clone(); ml_reference_value221 }).0).borrow().clone(); ml_reference_value220 }).4;
                                                        match getter.clone() {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field6587 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field6588 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field6590 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]), source_ml_source1_field6590.clone(), source_ml_source1_field6588.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source1_field6587.clone())
                                                                        }
                                                                    }
                                                                }))
                                                            }
                                                            self::JavaScriptSemanticValue::Null => {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field6593 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field6596 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source1_field6596.clone(), getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source1_field6593.clone())
                                                                    }
                                                                }))
                                                            }
                                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field6599 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field6602 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source1_field6602.clone(), getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source1_field6599.clone())
                                                                    }
                                                                }))
                                                            }
                                                            self::JavaScriptSemanticValue::Number(_) => {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field6605 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field6608 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source1_field6608.clone(), getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source1_field6605.clone())
                                                                    }
                                                                }))
                                                            }
                                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field6611 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field6614 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source1_field6614.clone(), getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source1_field6611.clone())
                                                                    }
                                                                }))
                                                            }
                                                            self::JavaScriptSemanticValue::Text(_) => {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field6617 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field6620 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source1_field6620.clone(), getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source1_field6617.clone())
                                                                    }
                                                                }))
                                                            }
                                                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field6623 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field6626 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source1_field6626.clone(), getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source1_field6623.clone())
                                                                    }
                                                                }))
                                                            }
                                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field6629 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field6632 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source1_field6632.clone(), getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source1_field6629.clone())
                                                                    }
                                                                }))
                                                            }
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field6635 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field6638 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                        (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source1_field6638.clone(), getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source1_field6635.clone())
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
}

pub fn accepts_java_script_ordinary_own_property_definition(object: self::JavaScriptOrdinaryObject) -> bool {
    ((((((((match object.clone() {
        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
            true
        }
        _ => {
            false
        }
    } || match object.clone() {
        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
            true
        }
        _ => {
            false
        }
    }) || match object.clone() {
        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
            true
        }
        _ => {
            false
        }
    }) || match object.clone() {
        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
            true
        }
        _ => {
            false
        }
    }) || match object.clone() {
        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
            true
        }
        _ => {
            false
        }
    }) || match object.clone() {
        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
            true
        }
        _ => {
            false
        }
    }) || match object.clone() {
        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
            true
        }
        _ => {
            false
        }
    }) || match object.clone() {
        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
            true
        }
        _ => {
            false
        }
    }) || match object.clone() {
        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
            true
        }
        _ => {
            false
        }
    })
}

pub fn define_java_script_array_own_property(heap: self::JavaScriptOrdinaryHeap, object: self::JavaScriptOrdinaryObject, key: self::JavaScriptSemanticValue, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, self::JavaScriptOrdinaryHeap, Option<(Option<bool>,)>, bool, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>)>> {
    {
        let plan = self::plan_java_script_array_property_definition(object.clone(), key.clone(), proposed.clone());
        match plan.clone() {
            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5) => {
                let shared_constructor_field1 = *shared_constructor_field1;
                if (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![114, 101, 115, 111, 117, 114, 99, 101]))) {
                    std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source1_field7499 = shared_constructor_field3.clone();
                        {
                            let source_ml_source1_field7500 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                            {
                                let source_ml_source1_field7501 = Some((Some(true),));
                                (source_ml_source1_field7499.clone(), heap.clone(), source_ml_source1_field7501.clone(), false, source_ml_source1_field7500.clone())
                            }
                        }
                    }))
                } else {
                    if (shared_constructor_field4 != (self::ml_string::Text::from_units(vec![]))) {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source1_field7507 = shared_constructor_field3.clone();
                            {
                                let source_ml_source1_field7508 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                (source_ml_source1_field7507.clone(), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7508.clone())
                            }
                        }))
                    } else {
                        {
                            let count = 0f64;
                            {
                                let source_ml_temporary_values104 = match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value7509, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value7509.clone()
                                    }
                                };
                                {
                                    let source_ml_temporary_index104 = self::ml::Big::from_i128(0);
                                    {
                                        let count_2 = self::source_ml_temporary_define_java_script_array_own_property_loop105(count, source_ml_temporary_values104.clone(), source_ml_temporary_index104.clone());
                                        if ((((count_2 - { let ml_reference_array222 = match object.clone() {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value7537, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value7537.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value7538, _, _, _) => {
                                                source_ml_source1_data_property_value7538.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value7539) => {
                                                source_ml_source1_data_property_value7539.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value7540, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value7540.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value7541, _) => {
                                                source_ml_source1_data_property_value7541.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value7542, _) => {
                                                source_ml_source1_data_property_value7542.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value7543, _) => {
                                                source_ml_source1_data_property_value7543.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value7544, _) => {
                                                source_ml_source1_data_property_value7544.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value7545) => {
                                                source_ml_source1_data_property_value7545.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value7546, _, _) => {
                                                source_ml_source1_data_property_value7546.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value7547, _, _) => {
                                                source_ml_source1_data_property_value7547.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value7548, _, _) => {
                                                source_ml_source1_data_property_value7548.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value7549, _, _) => {
                                                source_ml_source1_data_property_value7549.clone()
                                            }
                                        }; let ml_reference_projection222 = ml_reference_array222.borrow().len() as f64; ml_reference_projection222 }) + { let ml_reference_array223 = match shared_constructor_field1.clone() {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value7551, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value7551.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value7552, _, _, _) => {
                                                source_ml_source1_data_property_value7552.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value7553) => {
                                                source_ml_source1_data_property_value7553.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value7554, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value7554.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value7555, _) => {
                                                source_ml_source1_data_property_value7555.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value7556, _) => {
                                                source_ml_source1_data_property_value7556.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value7557, _) => {
                                                source_ml_source1_data_property_value7557.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value7558, _) => {
                                                source_ml_source1_data_property_value7558.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value7559) => {
                                                source_ml_source1_data_property_value7559.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value7560, _, _) => {
                                                source_ml_source1_data_property_value7560.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value7561, _, _) => {
                                                source_ml_source1_data_property_value7561.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value7562, _, _) => {
                                                source_ml_source1_data_property_value7562.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value7563, _, _) => {
                                                source_ml_source1_data_property_value7563.clone()
                                            }
                                        }; let ml_reference_projection223 = ml_reference_array223.borrow().len() as f64; ml_reference_projection223 })) > (16384f64)) {
                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source1_field7567 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                {
                                                    let source_ml_source1_field7568 = Some((Some(true),));
                                                    (self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]), heap.clone(), source_ml_source1_field7568.clone(), false, source_ml_source1_field7567.clone())
                                                }
                                            }))
                                        } else {
                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source1_field7570 = self::replace_java_script_ordinary_object(heap.clone(), shared_constructor_field1.clone());
                                                {
                                                    let source_ml_source1_field7572 = shared_constructor_field2;
                                                    {
                                                        let source_ml_source1_field7574 = shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source1_field7576 = shared_constructor_field5.clone();
                                                            (source_ml_source1_field7574.clone(), source_ml_source1_field7570.clone(), None::<(Option<bool>,)>, source_ml_source1_field7572, source_ml_source1_field7576.clone())
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

pub fn define_java_script_ordinary_property(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, self::JavaScriptOrdinaryHeap, Option<(Option<bool>,)>, bool, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>)>> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                {
                    let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
                    {
                        let error = if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                            heap_error.clone()
                        } else {
                            key_error.clone()
                        };
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source1_field7580 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                (error.clone(), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7580.clone())
                            }))
                        } else {
                            if (({ let ml_reference_value224 = proposed.borrow().clone(); ml_reference_value224 }).9 && ((self::read_java_script_ordinary_value_error(heap.clone(), ({ let ml_reference_value225 = proposed.borrow().clone(); ml_reference_value225 }).8)) != (self::ml_string::Text::from_units(vec![])))) {
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source1_field7584 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                    (self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 118, 97, 108, 117, 101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7584.clone())
                                }))
                            } else {
                                if (({ let ml_reference_value226 = proposed.borrow().clone(); ml_reference_value226 }).5 && ((self::read_java_script_ordinary_value_error(heap.clone(), ({ let ml_reference_value227 = proposed.borrow().clone(); ml_reference_value227 }).4)) != (self::ml_string::Text::from_units(vec![])))) {
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_field7588 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                        (self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 103, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7588.clone())
                                    }))
                                } else {
                                    if (({ let ml_reference_value228 = proposed.borrow().clone(); ml_reference_value228 }).7 && ((self::read_java_script_ordinary_value_error(heap.clone(), ({ let ml_reference_value229 = proposed.borrow().clone(); ml_reference_value229 }).6)) != (self::ml_string::Text::from_units(vec![])))) {
                                        std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source1_field7592 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                            (self::ml_string::Text::from_units(vec![112, 114, 111, 112, 111, 115, 101, 100, 32, 115, 101, 116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7592.clone())
                                        }))
                                    } else {
                                        {
                                            let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                            {
                                                let object = {
                                                    let source_ml_source1_array_read_value7595 = shared_constructor_field1.clone();
                                                    { let ml_reference_array230 = source_ml_source1_array_read_value7595.clone(); let ml_reference_position230 = index; let ml_reference_projection230 = self::ml_array::optional_at(&*ml_reference_array230.borrow(), self::ml_array::number_index(ml_reference_position230)); ml_reference_projection230 }
                                                };
                                                if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                        let source_ml_source1_field7600 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                        (self::ml_string::Text::from_units(vec![68, 101, 102, 105, 110, 101, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7600.clone())
                                                    }))
                                                } else {
                                                    match object.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                            if !(self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field7699 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                                    (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7699.clone())
                                                                }))
                                                            } else {
                                                                self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                            }
                                                        }
                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                            if !(self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field7703 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                                    (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7703.clone())
                                                                }))
                                                            } else {
                                                                self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                            }
                                                        }
                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                            self::define_java_script_array_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                        }
                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                            if !(self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field7707 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                                    (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7707.clone())
                                                                }))
                                                            } else {
                                                                self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                            }
                                                        }
                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                            if !(self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field7711 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                                    (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7711.clone())
                                                                }))
                                                            } else {
                                                                self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                            }
                                                        }
                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                            if !(self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field7715 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                                    (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7715.clone())
                                                                }))
                                                            } else {
                                                                self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                            }
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                            if !(self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field7719 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                                    (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7719.clone())
                                                                }))
                                                            } else {
                                                                self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                            }
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                            if !(self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field7723 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                                    (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7723.clone())
                                                                }))
                                                            } else {
                                                                self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                            }
                                                        }
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                            if !(self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field7727 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                                    (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7727.clone())
                                                                }))
                                                            } else {
                                                                self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                            }
                                                        }
                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                            if !(self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field7731 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                                    (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7731.clone())
                                                                }))
                                                            } else {
                                                                self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                            }
                                                        }
                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                            if !(self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field7735 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                                    (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7735.clone())
                                                                }))
                                                            } else {
                                                                self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                            }
                                                        }
                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                            if !(self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field7739 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                                    (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7739.clone())
                                                                }))
                                                            } else {
                                                                self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                            }
                                                        }
                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                            if !(self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field7743 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                                    (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7743.clone())
                                                                }))
                                                            } else {
                                                                self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
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
