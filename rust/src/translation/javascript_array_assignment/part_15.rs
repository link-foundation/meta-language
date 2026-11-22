// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result;
use super::find_java_script_ordinary_object;
use super::locate_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_descriptor_kind;
use super::source_ml_source1_temporary_read_java_script_array_definition_state_error_loop137;
use super::test_java_script_array_length;
use super::test_java_script_array_length_key;
use super::test_java_script_semantic_accessor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_ordinary_get(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
) -> std::rc::Rc<
    std::cell::RefCell<(
        self::ml_string::Text,
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        self::JavaScriptSemanticValue,
        self::ml_string::Text,
        self::JavaScriptSemanticValue,
        self::JavaScriptSemanticValue,
    )>,
> {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
            {
                let receiver_error =
                    self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                {
                    let error = {
                        let ml_library_inequality_4 =
                            heap_error != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_4 {
                            heap_error.clone()
                        } else {
                            let ml_library_inequality_3 =
                                key_error != (self::ml_string::Text::from_units(vec![]));
                            if ml_library_inequality_3 {
                                key_error.clone()
                            } else {
                                let ml_library_inequality_2 =
                                    receiver_error != (self::ml_string::Text::from_units(vec![]));
                                if ml_library_inequality_2 {
                                    receiver_error.clone()
                                } else if (self::find_java_script_ordinary_object(
                                    match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                            source_ml_source2_data_property_value5806,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value5806.clone(),
                                    },
                                    identity.clone(),
                                )) < 0f64
                                {
                                    self::ml_string::Text::from_units(vec![
                                        97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 71,
                                        101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115,
                                        32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99,
                                        116, 32, 105, 100, 101, 110, 116, 105, 116, 121,
                                    ])
                                } else {
                                    self::ml_string::Text::from_units(vec![])
                                }
                            }
                        }
                    };
                    {
                        let ml_library_inequality_1 =
                            error != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_1 {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_field5809 =
                                    self::JavaScriptSemanticValue::Undefined;
                                {
                                    let source_ml_source2_field5810 =
                                        self::JavaScriptSemanticValue::Undefined;
                                    {
                                        let source_ml_source2_field5812 =
                                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                                self::JavaScriptSemanticValue,
                                            >::new(
                                            )));
                                        (
                                            self::ml_string::Text::from_units(vec![
                                                101, 114, 114, 111, 114,
                                            ]),
                                            source_ml_source2_field5812,
                                            source_ml_source2_field5810,
                                            error.clone(),
                                            receiver.clone(),
                                            source_ml_source2_field5809,
                                        )
                                    }
                                }
                            }))
                        } else {
                            {
                                let target = {
                                    let source_ml_source2_array_read_value5815 = match heap.clone()
                                    {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                            source_ml_source2_data_property_value5813,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value5813.clone(),
                                    };
                                    {
                                        let source_ml_source2_array_read_index5816 =
                                            self::find_java_script_ordinary_object(
                                                match heap.clone() {
                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                        source_ml_source2_data_property_value5814,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                    ) => source_ml_source2_data_property_value5814
                                                        .clone(),
                                                },
                                                identity.clone(),
                                            );
                                        {
                                            let ml_reference_array177 =
                                                source_ml_source2_array_read_value5815;
                                            let ml_reference_position177 =
                                                source_ml_source2_array_read_index5816;
                                            self::ml_array::optional_at(
                                                &*ml_reference_array177.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position177,
                                                ),
                                            )
                                        }
                                    }
                                };
                                if ((((((((!(match (target.clone(), None::<()>) {
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
                                }) && match target.clone().expect("a proven present value")
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
                                }) && match target.clone().expect("a proven present value") {
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
                                } {
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_field6674 =
                                            self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source2_field6679 = (match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                None::<self::JavaScriptSemanticValue>
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, source_ml_source2_data_property_value6675, _) => {
                                                let source_ml_source2_data_property_value6675 = *source_ml_source2_data_property_value6675;
                                                Some(source_ml_source2_data_property_value6675.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, source_ml_source2_data_property_value6676, _) => {
                                                let source_ml_source2_data_property_value6676 = *source_ml_source2_data_property_value6676;
                                                Some(source_ml_source2_data_property_value6676.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, source_ml_source2_data_property_value6677, _) => {
                                                let source_ml_source2_data_property_value6677 = *source_ml_source2_data_property_value6677;
                                                Some(source_ml_source2_data_property_value6677.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, source_ml_source2_data_property_value6678, _) => {
                                                let source_ml_source2_data_property_value6678 = *source_ml_source2_data_property_value6678;
                                                Some(source_ml_source2_data_property_value6678.clone())
                                            }
                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                            {
                                                let source_ml_source2_field6680 =
                                                    self::JavaScriptSemanticValue::Object(
                                                        identity.clone(),
                                                        false,
                                                    );
                                                {
                                                    let source_ml_source2_field6681 =
                                                        std::rc::Rc::new(std::cell::RefCell::new(
                                                            vec![key.clone(), receiver.clone()],
                                                        ));
                                                    (
                                                        self::ml_string::Text::from_units(vec![
                                                            99, 97, 108, 108,
                                                        ]),
                                                        source_ml_source2_field6681,
                                                        source_ml_source2_field6679,
                                                        self::ml_string::Text::from_units(vec![]),
                                                        source_ml_source2_field6680,
                                                        source_ml_source2_field6674,
                                                    )
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
                                        if (({ property.borrow().clone() }).1) {
                                            {
                                                let ml_product_condition_1 = {
                                                    (({ property.borrow().clone() }).0)
                                                        .borrow()
                                                        .clone()
                                                };
                                                if ml_product_condition_1 {
                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                        let source_ml_source2_field6690 = ({
                                                            (({ property.borrow().clone() }).0)
                                                                .borrow()
                                                                .clone()
                                                        })
                                                        .8;
                                                        {
                                                            let source_ml_source2_field6691 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field6693 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]), source_ml_source2_field6693, source_ml_source2_field6691, self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field6690)
                                                            }
                                                        }
                                                    }))
                                                } else {
                                                    {
                                                        let getter = ({
                                                            (({ property.borrow().clone() }).0)
                                                                .borrow()
                                                                .clone()
                                                        })
                                                        .4;
                                                        match getter.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_field6705 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field6706 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source2_field6708 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                    (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]), source_ml_source2_field6708, source_ml_source2_field6706, self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field6705)
                                                                }
                                                            }
                                                        }))
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_field6711 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field6714 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field6714, getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field6711)
                                                            }
                                                        }))
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_field6717 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field6720 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field6720, getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field6717)
                                                            }
                                                        }))
                                                    }
                                                    self::JavaScriptSemanticValue::Number(_) => {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_field6723 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field6726 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field6726, getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field6723)
                                                            }
                                                        }))
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(_) => {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_field6729 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field6732 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field6732, getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field6729)
                                                            }
                                                        }))
                                                    }
                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_field6735 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field6738 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field6738, getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field6735)
                                                            }
                                                        }))
                                                    }
                                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_field6741 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field6744 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field6744, getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field6741)
                                                            }
                                                        }))
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_field6747 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field6750 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field6750, getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field6747)
                                                            }
                                                        }))
                                                    }
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_field6753 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source2_field6756 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                (self::ml_string::Text::from_units(vec![99, 97, 108, 108]), source_ml_source2_field6756, getter.clone(), self::ml_string::Text::from_units(vec![]), receiver.clone(), source_ml_source2_field6753)
                                                            }
                                                        }))
                                                    }
                                                }
                                                    }
                                                }
                                            }
                                        } else {
                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source2_field6684 =
                                                    self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source2_field6685 =
                                                        self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source2_field6687 =
                                                            std::rc::Rc::new(
                                                                std::cell::RefCell::new(Vec::<
                                                                    self::JavaScriptSemanticValue,
                                                                >::new(
                                                                )),
                                                            );
                                                        (
                                                            self::ml_string::Text::from_units(
                                                                vec![118, 97, 108, 117, 101],
                                                            ),
                                                            source_ml_source2_field6687,
                                                            source_ml_source2_field6685,
                                                            self::ml_string::Text::from_units(
                                                                vec![],
                                                            ),
                                                            receiver.clone(),
                                                            source_ml_source2_field6684,
                                                        )
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_definition_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayDefinitionState,
) -> self::ml_string::Text {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_3 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_3 {
                error.clone()
            } else if (match state.clone() {
                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, _, _, _) => {
                    false
                }
            } || !(self::ml_array::includes(
                &{
                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                        self::ml_string::Text::from_units(vec![
                            102, 105, 114, 115, 116, 32, 110, 117, 109, 98, 101, 114,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            115, 101, 99, 111, 110, 100, 32, 110, 117, 109, 98, 101, 114,
                        ]),
                        self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]),
                    ])))
                    .borrow()
                    .clone()
                },
                &match state.clone() {
                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value17378,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value17378.clone(),
                },
                0f64,
                false,
                |a, b| a == b,
            ))) || !(self::test_java_script_array_length(match state.clone() {
                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value17379,
                    _,
                ) => source_ml_source2_data_property_value17379,
            })) {
                self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110,
                    32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                    117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 111, 110, 118, 101,
                    114, 115, 105, 111, 110, 32, 112, 104, 97, 115, 101,
                ])
            } else if {
                let source_ml_source2_s17381 = match state.clone() {
                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value17391,
                    ) => {
                        let source_ml_source2_data_property_value17391 =
                            *source_ml_source2_data_property_value17391;
                        source_ml_source2_data_property_value17391.clone()
                    }
                };
                match source_ml_source2_s17381 {
                    self::JavaScriptSemanticValue::Number(_) => false,
                    _ => true,
                }
            } {
                self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 109,
                    112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32,
                    105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 115, 101, 109, 97, 110, 116, 105,
                    99, 32, 78, 117, 109, 98, 101, 114,
                ])
            } else {
                {
                    let number_error = self::read_java_script_ordinary_value_error(
                        heap.clone(),
                        match state.clone() {
                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value17392,
                            ) => {
                                let source_ml_source2_data_property_value17392 =
                                    *source_ml_source2_data_property_value17392;
                                source_ml_source2_data_property_value17392.clone()
                            }
                        },
                    );
                    {
                        let ml_library_inequality_2 =
                            number_error != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_2 {
                            number_error.clone()
                        } else {
                            {
                                let key_error = self::read_java_script_ordinary_key_error(heap.clone(), match state.clone() {
                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, source_ml_source2_data_property_value17393, _, _, _, _) => {
                                        let source_ml_source2_data_property_value17393 = *source_ml_source2_data_property_value17393;
                                        source_ml_source2_data_property_value17393.clone()
                                    }
                                });
                                {
                                    let ml_library_inequality_1 =
                                        key_error != (self::ml_string::Text::from_units(vec![]));
                                    if ml_library_inequality_1 {
                                        key_error.clone()
                                    } else {
                                        {
                                            let object = {
                                                let source_ml_source2_array_read_value17397 = match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value17394, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value17394.clone()
                                                }
                                            };
                                                {
                                                    let source_ml_source2_array_read_index17398 = self::find_java_script_ordinary_object(match heap.clone() {
                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value17395, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value17395.clone()
                                                    }
                                                }, match state.clone() {
                                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(source_ml_source2_data_property_value17396, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value17396.clone()
                                                    }
                                                });
                                                    {
                                                        let ml_reference_array186 =
                                                            source_ml_source2_array_read_value17397;
                                                        let ml_reference_position186 =
                                                            source_ml_source2_array_read_index17398;
                                                        self::ml_array::optional_at(
                                                            &*ml_reference_array186.borrow(),
                                                            self::ml_array::number_index(
                                                                ml_reference_position186,
                                                            ),
                                                        )
                                                    }
                                                }
                                            };
                                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        } {
                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 114, 97, 110, 100, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114])
                                        } else if (self::read_java_script_property_descriptor_kind(match state.clone() {
                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value17495, _, _, _) => {
                                                    source_ml_source2_data_property_value17495.clone()
                                                }
                                            })) == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100])) {
                                                self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 118, 97, 108, 105, 100, 32, 112, 97, 114, 116, 105, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114])
                                            } else {
                                                {
                                                    let source_ml_source1_temporary_values136 = std::rc::Rc::new(std::cell::RefCell::new(vec![({ (match state.clone() {
                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value17496, _, _, _) => {
                                                            source_ml_source2_data_property_value17496.clone()
                                                        }
                                                    }).borrow().clone() }).8, ({ (match state.clone() {
                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value17497, _, _, _) => {
                                                            source_ml_source2_data_property_value17497.clone()
                                                        }
                                                    }).borrow().clone() }).4, ({ (match state.clone() {
                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value17498, _, _, _) => {
                                                            source_ml_source2_data_property_value17498.clone()
                                                        }
                                                    }).borrow().clone() }).6]));
                                                    {
                                                        let source_ml_source1_temporary_index136 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source2_s17499 = self::source_ml_source1_temporary_read_java_script_array_definition_state_error_loop137(heap.clone(), source_ml_source1_temporary_values136, source_ml_source1_temporary_index136);
                                                            match source_ml_source2_s17499 {
                                                                self::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Done => {
                                                                    if (({ (match state.clone() {
                                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value17500, _, _, _) => {
                                                                            source_ml_source2_data_property_value17500.clone()
                                                                        }
                                                                    }).borrow().clone() }).5 && !(self::test_java_script_semantic_accessor(({ (match state.clone() {
                                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value17501, _, _, _) => {
                                                                            source_ml_source2_data_property_value17501.clone()
                                                                        }
                                                                    }).borrow().clone() }).4))) || (({ (match state.clone() {
                                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value17502, _, _, _) => {
                                                                            source_ml_source2_data_property_value17502.clone()
                                                                        }
                                                                    }).borrow().clone() }).7 && !(self::test_java_script_semantic_accessor(({ (match state.clone() {
                                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value17503, _, _, _) => {
                                                                            source_ml_source2_data_property_value17503.clone()
                                                                        }
                                                                    }).borrow().clone() }).6))) {
                                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 99, 99, 101, 115, 115, 111, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 118, 97, 108, 117, 101, 115])
                                                                    } else if ((match state.clone() {
                                                                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, source_ml_source2_data_property_value17505, _, _) => {
                                                                                source_ml_source2_data_property_value17505.clone()
                                                                            }
                                                                        }) != (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]))) && (!(self::test_java_script_array_length_key(match state.clone() {
                                                                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, source_ml_source2_data_property_value17506, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value17506 = *source_ml_source2_data_property_value17506;
                                                                                source_ml_source2_data_property_value17506.clone()
                                                                            }
                                                                        })) || !(({ (match state.clone() {
                                                                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value17507, _, _, _) => {
                                                                                source_ml_source2_data_property_value17507.clone()
                                                                            }
                                                                        }).borrow().clone() }).9)) {
                                                                            self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 118, 97, 108, 117, 101, 32, 115, 116, 97, 114, 116, 115, 32, 116, 104, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 112, 114, 111, 116, 111, 99, 111, 108])
                                                                        } else {
                                                                            self::ml_string::Text::from_units(vec![])
                                                                        }
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Return(source_ml_source1_temporary_result137) => {
                                                                    source_ml_source1_temporary_result137.clone()
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn begin_java_script_array_property_definition(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
    descriptor: std::rc::Rc<
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
) -> self::JavaScriptArrayCompletion {
    {
        let state = self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
            identity.clone(),
            Box::new(key.clone()),
            descriptor.clone(),
            if self::test_java_script_array_length_key(key.clone())
                && ({ descriptor.borrow().clone() }).9
            {
                self::ml_string::Text::from_units(vec![
                    102, 105, 114, 115, 116, 32, 110, 117, 109, 98, 101, 114,
                ])
            } else {
                self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121])
            },
            0f64,
            Box::new(self::JavaScriptSemanticValue::Number(0f64)),
        );
        {
            let error =
                self::read_java_script_array_definition_state_error(heap.clone(), state.clone());
            if error == (self::ml_string::Text::from_units(vec![])) {
                self::JavaScriptArrayCompletion::ArrayDefinitionContinue(
                    Box::new(heap.clone()),
                    Box::new(state.clone()),
                )
            } else {
                self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), error.clone())
            }
        }
    }
}
