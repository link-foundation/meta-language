// Generated from complete js/src/translation/javascript-arrays.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptBoundObjectErrorJoin51;
use super::MlTemporaryReadJavaScriptBoundObjectErrorLoop53Result;
use super::MlTemporaryReadJavaScriptClosureObjectErrorJoin47;
use super::MlTemporaryReadJavaScriptClosureObjectErrorLoop49Result;
use super::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44;
use super::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::source_ml_temporary_read_java_script_bound_object_error_loop53;
use super::source_ml_temporary_read_java_script_closure_object_error_loop49;
use super::source_ml_temporary_read_java_script_opaque_object_error_loop45;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_opaque_object_error(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
) -> self::ml_string::Text {
    {
        let source_ml_source1_s2399 = if (((((((match object.clone() {
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
                    source_ml_source1_data_property_value3255,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3255,
                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3256,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3256,
                self::JavaScriptOrdinaryObject::ArrayObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3257,
                    _,
                ) => source_ml_source1_data_property_value3257,
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3258,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3258,
                self::JavaScriptOrdinaryObject::MapObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3259,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3259,
                self::JavaScriptOrdinaryObject::SetObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3260,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3260,
                self::JavaScriptOrdinaryObject::WeakMapObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3261,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3261,
                self::JavaScriptOrdinaryObject::WeakSetObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3262,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3262,
                self::JavaScriptOrdinaryObject::OrdinaryObject(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3263,
                    _,
                ) => source_ml_source1_data_property_value3263,
                self::JavaScriptOrdinaryObject::HostErrorObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3264,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3264,
                self::JavaScriptOrdinaryObject::HostURLObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3265,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3265,
                self::JavaScriptOrdinaryObject::HostArrayObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3266,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3266,
                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value3267,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value3267,
            } || (({
                let ml_reference_array138 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3281,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value3281.clone(),
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3282,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value3282.clone(),
                    self::JavaScriptOrdinaryObject::ArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3283,
                    ) => source_ml_source1_data_property_value3283.clone(),
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3284,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value3284.clone(),
                    self::JavaScriptOrdinaryObject::MapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3285,
                        _,
                    ) => source_ml_source1_data_property_value3285.clone(),
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3286,
                        _,
                    ) => source_ml_source1_data_property_value3286.clone(),
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3287,
                        _,
                    ) => source_ml_source1_data_property_value3287.clone(),
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3288,
                        _,
                    ) => source_ml_source1_data_property_value3288.clone(),
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3289,
                    ) => source_ml_source1_data_property_value3289.clone(),
                    self::JavaScriptOrdinaryObject::HostErrorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3290,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value3290.clone(),
                    self::JavaScriptOrdinaryObject::HostURLObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3291,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value3291.clone(),
                    self::JavaScriptOrdinaryObject::HostArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3292,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value3292.clone(),
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value3293,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value3293.clone(),
                };
                ml_reference_array138.borrow().len() as f64
            }) != 0f64)
            {
                self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 111, 98, 106, 101, 99, 116, 115, 32, 107, 101, 101, 112, 32, 104, 111, 115, 116, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 117, 116, 115, 105, 100, 101, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 110, 97, 112, 115, 104, 111, 116, 115]))
            } else {
                {
                    let source_ml_temporary_values43 =
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
                                    source_ml_source1_data_property_value3294,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value3294 =
                                        *source_ml_source1_data_property_value3294;
                                    Some(source_ml_source1_data_property_value3294.clone())
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value3295,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value3295 =
                                        *source_ml_source1_data_property_value3295;
                                    Some(source_ml_source1_data_property_value3295.clone())
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value3296,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value3296 =
                                        *source_ml_source1_data_property_value3296;
                                    Some(source_ml_source1_data_property_value3296.clone())
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value3297,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value3297 =
                                        *source_ml_source1_data_property_value3297;
                                    Some(source_ml_source1_data_property_value3297.clone())
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
                                    source_ml_source1_data_property_value3298,
                                ) => {
                                    let source_ml_source1_data_property_value3298 =
                                        *source_ml_source1_data_property_value3298;
                                    Some(source_ml_source1_data_property_value3298.clone())
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value3299,
                                ) => {
                                    let source_ml_source1_data_property_value3299 =
                                        *source_ml_source1_data_property_value3299;
                                    Some(source_ml_source1_data_property_value3299.clone())
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value3300,
                                ) => {
                                    let source_ml_source1_data_property_value3300 =
                                        *source_ml_source1_data_property_value3300;
                                    Some(source_ml_source1_data_property_value3300.clone())
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value3301,
                                ) => {
                                    let source_ml_source1_data_property_value3301 =
                                        *source_ml_source1_data_property_value3301;
                                    Some(source_ml_source1_data_property_value3301.clone())
                                }
                            },
                        ]));
                    {
                        let source_ml_temporary_index43 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source1_s3302 = self::source_ml_temporary_read_java_script_opaque_object_error_loop45(heap.clone(), source_ml_temporary_values43, source_ml_temporary_index43);
                            match source_ml_source1_s3302 {
                                self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Done => {
                                    self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44Next
                                }
                                self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Return(source_ml_temporary_result45) => {
                                    self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44Return(source_ml_temporary_result45.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44Next
        };
        match source_ml_source1_s2399 {
            self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44Return(source_ml_temporary_result44) => {
                source_ml_temporary_result44.clone()
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
        let source_ml_source1_s3612 = match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, callable, _, object_owner, ordinal, captures, lexical_receiver, lexical_new_target) => {
                let lexical_receiver = *lexical_receiver;
                if ((((((!callable || (object_owner <= (self::ml::Big::from_i128(0)))) || (object_owner > (self::ml::Big::from_i128(18_446_744_073_709_551_615)))) || !(self::ml_math::is_safe_integer(ordinal))) || (ordinal < 0f64)) || (ordinal > 4095f64)) || {
                    let source_ml_source1_same_value_operand3703 = -0f64;
                    (((ordinal == source_ml_source1_same_value_operand3703) && ((ordinal != 0f64) || ((1f64 / ordinal) == (1f64 / source_ml_source1_same_value_operand3703)))) || ((ordinal != ordinal) && (source_ml_source1_same_value_operand3703 != source_ml_source1_same_value_operand3703)))
                }) || (({ let ml_reference_array139 = captures.clone(); ml_reference_array139.borrow().len() as f64 }) > 4096f64) {
                    self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Return(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114, 44, 32, 108, 101, 120, 105, 99, 97, 108, 32, 115, 105, 116, 101, 44, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 98, 114, 97, 110, 100, 32, 97, 110, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 97, 112, 116, 117, 114, 101, 115]))
                } else {
                    {
                        let source_ml_temporary_read_java_script_closure_object_error_written48 = { let ml_field_owner141 = budget.clone(); let ml_field_previous141 = ml_field_owner141.borrow().0; let ml_field_value141 = { let ml_reference_array141 = captures.clone(); ml_reference_array141.borrow().len() as f64 }; ml_field_owner141.borrow_mut().0 = ml_field_previous141 + ml_field_value141; () };
                        if (({ budget.borrow().clone() }).0) > 1_048_576f64 {
                            self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Return(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 104, 101, 97, 112, 32, 118, 97, 108, 117, 101, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            {
                                let owner = {
                                    let source_ml_source1_array_read_value3706 = match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value3704, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value3704.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_array_read_index3707 = self::find_java_script_ordinary_object(match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value3705, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value3705.clone()
                                            }
                                        }, object_owner.clone());
                                        { let ml_reference_array143 = source_ml_source1_array_read_value3706; let ml_reference_position143 = source_ml_source1_array_read_index3707; self::ml_array::optional_at(&*ml_reference_array143.borrow(), self::ml_array::number_index(ml_reference_position143)) }
                                    }
                                };
                                if ((match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(match owner.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source1_data_property_value3708, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value3708
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source1_data_property_value3709, _, _, _, _) => {
                                        source_ml_source1_data_property_value3709
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source1_data_property_value3710, _) => {
                                        source_ml_source1_data_property_value3710
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source1_data_property_value3711, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value3711
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source1_data_property_value3712, _, _) => {
                                        source_ml_source1_data_property_value3712
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source1_data_property_value3713, _, _) => {
                                        source_ml_source1_data_property_value3713
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source1_data_property_value3714, _, _) => {
                                        source_ml_source1_data_property_value3714
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source1_data_property_value3715, _, _) => {
                                        source_ml_source1_data_property_value3715
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source1_data_property_value3716, _) => {
                                        source_ml_source1_data_property_value3716
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source1_data_property_value3717, _, _, _) => {
                                        source_ml_source1_data_property_value3717
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source1_data_property_value3718, _, _, _) => {
                                        source_ml_source1_data_property_value3718
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source1_data_property_value3719, _, _, _) => {
                                        source_ml_source1_data_property_value3719
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source1_data_property_value3720, _, _, _) => {
                                        source_ml_source1_data_property_value3720
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
                                    self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Return(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 99, 111, 100, 101, 32, 111, 119, 110, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 116, 105, 99, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 44, 32, 110, 101, 118, 101, 114, 32, 97, 110, 111, 116, 104, 101, 114, 32, 114, 117, 110, 116, 105, 109, 101, 32, 99, 108, 111, 115, 117, 114, 101, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
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
                                            self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value3920) => {
                                                Some(source_ml_source1_data_property_value3920)
                                            }
                                        } { Some(ml_present) => ml_present, None => false })) || ((self::read_java_script_ordinary_value_error(heap.clone(), new_target.clone().expect("a proven present value"))) != (self::ml_string::Text::from_units(vec![])))) {
                                            self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Return(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 116, 105, 111, 110, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
                                        } else {
                                            {
                                                let source_ml_temporary_values46 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![lexical_receiver.clone()], { captures.borrow().clone() })));
                                                {
                                                    let source_ml_temporary_index46 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s3921 = self::source_ml_temporary_read_java_script_closure_object_error_loop49(heap.clone(), source_ml_temporary_values46, source_ml_temporary_index46);
                                                        match source_ml_source1_s3921 {
                                                            self::MlTemporaryReadJavaScriptClosureObjectErrorLoop49Result::MlTemporaryReadJavaScriptClosureObjectErrorLoop49Done => {
                                                                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Next
                                                            }
                                                            self::MlTemporaryReadJavaScriptClosureObjectErrorLoop49Result::MlTemporaryReadJavaScriptClosureObjectErrorLoop49Return(source_ml_temporary_result49) => {
                                                                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Return(source_ml_temporary_result49.clone())
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
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Next
            }
            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Next
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Next
            }
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Next
            }
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Next
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Next
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Next
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Next
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Next
            }
            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Next
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Next
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Next
            }
        };
        match source_ml_source1_s3612 {
            self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlTemporaryReadJavaScriptClosureObjectErrorJoin47::MlTemporaryReadJavaScriptClosureObjectErrorJoin47Return(source_ml_temporary_result47) => {
                source_ml_temporary_result47.clone()
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
        let source_ml_source1_s4026 = match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Next
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
                    self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value4127) => {
                        Some(source_ml_source1_data_property_value4127)
                    }
                } { Some(ml_present) => ml_present, None => false })) || (({ let ml_reference_array145 = bound_arguments.clone(); ml_reference_array145.borrow().len() as f64 }) > 1_048_576f64) {
                    self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 116, 97, 114, 103, 101, 116, 115, 32, 97, 110, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105, 115, 116, 115]))
                } else {
                    {
                        let source_ml_temporary_read_java_script_bound_object_error_written52 = { let ml_field_owner147 = budget.clone(); let ml_field_previous147 = ml_field_owner147.borrow().0; let ml_field_value147 = { let ml_reference_array147 = bound_arguments.clone(); ml_reference_array147.borrow().len() as f64 } + 2f64; ml_field_owner147.borrow_mut().0 = ml_field_previous147 + ml_field_value147; () };
                        if (({ budget.borrow().clone() }).0) > 1_048_576f64 {
                            self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 115, 108, 111, 116, 115, 32, 97, 110, 100, 32, 99, 108, 111, 115, 117, 114, 101, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 115, 104, 97, 114, 101, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 118, 97, 108, 117, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            {
                                let source_ml_temporary_values50 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![bound_target.clone(), bound_this.clone()], { bound_arguments.borrow().clone() })));
                                {
                                    let source_ml_temporary_index50 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source1_s4130 = self::source_ml_temporary_read_java_script_bound_object_error_loop53(heap.clone(), source_ml_temporary_values50, source_ml_temporary_index50);
                                        match source_ml_source1_s4130 {
                                            self::MlTemporaryReadJavaScriptBoundObjectErrorLoop53Result::MlTemporaryReadJavaScriptBoundObjectErrorLoop53Done => {
                                                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Next
                                            }
                                            self::MlTemporaryReadJavaScriptBoundObjectErrorLoop53Result::MlTemporaryReadJavaScriptBoundObjectErrorLoop53Return(source_ml_temporary_result53) => {
                                                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Return(source_ml_temporary_result53.clone())
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
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Next
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Next
            }
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Next
            }
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Next
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Next
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Next
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Next
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Next
            }
            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Next
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Next
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Next
            }
        };
        match source_ml_source1_s4026 {
            self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlTemporaryReadJavaScriptBoundObjectErrorJoin51::MlTemporaryReadJavaScriptBoundObjectErrorJoin51Return(source_ml_temporary_result51) => {
                source_ml_temporary_result51.clone()
            }
        }
    }
}
