// Generated from complete js/src/translation/javascript-collection-iterator-results.js. Do not edit by hand.
use super::JavaScriptArrayPropertyPlan;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::accepts_java_script_ordinary_own_property_definition;
use super::commit_java_script_ordinary_own_property;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_array_property_definition;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_temporary_define_java_script_array_own_property_loop120;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn define_java_script_array_own_property(
    heap: self::JavaScriptOrdinaryHeap,
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
) -> std::rc::Rc<
    std::cell::RefCell<(
        self::ml_string::Text,
        self::JavaScriptOrdinaryHeap,
        Option<(Option<bool>,)>,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
    )>,
> {
    {
        let plan = self::plan_java_script_array_property_definition(
            object.clone(),
            key.clone(),
            proposed.clone(),
        );
        if (match plan.clone() {
            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                _,
                _,
                _,
                source_ml_source1_data_property_value7613,
                _,
            ) => source_ml_source1_data_property_value7613.clone(),
        }) == (self::ml_string::Text::from_units(vec![114, 101, 115, 111, 117, 114, 99, 101]))
        {
            std::rc::Rc::new(std::cell::RefCell::new({
                let source_ml_source1_field7617 = match plan.clone() {
                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                        _,
                        _,
                        source_ml_source1_data_property_value7616,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value7616.clone(),
                };
                {
                    let source_ml_source1_field7618 = std::rc::Rc::new(std::cell::RefCell::new(
                        Vec::<self::ml_string::Text>::new(),
                    ));
                    {
                        let source_ml_source1_field7619 = Some(Some(true));
                        (
                            source_ml_source1_field7617,
                            heap.clone(),
                            source_ml_source1_field7619,
                            false,
                            source_ml_source1_field7618,
                        )
                    }
                }
            }))
        } else if (match plan.clone() {
            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                _,
                _,
                _,
                source_ml_source1_data_property_value7621,
                _,
            ) => source_ml_source1_data_property_value7621.clone(),
        }) != (self::ml_string::Text::from_units(vec![]))
        {
            std::rc::Rc::new(std::cell::RefCell::new({
                let source_ml_source1_field7625 = match plan.clone() {
                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                        _,
                        _,
                        source_ml_source1_data_property_value7624,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value7624.clone(),
                };
                {
                    let source_ml_source1_field7626 = std::rc::Rc::new(std::cell::RefCell::new(
                        Vec::<self::ml_string::Text>::new(),
                    ));
                    (
                        source_ml_source1_field7625,
                        heap.clone(),
                        None::<(Option<bool>,)>,
                        false,
                        source_ml_source1_field7626,
                    )
                }
            }))
        } else {
            {
                let count = 0f64;
                {
                    let source_ml_temporary_values119 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value7627,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value7627.clone(),
                    };
                    {
                        let source_ml_temporary_index119 = self::ml::Big::from_i128(0);
                        {
                            let count_2 = self::source_ml_temporary_define_java_script_array_own_property_loop120(count, source_ml_temporary_values119, source_ml_temporary_index119);
                            if ((count_2 - {
                                let ml_reference_array180 = match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7655,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value7655.clone(),
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7656,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value7656.clone(),
                                    self::JavaScriptOrdinaryObject::ArrayObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7657,
                                    ) => source_ml_source1_data_property_value7657.clone(),
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7658,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value7658.clone(),
                                    self::JavaScriptOrdinaryObject::MapObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7659,
                                        _,
                                    ) => source_ml_source1_data_property_value7659.clone(),
                                    self::JavaScriptOrdinaryObject::SetObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7660,
                                        _,
                                    ) => source_ml_source1_data_property_value7660.clone(),
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7661,
                                        _,
                                    ) => source_ml_source1_data_property_value7661.clone(),
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7662,
                                        _,
                                    ) => source_ml_source1_data_property_value7662.clone(),
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7663,
                                    ) => source_ml_source1_data_property_value7663.clone(),
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7664,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value7664.clone(),
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7665,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value7665.clone(),
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7666,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value7666.clone(),
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7667,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value7667.clone(),
                                };
                                ml_reference_array180.borrow().len() as f64
                            }) + {
                                let ml_reference_array181 = match match plan.clone() {
                                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                                        source_ml_source1_data_property_value7668,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source1_data_property_value7668 =
                                            *source_ml_source1_data_property_value7668;
                                        source_ml_source1_data_property_value7668.clone()
                                    }
                                } {
                                    self::JavaScriptOrdinaryObject::ClosureObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7669,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value7669.clone(),
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7670,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value7670.clone(),
                                    self::JavaScriptOrdinaryObject::ArrayObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7671,
                                    ) => source_ml_source1_data_property_value7671.clone(),
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7672,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value7672.clone(),
                                    self::JavaScriptOrdinaryObject::MapObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7673,
                                        _,
                                    ) => source_ml_source1_data_property_value7673.clone(),
                                    self::JavaScriptOrdinaryObject::SetObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7674,
                                        _,
                                    ) => source_ml_source1_data_property_value7674.clone(),
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7675,
                                        _,
                                    ) => source_ml_source1_data_property_value7675.clone(),
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7676,
                                        _,
                                    ) => source_ml_source1_data_property_value7676.clone(),
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7677,
                                    ) => source_ml_source1_data_property_value7677.clone(),
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7678,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value7678.clone(),
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7679,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value7679.clone(),
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7680,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value7680.clone(),
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value7681,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value7681.clone(),
                                };
                                ml_reference_array181.borrow().len() as f64
                            }) > 16_384f64
                            {
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source1_field7685 =
                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                            self::ml_string::Text,
                                        >::new(
                                        )));
                                    {
                                        let source_ml_source1_field7686 = Some(Some(true));
                                        (
                                            self::ml_string::Text::from_units(vec![
                                                111, 114, 100, 105, 110, 97, 114, 121, 32, 112,
                                                114, 111, 112, 101, 114, 116, 105, 101, 115, 32,
                                                101, 120, 99, 101, 101, 100, 32, 116, 104, 101,
                                                105, 114, 32, 101, 120, 112, 108, 105, 99, 105,
                                                116, 32, 111, 114, 100, 101, 114, 101, 100, 32,
                                                115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100,
                                                103, 101, 116,
                                            ]),
                                            heap.clone(),
                                            source_ml_source1_field7686,
                                            false,
                                            source_ml_source1_field7685,
                                        )
                                    }
                                }))
                            } else {
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source1_field7688 = self::replace_java_script_ordinary_object(heap.clone(), match plan.clone() {
                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value7687, _, _, _, _) => {
                                                let source_ml_source1_data_property_value7687 = *source_ml_source1_data_property_value7687;
                                                source_ml_source1_data_property_value7687.clone()
                                            }
                                        });
                                    {
                                        let source_ml_source1_field7690 = match plan.clone() {
                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, source_ml_source1_data_property_value7689, _, _, _) => {
                                                    source_ml_source1_data_property_value7689
                                                }
                                            };
                                        {
                                            let source_ml_source1_field7692 = match plan.clone() {
                                                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, source_ml_source1_data_property_value7691, _, _) => {
                                                        source_ml_source1_data_property_value7691.clone()
                                                    }
                                                };
                                            {
                                                let source_ml_source1_field7694 = match plan.clone() {
                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value7693) => {
                                                            source_ml_source1_data_property_value7693.clone()
                                                        }
                                                    };
                                                (
                                                    source_ml_source1_field7692,
                                                    source_ml_source1_field7688,
                                                    None::<(Option<bool>,)>,
                                                    source_ml_source1_field7690,
                                                    source_ml_source1_field7694,
                                                )
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn define_java_script_ordinary_property(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
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
) -> std::rc::Rc<
    std::cell::RefCell<(
        self::ml_string::Text,
        self::JavaScriptOrdinaryHeap,
        Option<(Option<bool>,)>,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
    )>,
> {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
            {
                let error = {
                    let ml_library_inequality_2 =
                        heap_error != (self::ml_string::Text::from_units(vec![]));
                    if ml_library_inequality_2 {
                        heap_error.clone()
                    } else {
                        key_error
                    }
                };
                {
                    let ml_library_inequality_1 =
                        error != (self::ml_string::Text::from_units(vec![]));
                    if ml_library_inequality_1 {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source1_field7698 =
                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                    self::ml_string::Text,
                                >::new(
                                )));
                            (
                                error.clone(),
                                heap.clone(),
                                None::<(Option<bool>,)>,
                                false,
                                source_ml_source1_field7698,
                            )
                        }))
                    } else {
                        let ml_product_condition_3 = { proposed.borrow().clone() };
                        if ml_product_condition_3 {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source1_field7702 =
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                        self::ml_string::Text,
                                    >::new(
                                    )));
                                (
                                    self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 112, 111, 115, 101, 100, 32, 112, 114, 111,
                                        112, 101, 114, 116, 121, 32, 118, 97, 108, 117, 101, 115,
                                        32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103,
                                        32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97,
                                        108, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116,
                                        101, 120, 116,
                                    ]),
                                    heap.clone(),
                                    None::<(Option<bool>,)>,
                                    false,
                                    source_ml_source1_field7702,
                                )
                            }))
                        } else {
                            let ml_product_condition_2 = { proposed.borrow().clone() };
                            if ml_product_condition_2 {
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source1_field7706 =
                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                            self::ml_string::Text,
                                        >::new(
                                        )));
                                    (
                                        self::ml_string::Text::from_units(vec![
                                            112, 114, 111, 112, 111, 115, 101, 100, 32, 103, 101,
                                            116, 116, 101, 114, 115, 32, 109, 117, 115, 116, 32,
                                            98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116,
                                            104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108,
                                            111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116,
                                        ]),
                                        heap.clone(),
                                        None::<(Option<bool>,)>,
                                        false,
                                        source_ml_source1_field7706,
                                    )
                                }))
                            } else {
                                let ml_product_condition_1 = { proposed.borrow().clone() };
                                if ml_product_condition_1 {
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_field7710 =
                                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                                self::ml_string::Text,
                                            >::new(
                                            )));
                                        (
                                            self::ml_string::Text::from_units(vec![
                                                112, 114, 111, 112, 111, 115, 101, 100, 32, 115,
                                                101, 116, 116, 101, 114, 115, 32, 109, 117, 115,
                                                116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111,
                                                32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108,
                                                32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110,
                                                116, 101, 120, 116,
                                            ]),
                                            heap.clone(),
                                            None::<(Option<bool>,)>,
                                            false,
                                            source_ml_source1_field7710,
                                        )
                                    }))
                                } else {
                                    {
                                        let index = self::find_java_script_ordinary_object(
                                            match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                    source_ml_source1_data_property_value7711,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => source_ml_source1_data_property_value7711
                                                    .clone(),
                                            },
                                            identity.clone(),
                                        );
                                        {
                                            let object = {
                                                let source_ml_source1_array_read_value7713 = match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value7712, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value7712.clone()
                                                }
                                            };
                                                {
                                                    let ml_reference_array188 =
                                                        source_ml_source1_array_read_value7713;
                                                    let ml_reference_position188 = index;
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array188.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position188,
                                                        ),
                                                    )
                                                }
                                            };
                                            if match (object.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source1_field7718 =
                                                        std::rc::Rc::new(std::cell::RefCell::new(
                                                            Vec::<self::ml_string::Text>::new(),
                                                        ));
                                                    (
                                                        self::ml_string::Text::from_units(vec![
                                                            68, 101, 102, 105, 110, 101, 79, 119,
                                                            110, 80, 114, 111, 112, 101, 114, 116,
                                                            121, 32, 110, 101, 101, 100, 115, 32,
                                                            105, 116, 115, 32, 97, 99, 116, 117,
                                                            97, 108, 32, 79, 98, 106, 101, 99, 116,
                                                            32, 105, 100, 101, 110, 116, 105, 116,
                                                            121,
                                                        ]),
                                                        heap.clone(),
                                                        None::<(Option<bool>,)>,
                                                        false,
                                                        source_ml_source1_field7718,
                                                    )
                                                }))
                                            } else {
                                                match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                    if (self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                        self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_field7817 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                            (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7817)
                                                        }))
                                                    }
                                                }
                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                    if (self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                        self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_field7821 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                            (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7821)
                                                        }))
                                                    }
                                                }
                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                    self::define_java_script_array_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                }
                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                    if (self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                        self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_field7825 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                            (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7825)
                                                        }))
                                                    }
                                                }
                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                    if (self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                        self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_field7829 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                            (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7829)
                                                        }))
                                                    }
                                                }
                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                    if (self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                        self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_field7833 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                            (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7833)
                                                        }))
                                                    }
                                                }
                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                    if (self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                        self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_field7837 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                            (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7837)
                                                        }))
                                                    }
                                                }
                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                    if (self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                        self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_field7841 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                            (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7841)
                                                        }))
                                                    }
                                                }
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                    if (self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                        self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_field7845 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                            (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7845)
                                                        }))
                                                    }
                                                }
                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                    if (self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                        self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_field7849 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                            (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7849)
                                                        }))
                                                    }
                                                }
                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                    if (self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                        self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_field7853 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                            (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7853)
                                                        }))
                                                    }
                                                }
                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                    if (self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                        self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_field7857 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                            (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7857)
                                                        }))
                                                    }
                                                }
                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                    if (self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                        self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_field7861 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                            (self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7861)
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
