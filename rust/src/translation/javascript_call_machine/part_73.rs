// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPrimitiveWrapperState;
use super::JavaScriptPrimitiveWrapperTransition;
use super::JavaScriptSemanticValue;
use super::construct_java_script_wrapper_own_properties;
use super::find_java_script_ordinary_object;
use super::insert_java_script_ordinary_object;
use super::ml;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_primitive_wrapper_state_error;
use super::read_java_script_primitive_wrapper_work_error;
use super::read_java_script_semantic_text_length;
use super::test_java_script_ordinary_symbol;
use super::test_java_script_standard_array_creation_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_primitive_wrapper(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveWrapperState,
    identity: self::ml::Big,
) -> self::JavaScriptPrimitiveWrapperTransition {
    match state.clone() {
        self::JavaScriptPrimitiveWrapperState::PrimitiveWrapperState(
            shared_constructor_field9,
            shared_constructor_field10,
        ) => {
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field7,
                    shared_constructor_field8,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let invalid = self::read_java_script_primitive_wrapper_state_error(
                        heap.clone(),
                        state.clone(),
                    );
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperInvalid(
                            Box::new(heap.clone()),
                            invalid.clone(),
                        )
                    } else {
                        {
                            let work = self::read_java_script_primitive_wrapper_work_error(
                                heap.clone(),
                                state.clone(),
                            );
                            if (work != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), work.clone())
                            } else {
                                if ((((identity <= (self::ml::Big::from_i128(0)))
                                    || (identity
                                        > (self::ml::Big::from_i128(18446744073709551615))))
                                    || ((self::find_java_script_ordinary_object(
                                        shared_constructor_field7.clone(),
                                        identity.clone(),
                                    )) >= (0f64)))
                                    || self::test_java_script_ordinary_symbol(
                                        shared_constructor_field8.clone(),
                                        identity.clone(),
                                    ))
                                {
                                    self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 119, 114, 97, 112, 112, 101, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
                                } else {
                                    {
                                        let length = self::read_java_script_semantic_text_length(
                                            shared_constructor_field9.clone(),
                                        );
                                        {
                                            let properties =
                                                self::construct_java_script_wrapper_own_properties(
                                                    shared_constructor_field9.clone(),
                                                    length,
                                                );
                                            {
                                                let allocated =
                                                    self::insert_java_script_ordinary_object(
                                                        heap.clone(),
                                                        identity.clone(),
                                                        shared_constructor_field10.clone(),
                                                    );
                                                if (((({
                                                    let ml_reference_value810 =
                                                        allocated.borrow().clone();
                                                    ml_reference_value810
                                                })
                                                .0)
                                                    .0)
                                                    != (self::ml_string::Text::from_units(vec![])))
                                                {
                                                    self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperInvalid(Box::new(heap.clone()), (({ let ml_reference_value811 = allocated.borrow().clone(); ml_reference_value811 }).0).0)
                                                } else {
                                                    {
                                                        let input =
                                                            shared_constructor_field9.clone();
                                                        {
                                                            let prototype =
                                                                shared_constructor_field10.clone();
                                                            {
                                                                let objects = std::rc::Rc::new(
                                                                    std::cell::RefCell::new({
                                                                        let ml_values812 = {
                                                                            let ml_reference_value813 = (match (({ let ml_reference_value814 = allocated.borrow().clone(); ml_reference_value814 }).0).1 {
                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value13796, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value13796.clone()
                                                                        }
                                                                    }).borrow().clone();
                                                                            ml_reference_value813
                                                                        };
                                                                        let ml_callback812 = {
                                                                            let identity =
                                                                                identity.clone();
                                                                            let input =
                                                                                input.clone();
                                                                            let length = length;
                                                                            let prototype =
                                                                                prototype.clone();
                                                                            let properties =
                                                                                properties.clone();
                                                                            std::rc::Rc::new(move |object: self::JavaScriptOrdinaryObject| -> self::JavaScriptOrdinaryObject { if ((((match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value13810, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value13810.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value13811, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value13811.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value13812, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value13812.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value13813, _, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value13813.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value13814, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value13814.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value13815, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value13815.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value13816, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value13816.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value13817, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value13817.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value13818, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value13818.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value13819, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value13819.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value13820, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value13820.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value13821, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value13821.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value13822, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value13822.clone()
                                                                        }
                                                                    }) != identity) || match input.clone() {
                                                                        self::JavaScriptSemanticValue::Null => {
                                                                            true
                                                                        }
                                                                        _ => {
                                                                            false
                                                                        }
                                                                    }) || match input.clone() {
                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                            true
                                                                        }
                                                                        _ => {
                                                                            false
                                                                        }
                                                                    }) {
                                                                        object.clone()
                                                                    } else {
                                                                        if (length >= (0f64)) {
                                                                            {
                                                                                let source_ml_source2_constructor_field13823 = identity.clone();
                                                                                {
                                                                                    let source_ml_source2_constructor_field13824 = prototype.clone();
                                                                                    {
                                                                                        let source_ml_source2_constructor_field13825 = true;
                                                                                        {
                                                                                            let source_ml_source2_constructor_field13826 = false;
                                                                                            {
                                                                                                let source_ml_source2_constructor_field13827 = properties.clone();
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field13828 = Some(input.clone());
                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field13823.clone(), Box::new(source_ml_source2_constructor_field13824.clone()), source_ml_source2_constructor_field13825, None::<(Option<bool>,)>, None::<(Option<bool>,)>, Some((source_ml_source2_constructor_field13828.clone(),)), None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field13826, source_ml_source2_constructor_field13827.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            {
                                                                                let source_ml_source2_constructor_field13829 = identity.clone();
                                                                                {
                                                                                    let source_ml_source2_constructor_field13830 = prototype.clone();
                                                                                    {
                                                                                        let source_ml_source2_constructor_field13831 = true;
                                                                                        {
                                                                                            let source_ml_source2_constructor_field13832 = false;
                                                                                            {
                                                                                                let source_ml_source2_constructor_field13833 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field13834 = Some(input.clone());
                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field13829.clone(), Box::new(source_ml_source2_constructor_field13830.clone()), source_ml_source2_constructor_field13831, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, Some((source_ml_source2_constructor_field13834.clone(),)), source_ml_source2_constructor_field13832, source_ml_source2_constructor_field13833.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } })
                                                                        };
                                                                        ml_values812.iter().cloned().enumerate().map(|(ml_index812, ml_value812)| ml_callback812(ml_value812)).collect::<Vec<_>>()
                                                                    }),
                                                                );
                                                                {
                                                                    let next = {
                                                                        let source_ml_source2_nominal_spread13835 = (({ let ml_reference_value815 = allocated.borrow().clone(); ml_reference_value815 }).0).1;
                                                                        match source_ml_source2_nominal_spread13835.clone() {
                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                                                                    {
                                                                                        let source_ml_source2_nominal_field13836 = shared_constructor_field1.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field13838 = shared_constructor_field2.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field13840 = shared_constructor_field3.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field13842 = shared_constructor_field4.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field13844 = shared_constructor_field5.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field13846 = shared_constructor_field6.clone();
                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(objects.clone(), source_ml_source2_nominal_field13838.clone(), source_ml_source2_nominal_field13840.clone(), source_ml_source2_nominal_field13842.clone(), source_ml_source2_nominal_field13844.clone(), source_ml_source2_nominal_field13846.clone())
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                    };
                                                                    {
                                                                        let error = self::read_java_script_ordinary_heap_error(next.clone());
                                                                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperInvalid(Box::new(heap.clone()), error.clone())
                                                                            } else {
                                                                                self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperCompleted(Box::new(next.clone()), Box::new(self::JavaScriptSemanticValue::Object(identity.clone(), false)))
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
pub fn test_java_script_array_creation_operation(operation: &self::ml_string::Text) -> bool {
    self::test_java_script_standard_array_creation_operation(operation)
}
