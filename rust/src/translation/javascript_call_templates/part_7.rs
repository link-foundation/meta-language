// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptBoundObjectErrorJoin62;
use super::MlTemporaryReadJavaScriptBoundObjectErrorLoop64Result;
use super::MlTemporaryReadJavaScriptClosureObjectErrorJoin58;
use super::MlTemporaryReadJavaScriptClosureObjectErrorLoop60Result;
use super::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin55;
use super::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop56Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::source_ml_temporary_read_java_script_bound_object_error_loop64;
use super::source_ml_temporary_read_java_script_closure_object_error_loop60;
use super::source_ml_temporary_read_java_script_opaque_object_error_loop56;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_opaque_object_error(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
) -> self::ml_string::Text {
    {
        let source_ml_source1_s2679 = if (((((((match object.clone() {
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => false,
            _ => true,
        } && match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => false,
            _ => true,
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => false,
            _ => true,
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => false,
            _ => true,
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => false,
            _ => true,
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => false,
            _ => true,
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => false,
            _ => true,
        }) && match object.clone() {
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
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => false,
            _ => true,
        } {
            if match object.clone() {
                self::JavaScriptOrdinaryObject::ClosureObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3535,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3535,
                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3536,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3536,
                self::JavaScriptOrdinaryObject::ArrayObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3537,
                    _,
                ) => source_ml_source1_data_property_value3537,
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3538,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3538,
                self::JavaScriptOrdinaryObject::MapObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3539,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3539,
                self::JavaScriptOrdinaryObject::SetObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3540,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3540,
                self::JavaScriptOrdinaryObject::WeakMapObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3541,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3541,
                self::JavaScriptOrdinaryObject::WeakSetObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3542,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3542,
                self::JavaScriptOrdinaryObject::OrdinaryObject(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3543,
                    _,
                ) => source_ml_source1_data_property_value3543,
                self::JavaScriptOrdinaryObject::HostErrorObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3544,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3544,
                self::JavaScriptOrdinaryObject::HostURLObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3545,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3545,
                self::JavaScriptOrdinaryObject::HostArrayObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3546,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3546,
                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3547,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3547,
            } || (({
                let ml_reference_array40 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3561,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value3561.clone(),
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3562,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value3562.clone(),
                    self::JavaScriptOrdinaryObject::ArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3563,
                    ) => source_ml_source1_data_property_value3563.clone(),
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3564,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value3564.clone(),
                    self::JavaScriptOrdinaryObject::MapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3565,
                        _,
                    ) => source_ml_source1_data_property_value3565.clone(),
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3566,
                        _,
                    ) => source_ml_source1_data_property_value3566.clone(),
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3567,
                        _,
                    ) => source_ml_source1_data_property_value3567.clone(),
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3568,
                        _,
                    ) => source_ml_source1_data_property_value3568.clone(),
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3569,
                    ) => source_ml_source1_data_property_value3569.clone(),
                    self::JavaScriptOrdinaryObject::HostErrorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3570,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value3570.clone(),
                    self::JavaScriptOrdinaryObject::HostURLObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3571,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value3571.clone(),
                    self::JavaScriptOrdinaryObject::HostArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3572,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value3572.clone(),
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3573,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value3573.clone(),
                };
                ml_reference_array40.borrow().len() as f64
            }) != 0f64)
            {
                self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin55::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin55Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 111, 98, 106, 101, 99, 116, 115, 32, 107, 101, 101, 112, 32, 104, 111, 115, 116, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 117, 116, 115, 105, 100, 101, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 110, 97, 112, 115, 104, 111, 116, 115]))
            } else {
                {
                    let source_ml_temporary_values54 =
                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                            match object.clone() {
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
                                ) => None::<self::JavaScriptSemanticValue>,
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => None::<self::JavaScriptSemanticValue>,
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                    None::<self::JavaScriptSemanticValue>
                                }
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
                                ) => None::<self::JavaScriptSemanticValue>,
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
                                ) => None::<self::JavaScriptSemanticValue>,
                                self::JavaScriptOrdinaryObject::HostErrorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value3574,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value3574 =
                                        *source_ml_source1_data_property_value3574;
                                    Some(source_ml_source1_data_property_value3574.clone())
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value3575,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value3575 =
                                        *source_ml_source1_data_property_value3575;
                                    Some(source_ml_source1_data_property_value3575.clone())
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value3576,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value3576 =
                                        *source_ml_source1_data_property_value3576;
                                    Some(source_ml_source1_data_property_value3576.clone())
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value3577,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value3577 =
                                        *source_ml_source1_data_property_value3577;
                                    Some(source_ml_source1_data_property_value3577.clone())
                                }
                            },
                            match object.clone() {
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
                                ) => None::<self::JavaScriptSemanticValue>,
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => None::<self::JavaScriptSemanticValue>,
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                    None::<self::JavaScriptSemanticValue>
                                }
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
                                ) => None::<self::JavaScriptSemanticValue>,
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
                                ) => None::<self::JavaScriptSemanticValue>,
                                self::JavaScriptOrdinaryObject::HostErrorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value3578,
                                ) => {
                                    let source_ml_source1_data_property_value3578 =
                                        *source_ml_source1_data_property_value3578;
                                    Some(source_ml_source1_data_property_value3578.clone())
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value3579,
                                ) => {
                                    let source_ml_source1_data_property_value3579 =
                                        *source_ml_source1_data_property_value3579;
                                    Some(source_ml_source1_data_property_value3579.clone())
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value3580,
                                ) => {
                                    let source_ml_source1_data_property_value3580 =
                                        *source_ml_source1_data_property_value3580;
                                    Some(source_ml_source1_data_property_value3580.clone())
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value3581,
                                ) => {
                                    let source_ml_source1_data_property_value3581 =
                                        *source_ml_source1_data_property_value3581;
                                    Some(source_ml_source1_data_property_value3581.clone())
                                }
                            },
                        ]));
                    {
                        let source_ml_temporary_index54 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source1_s3582 = self::source_ml_temporary_read_java_script_opaque_object_error_loop56(heap.clone(), source_ml_temporary_values54, source_ml_temporary_index54);
                            match source_ml_source1_s3582 {
                                self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop56Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop56Done => {
                                    self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin55::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin55Next
                                }
                                self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop56Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop56Return(source_ml_temporary_result56) => {
                                    self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin55::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin55Return(source_ml_temporary_result56.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin55::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin55Next
        };
        match source_ml_source1_s2679 {
            self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin55::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin55Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin55::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin55Return(source_ml_temporary_result55) => {
                source_ml_temporary_result55.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closure_object_error(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
    budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>,
) -> self::ml_string::Text {
    {
        let source_ml_source1_s3892 = match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, callable, _, object_owner, ordinal, captures, lexical_receiver, lexical_new_target) => {
                let lexical_receiver = *lexical_receiver;
                if ((((((!callable || (object_owner <= (self::ml::Big::from_i128(0)))) || (object_owner > (self::ml::Big::from_i128(18_446_744_073_709_551_615)))) || !(self::ml_math::is_safe_integer(ordinal))) || (ordinal < 0f64)) || (ordinal > 4095f64)) || {
                    let source_ml_source1_same_value_operand3983 = -0f64;
                    (((ordinal == source_ml_source1_same_value_operand3983) && ((ordinal != 0f64) || ((1f64 / ordinal) == (1f64 / source_ml_source1_same_value_operand3983)))) || ((ordinal != ordinal) && (source_ml_source1_same_value_operand3983 != source_ml_source1_same_value_operand3983)))
                }) || (({ let ml_reference_array41 = captures.clone(); ml_reference_array41.borrow().len() as f64 }) > 4096f64) {
                    self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Return(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114, 44, 32, 108, 101, 120, 105, 99, 97, 108, 32, 115, 105, 116, 101, 44, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 98, 114, 97, 110, 100, 32, 97, 110, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 97, 112, 116, 117, 114, 101, 115]))
                } else {
                    {
                        let source_ml_temporary_read_java_script_closure_object_error_written59 = { let ml_field_owner43 = budget.clone(); let ml_field_previous43 = ml_field_owner43.borrow().0; let ml_field_value43 = { let ml_reference_array43 = captures.clone(); ml_reference_array43.borrow().len() as f64 }; ml_field_owner43.borrow_mut().0 = ml_field_previous43 + ml_field_value43; () };
                        if (({ budget.borrow().clone() }).0) > 1_048_576f64 {
                            self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Return(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 104, 101, 97, 112, 32, 118, 97, 108, 117, 101, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            {
                                let owner = {
                                    let source_ml_source1_array_read_value3986 = match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value3984, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value3984.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_array_read_index3987 = self::find_java_script_ordinary_object(match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value3985, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value3985.clone()
                                            }
                                        }, object_owner.clone());
                                        { let ml_reference_array45 = source_ml_source1_array_read_value3986; let ml_reference_position45 = source_ml_source1_array_read_index3987; self::ml_array::optional_at(&*ml_reference_array45.borrow(), self::ml_array::number_index(ml_reference_position45)) }
                                    }
                                };
                                if ((match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(match owner.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source1_data_property_value3988, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value3988
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source1_data_property_value3989, _, _, _, _) => {
                                        source_ml_source1_data_property_value3989
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source1_data_property_value3990, _) => {
                                        source_ml_source1_data_property_value3990
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source1_data_property_value3991, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value3991
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source1_data_property_value3992, _, _) => {
                                        source_ml_source1_data_property_value3992
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source1_data_property_value3993, _, _) => {
                                        source_ml_source1_data_property_value3993
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source1_data_property_value3994, _, _) => {
                                        source_ml_source1_data_property_value3994
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source1_data_property_value3995, _, _) => {
                                        source_ml_source1_data_property_value3995
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source1_data_property_value3996, _) => {
                                        source_ml_source1_data_property_value3996
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source1_data_property_value3997, _, _, _) => {
                                        source_ml_source1_data_property_value3997
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source1_data_property_value3998, _, _, _) => {
                                        source_ml_source1_data_property_value3998
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source1_data_property_value3999, _, _, _) => {
                                        source_ml_source1_data_property_value3999
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source1_data_property_value4000, _, _, _) => {
                                        source_ml_source1_data_property_value4000
                                    }
                                })) || match owner.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                }) || match owner.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                } {
                                    self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Return(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 99, 111, 100, 101, 32, 111, 119, 110, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 116, 105, 99, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 44, 32, 110, 101, 118, 101, 114, 32, 97, 110, 111, 116, 104, 101, 114, 32, 114, 117, 110, 116, 105, 109, 101, 32, 99, 108, 111, 115, 117, 114, 101, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
                                } else {
                                    {
                                        let new_target = ((lexical_new_target.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone();
                                        if !(match (new_target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((match new_target.clone().expect("a proven present value") {
                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        } || !(match match new_target.clone().expect("a proven present value") {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                None::<bool>
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                None::<bool>
                                            }
                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                None::<bool>
                                            }
                                            self::JavaScriptSemanticValue::Number(_) => {
                                                None::<bool>
                                            }
                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                None::<bool>
                                            }
                                            self::JavaScriptSemanticValue::Text(_) => {
                                                None::<bool>
                                            }
                                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                None::<bool>
                                            }
                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                None::<bool>
                                            }
                                            self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value4200) => {
                                                Some(source_ml_source1_data_property_value4200)
                                            }
                                        } { Some(ml_present) => ml_present, None => false })) || ((self::read_java_script_ordinary_value_error(heap.clone(), new_target.clone().expect("a proven present value"))) != (self::ml_string::Text::from_units(vec![])))) {
                                            self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Return(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 116, 105, 111, 110, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
                                        } else {
                                            {
                                                let source_ml_temporary_values57 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![lexical_receiver.clone()], { captures.borrow().clone() })));
                                                {
                                                    let source_ml_temporary_index57 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s4201 = self::source_ml_temporary_read_java_script_closure_object_error_loop60(heap.clone(), source_ml_temporary_values57, source_ml_temporary_index57);
                                                        match source_ml_source1_s4201 {
                                                            self::MlTemporaryReadJavaScriptClosureObjectErrorLoop60Result::MlTemporaryReadJavaScriptClosureObjectErrorLoop60Done => {
                                                                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Next
                                                            }
                                                            self::MlTemporaryReadJavaScriptClosureObjectErrorLoop60Result::MlTemporaryReadJavaScriptClosureObjectErrorLoop60Return(source_ml_temporary_result60) => {
                                                                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Return(source_ml_temporary_result60.clone())
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
            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Next
            }
            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Next
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Next
            }
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Next
            }
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Next
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Next
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Next
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Next
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Next
            }
            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Next
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Next
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Next
            }
        };
        match source_ml_source1_s3892 {
            self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlTemporaryReadJavaScriptClosureObjectErrorJoin58::MlTemporaryReadJavaScriptClosureObjectErrorJoin58Return(source_ml_temporary_result58) => {
                source_ml_temporary_result58.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_bound_object_error(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
    budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>,
) -> self::ml_string::Text {
    {
        let source_ml_source1_s4306 = match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Next
            }
            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, callable, _, bound_target, bound_this, bound_arguments) => {
                let bound_target = *bound_target;
                let bound_this = *bound_this;
                if ((!callable || match bound_target.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        false
                    }
                    _ => {
                        true
                    }
                }) || !(match match bound_target.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        None::<bool>
                    }
                    self::JavaScriptSemanticValue::Null => {
                        None::<bool>
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        None::<bool>
                    }
                    self::JavaScriptSemanticValue::Number(_) => {
                        None::<bool>
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        None::<bool>
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        None::<bool>
                    }
                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                        None::<bool>
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        None::<bool>
                    }
                    self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value4407) => {
                        Some(source_ml_source1_data_property_value4407)
                    }
                } { Some(ml_present) => ml_present, None => false })) || (({ let ml_reference_array47 = bound_arguments.clone(); ml_reference_array47.borrow().len() as f64 }) > 1_048_576f64) {
                    self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 116, 97, 114, 103, 101, 116, 115, 32, 97, 110, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105, 115, 116, 115]))
                } else {
                    {
                        let source_ml_temporary_read_java_script_bound_object_error_written63 = { let ml_field_owner49 = budget.clone(); let ml_field_previous49 = ml_field_owner49.borrow().0; let ml_field_value49 = { let ml_reference_array49 = bound_arguments.clone(); ml_reference_array49.borrow().len() as f64 } + 2f64; ml_field_owner49.borrow_mut().0 = ml_field_previous49 + ml_field_value49; () };
                        if (({ budget.borrow().clone() }).0) > 1_048_576f64 {
                            self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 115, 108, 111, 116, 115, 32, 97, 110, 100, 32, 99, 108, 111, 115, 117, 114, 101, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 115, 104, 97, 114, 101, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 118, 97, 108, 117, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            {
                                let source_ml_temporary_values61 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![bound_target.clone(), bound_this.clone()], { bound_arguments.borrow().clone() })));
                                {
                                    let source_ml_temporary_index61 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source1_s4410 = self::source_ml_temporary_read_java_script_bound_object_error_loop64(heap.clone(), source_ml_temporary_values61, source_ml_temporary_index61);
                                        match source_ml_source1_s4410 {
                                            self::MlTemporaryReadJavaScriptBoundObjectErrorLoop64Result::MlTemporaryReadJavaScriptBoundObjectErrorLoop64Done => {
                                                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Next
                                            }
                                            self::MlTemporaryReadJavaScriptBoundObjectErrorLoop64Result::MlTemporaryReadJavaScriptBoundObjectErrorLoop64Return(source_ml_temporary_result64) => {
                                                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Return(source_ml_temporary_result64.clone())
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Next
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Next
            }
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Next
            }
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Next
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Next
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Next
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Next
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Next
            }
            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Next
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Next
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Next
            }
        };
        match source_ml_source1_s4306 {
            self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlTemporaryReadJavaScriptBoundObjectErrorJoin62::MlTemporaryReadJavaScriptBoundObjectErrorJoin62Return(source_ml_temporary_result62) => {
                source_ml_temporary_result62.clone()
            }
        }
    }
}
