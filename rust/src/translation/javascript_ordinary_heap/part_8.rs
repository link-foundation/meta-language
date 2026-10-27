// Generated from complete js/src/translation/javascript-ordinary-heap.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::test_complete_java_script_property_descriptor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop15(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut object: self::JavaScriptOrdinaryObject,
    mut prototype: self::JavaScriptSemanticValue,
    mut depth: f64,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result {
    loop {
        return match prototype.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Null => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f804) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Number(source_ml_source1_f805) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f806) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f807) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f808) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f809) => {
                {
                    let depth_2 = depth + 1f64;
                    if (identity == (match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value811, _, _, _, _) => {
                            source_ml_source1_data_property_value811.clone()
                        }
                    })) || (depth_2 > ((match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value813, _) => {
                            source_ml_source1_data_property_value813
                        }
                    }).len() as f64)) {
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(String::from("ordinary prototype chains must be acyclic"))
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value814, _) => {
                                    source_ml_source1_data_property_value814
                                }
                            }, identity.clone());
                            {
                                let parent = self::ml_array::optional_at(&match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value815, _) => {
                                        source_ml_source1_data_property_value815
                                    }
                                }, self::ml_array::number_index(index));
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(String::from("every ordinary prototype must remain in the global directory"))
                                } else {
                                    {
                                        let prototype_2 = {
                                            let source_ml_source1_nullish_field_receiver816 = (parent.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver816.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source1_nullish_field_present817 = ((source_ml_source1_nullish_field_receiver816.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present817 {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value818, _, _, _) => {
                                                            let source_ml_source1_data_property_value818 = *source_ml_source1_data_property_value818;
                                                            source_ml_source1_data_property_value818.clone()
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'prototype')")).encode_utf16().collect(), message_present: true }).raise()
                                            }
                                        };
                                        {
                                            (prototype, depth) = (prototype_2, depth_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop14(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut object: self::JavaScriptOrdinaryObject,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result {
    loop {
        return if at
            < ((match object.clone() {
                self::JavaScriptOrdinaryObject::OrdinaryObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value820,
                ) => source_ml_source1_data_property_value820,
            })
            .len() as f64)
        {
            {
                let property = self::ml_array::optional_at(
                    &match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value821,
                        ) => source_ml_source1_data_property_value821,
                    },
                    self::ml_array::number_index(at),
                );
                if match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(String::from("ordinary property storage must be dense"))
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_key_error(heap.clone(), {
                            let source_ml_source1_nullish_field_receiver822 = (property.clone(),);
                            if match (source_ml_source1_nullish_field_receiver822.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source1_nullish_field_present823 =
                                        ((source_ml_source1_nullish_field_receiver822.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source1_nullish_field_present823 {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                            source_ml_source1_data_property_value824,
                                            _,
                                        ) => {
                                            let source_ml_source1_data_property_value824 =
                                                *source_ml_source1_data_property_value824;
                                            source_ml_source1_data_property_value824.clone()
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (String::from(
                                        "Cannot read properties of undefined (reading 'key')",
                                    ))
                                    .encode_utf16()
                                    .collect(),
                                    message_present: true,
                                })
                                .raise()
                            }
                        });
                        if !(key_error).is_empty() {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(key_error.clone())
                        } else if (self::find_java_script_ordinary_property(
                            match object.clone() {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value829,
                                ) => source_ml_source1_data_property_value829,
                            },
                            {
                                let source_ml_source1_nullish_field_receiver830 =
                                    (property.clone(),);
                                if match (source_ml_source1_nullish_field_receiver830.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present.clone();
                                        true
                                    }
                                    None => false,
                                } {
                                    {
                                        let source_ml_source1_nullish_field_present831 =
                                            ((source_ml_source1_nullish_field_receiver830.clone())
                                                .0)
                                                .clone()
                                                .expect("a proven present value");
                                        match source_ml_source1_nullish_field_present831 {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                                source_ml_source1_data_property_value832,
                                                _,
                                            ) => {
                                                let source_ml_source1_data_property_value832 =
                                                    *source_ml_source1_data_property_value832;
                                                source_ml_source1_data_property_value832.clone()
                                            }
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException {
                                        class_name: String::from("TypeError"),
                                        message_units: (String::from(
                                            "Cannot read properties of undefined (reading 'key')",
                                        ))
                                        .encode_utf16()
                                        .collect(),
                                        message_present: true,
                                    })
                                    .raise()
                                }
                            },
                        )) != at
                        {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(String::from("ordinary property keys must be unique without reordering their creation history"))
                        } else {
                            {
                                let descriptor = {
                                    let source_ml_source1_nullish_field_receiver833 =
                                        (property.clone(),);
                                    if match (source_ml_source1_nullish_field_receiver833.clone()).0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source1_nullish_field_present834 =
                                                ((source_ml_source1_nullish_field_receiver833
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source1_nullish_field_present834 {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value835) => {
                                                        source_ml_source1_data_property_value835.clone()
                                                    }
                                                }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'descriptor')")).encode_utf16().collect(), message_present: true }).raise()
                                    }
                                };
                                if (self::test_complete_java_script_property_descriptor(
                                    descriptor.clone(),
                                )) {
                                    {
                                        let source_ml_source1_s836 = if (descriptor.clone()).9 {
                                            {
                                                let error =
                                                    self::read_java_script_ordinary_value_error(
                                                        heap.clone(),
                                                        (descriptor.clone()).8,
                                                    );
                                                if !(error).is_empty() {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Return(error.clone())
                                                } else {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next
                                                }
                                            }
                                        } else {
                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next
                                        };
                                        match source_ml_source1_s836 {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next => {
                                                    {
                                                        let source_ml_source1_s837 = if (descriptor.clone()).5 {
                                                            {
                                                                let error_2 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).4);
                                                                if !(error_2).is_empty() {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Return(error_2.clone())
                                                                } else {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next
                                                        };
                                                        match source_ml_source1_s837 {
                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next => {
                                                                if (descriptor.clone()).7 {
                                                                    {
                                                                        let error_3 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).6);
                                                                        if !(error_3).is_empty() {
                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(error_3.clone())
                                                                        } else {
                                                                            {
                                                                                let at_2 = at + 1f64;
                                                                                {
                                                                                    at = at_2;
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    {
                                                                        let at_3 = at + 1f64;
                                                                        {
                                                                            at = at_3;
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Return(source_ml_temporary_result17) => {
                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(source_ml_temporary_result17.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Return(source_ml_temporary_result17_2) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(source_ml_temporary_result17_2.clone())
                                                }
                                            }
                                    }
                                } else {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(String::from("ordinary storage requires complete valid property descriptors"))
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop13(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut property_count: f64,
    mut source_ml_temporary_values10: Vec<self::JavaScriptOrdinaryObject>,
    mut source_ml_temporary_index10: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result {
    loop {
        return if source_ml_temporary_index10
            < (self::ml::Big::from_u128(source_ml_temporary_values10.len() as u128))
        {
            {
                let object = self::ml_array::at(
                    &source_ml_temporary_values10,
                    source_ml_temporary_index10.to_index(),
                );
                if {
                    let source_ml_source1_s839 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            _,
                            source_ml_source1_data_property_value847,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value847 =
                                *source_ml_source1_data_property_value847;
                            source_ml_source1_data_property_value847.clone()
                        }
                    };
                    match source_ml_source1_s839 {
                        self::JavaScriptSemanticValue::Undefined => true,
                        self::JavaScriptSemanticValue::Null => false,
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f840) => true,
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f841) => true,
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f842) => true,
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f843) => true,
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f844) => true,
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f845,
                            source_ml_source1_f846,
                        ) => true,
                    }
                } && {
                    let source_ml_source1_s849 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            _,
                            source_ml_source1_data_property_value857,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value857 =
                                *source_ml_source1_data_property_value857;
                            source_ml_source1_data_property_value857.clone()
                        }
                    };
                    match source_ml_source1_s849 {
                        self::JavaScriptSemanticValue::Undefined => true,
                        self::JavaScriptSemanticValue::Null => true,
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f850) => true,
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f851) => true,
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f852) => true,
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f853) => true,
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f854) => true,
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f855,
                            source_ml_source1_f856,
                        ) => false,
                    }
                } {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(String::from("an ordinary prototype must be null or a declared Object"))
                } else {
                    {
                        let prototype_error = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            match object.clone() {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    source_ml_source1_data_property_value858,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value858 =
                                        *source_ml_source1_data_property_value858;
                                    source_ml_source1_data_property_value858.clone()
                                }
                            },
                        );
                        if !(prototype_error).is_empty() {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(prototype_error.clone())
                        } else {
                            {
                                let property_count_2 = property_count
                                    + ((match object.clone() {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value859,
                                        ) => source_ml_source1_data_property_value859,
                                    })
                                    .len() as f64);
                                if (property_count_2 > 16_384f64)
                                    || (((match object.clone() {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value861,
                                        ) => source_ml_source1_data_property_value861,
                                    })
                                    .len() as f64)
                                        > 4096f64)
                                {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(String::from("ordinary properties exceed their explicit ordered storage budget"))
                                } else {
                                    {
                                        let at = 0f64;
                                        {
                                            let source_ml_source1_s862 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop14(heap.clone(), object.clone(), at);
                                            match source_ml_source1_s862 {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Done => {
                                                    {
                                                        let prototype = match object.clone() {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value863, _, _, _) => {
                                                                let source_ml_source1_data_property_value863 = *source_ml_source1_data_property_value863;
                                                                source_ml_source1_data_property_value863.clone()
                                                            }
                                                        };
                                                        {
                                                            let depth = 0f64;
                                                            {
                                                                let source_ml_source1_s864 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop15(heap.clone(), object.clone(), prototype, depth);
                                                                match source_ml_source1_s864 {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done => {
                                                                        {
                                                                            let source_ml_temporary_index10_2 = source_ml_temporary_index10.add(&self::ml::Big::from_i128(1));
                                                                            {
                                                                                (property_count, source_ml_temporary_index10) = (property_count_2, source_ml_temporary_index10_2);
                                                                                continue;
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(source_ml_temporary_result15) => {
                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(source_ml_temporary_result15.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(source_ml_temporary_result15_2) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(source_ml_temporary_result15_2.clone())
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
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop12(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut previous: self::ml::Big,
    mut source_ml_temporary_values9: Vec<self::ml::Big>,
    mut source_ml_temporary_index9: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result {
    loop {
        return if source_ml_temporary_index9
            < (self::ml::Big::from_u128(source_ml_temporary_values9.len() as u128))
        {
            {
                let symbol = self::ml_array::at(
                    &source_ml_temporary_values9,
                    source_ml_temporary_index9.to_index(),
                );
                if ((symbol <= previous)
                    || (symbol > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
                    || ((self::find_java_script_ordinary_object(
                        match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source1_data_property_value866,
                                _,
                            ) => source_ml_source1_data_property_value866,
                        },
                        symbol.clone(),
                    )) >= 0f64)
                {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Return(String::from("Symbols and Objects need distinct strictly ordered positive unsigned64 identities"))
                } else {
                    {
                        let previous_2 = symbol.clone();
                        {
                            let source_ml_temporary_index9_2 =
                                source_ml_temporary_index9.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_temporary_index9) =
                                    (previous_2, source_ml_temporary_index9_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Done(self::ml::Big),
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop11(
    mut previous: self::ml::Big,
    mut source_ml_temporary_values8: Vec<self::JavaScriptOrdinaryObject>,
    mut source_ml_temporary_index8: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result {
    loop {
        return if source_ml_temporary_index8
            < (self::ml::Big::from_u128(source_ml_temporary_values8.len() as u128))
        {
            {
                let object = self::ml_array::at(
                    &source_ml_temporary_values8,
                    source_ml_temporary_index8.to_index(),
                );
                if ((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        source_ml_source1_data_property_value868,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value868.clone(),
                }) <= previous)
                    || ((match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            source_ml_source1_data_property_value870,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value870.clone(),
                    }) > (self::ml::Big::from_i128(18_446_744_073_709_551_615)))
                {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Return(String::from("ordinary Objects need strictly ordered unique positive unsigned64 identities"))
                } else {
                    {
                        let previous_2 = match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source1_data_property_value871,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value871.clone(),
                        };
                        {
                            let source_ml_temporary_index8_2 =
                                source_ml_temporary_index8.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_temporary_index8) =
                                    (previous_2, source_ml_temporary_index8_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Done(previous.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result {
    MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done,
    MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Return(
        (
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
            bool,
            self::ml::Big,
        ),
    ),
}
