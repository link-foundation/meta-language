// Generated from complete js/src/translation/javascript-descriptor-conversion.js. Do not edit by hand.
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
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f853) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Number(source_ml_source1_f854) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f855) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f856) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f857) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f858) => {
                {
                    let depth_2 = depth + 1f64;
                    if (identity == (match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value860, _, _, _, _) => {
                            source_ml_source1_data_property_value860.clone()
                        }
                    })) || (depth_2 > ((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value862, _) => source_ml_source1_data_property_value862.as_slice() }).len() as f64)) {
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(String::from("ordinary prototype chains must be acyclic"))
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value863, _) => {
                                    source_ml_source1_data_property_value863
                                }
                            }, identity.clone());
                            {
                                let parent = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value864, _) => source_ml_source1_data_property_value864.as_slice() }), self::ml_array::number_index(index));
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(String::from("every ordinary prototype must remain in the global directory"))
                                } else {
                                    {
                                        let prototype_2 = match parent.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value865, _, _, _) => {
                                                let source_ml_source1_data_property_value865 = *source_ml_source1_data_property_value865;
                                                source_ml_source1_data_property_value865.clone()
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
            < ((match &object {
                self::JavaScriptOrdinaryObject::OrdinaryObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value867,
                ) => source_ml_source1_data_property_value867.as_slice(),
            })
            .len() as f64)
        {
            {
                let property = self::ml_array::optional_at(
                    (match &object {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value868,
                        ) => source_ml_source1_data_property_value868.as_slice(),
                    }),
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
                        let key_error = self::read_java_script_ordinary_key_error(
                            heap.clone(),
                            match property.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    source_ml_source1_data_property_value869,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value869 =
                                        *source_ml_source1_data_property_value869;
                                    source_ml_source1_data_property_value869.clone()
                                }
                            },
                        );
                        if !(key_error).is_empty() {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(key_error.clone())
                        } else if (self::find_java_script_ordinary_property(
                            match object.clone() {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value872,
                                ) => source_ml_source1_data_property_value872,
                            },
                            match property.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    source_ml_source1_data_property_value873,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value873 =
                                        *source_ml_source1_data_property_value873;
                                    source_ml_source1_data_property_value873.clone()
                                }
                            },
                        )) != at
                        {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(String::from("ordinary property keys must be unique without reordering their creation history"))
                        } else {
                            {
                                let descriptor =
                                    match property.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                            _,
                                            source_ml_source1_data_property_value874,
                                        ) => source_ml_source1_data_property_value874.clone(),
                                    };
                                if (self::test_complete_java_script_property_descriptor(
                                    descriptor.clone(),
                                )) {
                                    {
                                        let source_ml_source1_s875 = if (descriptor.clone()).9 {
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
                                        match source_ml_source1_s875 {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next => {
                                                    {
                                                        let source_ml_source1_s876 = if (descriptor.clone()).5 {
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
                                                        match source_ml_source1_s876 {
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
                    let source_ml_source1_s878 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            _,
                            source_ml_source1_data_property_value886,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value886 =
                                *source_ml_source1_data_property_value886;
                            source_ml_source1_data_property_value886.clone()
                        }
                    };
                    match source_ml_source1_s878 {
                        self::JavaScriptSemanticValue::Undefined => true,
                        self::JavaScriptSemanticValue::Null => false,
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f879) => true,
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f880) => true,
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f881) => true,
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f882) => true,
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f883) => true,
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f884,
                            source_ml_source1_f885,
                        ) => true,
                    }
                } && {
                    let source_ml_source1_s888 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            _,
                            source_ml_source1_data_property_value896,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value896 =
                                *source_ml_source1_data_property_value896;
                            source_ml_source1_data_property_value896.clone()
                        }
                    };
                    match source_ml_source1_s888 {
                        self::JavaScriptSemanticValue::Undefined => true,
                        self::JavaScriptSemanticValue::Null => true,
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f889) => true,
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f890) => true,
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f891) => true,
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f892) => true,
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f893) => true,
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f894,
                            source_ml_source1_f895,
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
                                    source_ml_source1_data_property_value897,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value897 =
                                        *source_ml_source1_data_property_value897;
                                    source_ml_source1_data_property_value897.clone()
                                }
                            },
                        );
                        if !(prototype_error).is_empty() {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(prototype_error.clone())
                        } else {
                            {
                                let property_count_2 = property_count
                                    + ((match &object {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value898,
                                        ) => source_ml_source1_data_property_value898.as_slice(),
                                    })
                                    .len() as f64);
                                if (property_count_2 > 16_384f64)
                                    || (((match &object {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value900,
                                        ) => source_ml_source1_data_property_value900.as_slice(),
                                    })
                                    .len() as f64)
                                        > 4096f64)
                                {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(String::from("ordinary properties exceed their explicit ordered storage budget"))
                                } else {
                                    {
                                        let at = 0f64;
                                        {
                                            let source_ml_source1_s901 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop14(heap.clone(), object.clone(), at);
                                            match source_ml_source1_s901 {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Done => {
                                                    {
                                                        let prototype = match object.clone() {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value902, _, _, _) => {
                                                                let source_ml_source1_data_property_value902 = *source_ml_source1_data_property_value902;
                                                                source_ml_source1_data_property_value902.clone()
                                                            }
                                                        };
                                                        {
                                                            let depth = 0f64;
                                                            {
                                                                let source_ml_source1_s903 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop15(heap.clone(), object.clone(), prototype, depth);
                                                                match source_ml_source1_s903 {
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
                                source_ml_source1_data_property_value905,
                                _,
                            ) => source_ml_source1_data_property_value905,
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
                        source_ml_source1_data_property_value907,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value907.clone(),
                }) <= previous)
                    || ((match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            source_ml_source1_data_property_value909,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value909.clone(),
                    }) > (self::ml::Big::from_i128(18_446_744_073_709_551_615)))
                {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Return(String::from("ordinary Objects need strictly ordered unique positive unsigned64 identities"))
                } else {
                    {
                        let previous_2 = match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source1_data_property_value910,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value910.clone(),
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_locate_java_script_ordinary_property_loop18(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut key: self::JavaScriptSemanticValue,
    mut current: self::ml::Big,
    mut depth: f64,
) -> self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result {
    loop {
        return if depth
            < ((match &heap {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    source_ml_source1_data_property_value912,
                    _,
                ) => source_ml_source1_data_property_value912.as_slice(),
            })
            .len() as f64)
        {
            {
                let index = self::find_java_script_ordinary_object(
                    match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value913,
                            _,
                        ) => source_ml_source1_data_property_value913,
                    },
                    current.clone(),
                );
                {
                    let object = self::ml_array::optional_at(
                        (match &heap {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source1_data_property_value914,
                                _,
                            ) => source_ml_source1_data_property_value914.as_slice(),
                        }),
                        self::ml_array::number_index(index),
                    );
                    if match (object.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                    } else {
                        {
                            let at = self::find_java_script_ordinary_property(
                                match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value915,
                                    ) => source_ml_source1_data_property_value915,
                                },
                                key.clone(),
                            );
                            {
                                let property = self::ml_array::optional_at(
                                    &match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value916,
                                        ) => source_ml_source1_data_property_value916,
                                    },
                                    self::ml_array::number_index(at),
                                );
                                if (match (property.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) {
                                    {
                                        let prototype =
                                            match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                                    _,
                                                    source_ml_source1_data_property_value921,
                                                    _,
                                                    _,
                                                    _,
                                                ) => {
                                                    let source_ml_source1_data_property_value921 =
                                                        *source_ml_source1_data_property_value921;
                                                    source_ml_source1_data_property_value921.clone()
                                                }
                                            };
                                        match prototype {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f922) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Number(source_ml_source1_f923) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f924) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Text(source_ml_source1_f925) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f926) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Object(current_2, source_ml_source1_f927) => {
                                                {
                                                    let depth_2 = depth + 1f64;
                                                    {
                                                        (current, depth) = (current_2.clone(), depth_2);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Return({
                                        let source_ml_source1_field917 = true;
                                        {
                                            let source_ml_source1_field918 = current.clone();
                                            {
                                                let source_ml_source1_field920 = match property.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value919) => {
                                                        source_ml_source1_data_property_value919.clone()
                                                    }
                                                };
                                                (source_ml_source1_field920, source_ml_source1_field917, source_ml_source1_field918)
                                            }
                                        }
                                    })
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_define_java_script_ordinary_property_loop20(
    mut count: f64,
    mut source_ml_temporary_values19: Vec<self::JavaScriptOrdinaryObject>,
    mut source_ml_temporary_index19: self::ml::Big,
) -> f64 {
    loop {
        if source_ml_temporary_index19
            < (self::ml::Big::from_u128(source_ml_temporary_values19.len() as u128))
        {
            {
                let entry = self::ml_array::at(
                    &source_ml_temporary_values19,
                    source_ml_temporary_index19.to_index(),
                );
                {
                    let count_2 = count
                        + ((match &entry {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value928,
                            ) => source_ml_source1_data_property_value928.as_slice(),
                        })
                        .len() as f64);
                    {
                        let source_ml_temporary_index19_2 =
                            source_ml_temporary_index19.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_temporary_index19) =
                                (count_2, source_ml_temporary_index19_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { count };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result {
    MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done,
    MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Return(
        (String, self::JavaScriptOrdinaryHeap, bool),
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_set_java_script_ordinary_prototype_loop21(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut identity: self::ml::Big,
    mut next: self::JavaScriptSemanticValue,
    mut depth: f64,
) -> self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result {
    loop {
        return if depth
            <= ((match &heap {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    source_ml_source1_data_property_value930,
                    _,
                ) => source_ml_source1_data_property_value930.as_slice(),
            })
            .len() as f64)
        {
            match next.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Null => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f931) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f932) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f933) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f934) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f935) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Object(next_identity, source_ml_source1_f936) => {
                    if next_identity == identity {
                        self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Return({
                            let source_ml_source1_field937 = heap.clone();
                            {
                                let source_ml_source1_field938 = false;
                                {
                                    let source_ml_source1_field939 = String::new();
                                    (source_ml_source1_field939, source_ml_source1_field937, source_ml_source1_field938)
                                }
                            }
                        })
                    } else {
                        {
                            let at = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value940, _) => {
                                    source_ml_source1_data_property_value940
                                }
                            }, next_identity.clone());
                            {
                                let parent = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value941, _) => source_ml_source1_data_property_value941.as_slice() }), self::ml_array::number_index(at));
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Return({
                                        let source_ml_source1_field942 = heap.clone();
                                        {
                                            let source_ml_source1_field943 = false;
                                            {
                                                let source_ml_source1_field944 = String::from("every ordinary prototype must remain in the global directory");
                                                (source_ml_source1_field944, source_ml_source1_field942, source_ml_source1_field943)
                                            }
                                        }
                                    })
                                } else {
                                    {
                                        let next_2 = match parent.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value945, _, _, _) => {
                                                let source_ml_source1_data_property_value945 = *source_ml_source1_data_property_value945;
                                                source_ml_source1_data_property_value945.clone()
                                            }
                                        };
                                        {
                                            let depth_2 = depth + 1f64;
                                            {
                                                (next, depth) = (next_2, depth_2);
                                                continue;
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
            self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
        };
    }
}
