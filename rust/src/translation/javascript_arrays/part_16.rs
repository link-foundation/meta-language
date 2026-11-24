// Generated from complete js/src/translation/javascript-arrays.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_math;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_unit;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_array_value(
    heap: self::JavaScriptOrdinaryHeap,
    value: self::JavaScriptSemanticValue,
) -> std::rc::Rc<std::cell::RefCell<(bool, self::ml_string::Text)>> {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_2 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                std::rc::Rc::new(std::cell::RefCell::new(false, error.clone()))
            } else {
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    {
                        let ml_library_inequality_1 =
                            invalid != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_1 {
                            std::rc::Rc::new(std::cell::RefCell::new(false, invalid.clone()))
                        } else {
                            match value.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    std::rc::Rc::new(std::cell::RefCell::new(
                                        false,
                                        self::ml_string::Text::from_units(vec![]),
                                    ))
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    std::rc::Rc::new(std::cell::RefCell::new(
                                        false,
                                        self::ml_string::Text::from_units(vec![]),
                                    ))
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    std::rc::Rc::new(std::cell::RefCell::new(
                                        false,
                                        self::ml_string::Text::from_units(vec![]),
                                    ))
                                }
                                self::JavaScriptSemanticValue::Number(_) => {
                                    std::rc::Rc::new(std::cell::RefCell::new(
                                        false,
                                        self::ml_string::Text::from_units(vec![]),
                                    ))
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    std::rc::Rc::new(std::cell::RefCell::new(
                                        false,
                                        self::ml_string::Text::from_units(vec![]),
                                    ))
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    std::rc::Rc::new(std::cell::RefCell::new(
                                        false,
                                        self::ml_string::Text::from_units(vec![]),
                                    ))
                                }
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    std::rc::Rc::new(std::cell::RefCell::new(
                                        false,
                                        self::ml_string::Text::from_units(vec![]),
                                    ))
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    std::rc::Rc::new(std::cell::RefCell::new(
                                        false,
                                        self::ml_string::Text::from_units(vec![]),
                                    ))
                                }
                                self::JavaScriptSemanticValue::Object(identity, _) => {
                                    let object = {
                                        let source_ml_source1_array_read_value16409 = match heap
                                            .clone()
                                        {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                source_ml_source1_data_property_value16407,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value16407.clone(),
                                        };
                                        {
                                            let source_ml_source1_array_read_index16410 = self::find_java_script_ordinary_object(match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value16408, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value16408.clone()
                                            }
                                        }, identity.clone());
                                            {
                                                let ml_reference_array200 =
                                                    source_ml_source1_array_read_value16409;
                                                let ml_reference_position200 =
                                                    source_ml_source1_array_read_index16410;
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array200.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position200,
                                                    ),
                                                )
                                            }
                                        }
                                    };
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_field16601 =
                                            !(match (object.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            }) && (match object
                                                .clone()
                                                .expect("a proven present value")
                                            {
                                                self::JavaScriptOrdinaryObject::ArrayObject(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => true,
                                                _ => false,
                                            } || match object
                                                .clone()
                                                .expect("a proven present value")
                                            {
                                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => true,
                                                _ => false,
                                            });
                                        (
                                            source_ml_source1_field16601,
                                            self::ml_string::Text::from_units(vec![]),
                                        )
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_weak_key_eligibility_loop2(
    mut value: self::JavaScriptSemanticValue,
    mut facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>,
    mut low: f64,
    mut high: f64,
) -> f64 {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let row = {
                        let ml_reference_array201 = facts.clone();
                        let ml_reference_position201 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array201.borrow(),
                            self::ml_array::number_index(ml_reference_position201),
                        )
                    };
                    if !(match (row.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && {
                        let source_ml_source1_comparison_operand16608 =
                            match row.clone().expect("a proven present value") {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                    source_ml_source1_data_property_value16605,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value16605.clone(),
                            };
                        {
                            let source_ml_source1_comparison_operand16609 = match value.clone() {
                                self::JavaScriptSemanticValue::Undefined => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Null => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Boolean(_) => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Number(_) => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Integer(_) => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Text(_) => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    None::<self::ml::Big>
                                }
                                self::JavaScriptSemanticValue::Symbol(
                                    source_ml_source1_data_property_value16606,
                                ) => Some(source_ml_source1_data_property_value16606.clone()),
                                self::JavaScriptSemanticValue::Object(
                                    source_ml_source1_data_property_value16607,
                                    _,
                                ) => Some(source_ml_source1_data_property_value16607.clone()),
                            };
                            if match (
                                source_ml_source1_comparison_operand16609.clone(),
                                None::<self::ml::Big>,
                            ) {
                                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                (None, None) => true,
                                _ => false,
                            } {
                                {
                                    let source_ml_source1_comparison_order16610 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,source_ml_source1_comparison_operand16608.clone());
                                    (source_ml_source1_comparison_order16610 == 1f64)
                                }
                            } else {
                                source_ml_source1_comparison_operand16608
                                    < (source_ml_source1_comparison_operand16609
                                        .clone()
                                        .expect("a proven present value"))
                            }
                        }
                    } {
                        {
                            let low_2 = middle + 1f64;
                            {
                                low = low_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            high = middle;
                            continue;
                        }
                    }
                }
            }
        } else {
            low
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop3Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop3Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop3(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result {
    loop {
        return if at
            < ({
                let ml_reference_array202 = units.clone();
                ml_reference_array202.borrow().len() as f64
            }) {
            {
                let unit = ({
                    let ml_reference_array203 = units.clone();
                    let ml_reference_position203 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array203.borrow(),
                        self::ml_array::number_index(ml_reference_position203),
                    )
                })
                .clone()
                .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
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
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result {
    MlTemporaryTestJavaScriptSemanticTextEqualLoop4Done,
    MlTemporaryTestJavaScriptSemanticTextEqualLoop4Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_text_equal_loop4(
    mut left: self::JavaScriptSemanticValue,
    mut right: self::JavaScriptSemanticValue,
    mut length: f64,
    mut at: f64,
) -> self::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result {
    loop {
        return if at < length {
            {
                let a = match left.clone() {
                    self::JavaScriptSemanticValue::Text(units) => ({
                        let ml_reference_array204 = units.clone();
                        let ml_reference_position204 = at;
                        self::ml_array::optional_at(
                            &*ml_reference_array204.borrow(),
                            self::ml_array::number_index(ml_reference_position204),
                        )
                    })
                    .clone()
                    .unwrap_or_else(|| -1f64),
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_f16617) => {
                        let text = source_ml_source1_f16617.clone();
                        {
                            let source_ml_source1_string_unit_operand16671 = text;
                            {
                                let source_ml_source1_string_unit_operand16672 = at;
                                (std::rc::Rc::new(
                                    move |text_2: self::ml_string::Text, position: f64| -> f64 {
                                        {
                                            let index = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if (index < 0f64) || (index >= (text_2.length())) {
                                                f64::NAN
                                            } else {
                                                ({
                                                let source_ml_source1_string_unit_operand16678 = {
                                                    let source_ml_source1_string_range_operand16673 = text_2.clone();
                                                    {
                                                        let source_ml_source1_string_range_operand16674 = index;
                                                        {
                                                            let source_ml_source1_string_range_operand16675 = index + 1f64;
                                                            {
                                                                let source_ml_source1_string_range_position16676 = source_ml_source1_string_range_operand16674;
                                                                {
                                                                    let source_ml_source1_string_range_position16677 = source_ml_source1_string_range_operand16675;
                                                                    source_ml_source1_string_range_operand16673.slice(source_ml_source1_string_range_position16676, source_ml_source1_string_range_position16677)
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_string_unit_operand16679 = 0f64;
                                                    source_ml_source1_string_unit_operand16678.code_point_at(source_ml_source1_string_unit_operand16679)
                                                }
                                            }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        }
                                    },
                                ))(
                                    source_ml_source1_string_unit_operand16671,
                                    source_ml_source1_string_unit_operand16672,
                                )
                            }
                        }
                    }
                    _ => -1f64,
                };
                {
                    let b = match right.clone() {
                        self::JavaScriptSemanticValue::Text(units_2) => ({
                            let ml_reference_array205 = units_2.clone();
                            let ml_reference_position205 = at;
                            self::ml_array::optional_at(
                                &*ml_reference_array205.borrow(),
                                self::ml_array::number_index(ml_reference_position205),
                            )
                        })
                        .clone()
                        .unwrap_or_else(|| -1f64),
                        self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_f16700) => {
                            let text_2 = source_ml_source1_f16700.clone();
                            {
                                let source_ml_source1_string_unit_operand16754 = text_2.clone();
                                {
                                    let source_ml_source1_string_unit_operand16755 = at;
                                    (std::rc::Rc::new(
                                        move |text_3: self::ml_string::Text,
                                              position: f64|
                                              -> f64 {
                                            {
                                                let index = if position.is_nan() {
                                                    0f64
                                                } else {
                                                    position.trunc()
                                                };
                                                if (index < 0f64) || (index >= (text_3.length())) {
                                                    f64::NAN
                                                } else {
                                                    ({
                                                    let source_ml_source1_string_unit_operand16761 = {
                                                        let source_ml_source1_string_range_operand16756 = text_3.clone();
                                                        {
                                                            let source_ml_source1_string_range_operand16757 = index;
                                                            {
                                                                let source_ml_source1_string_range_operand16758 = index + 1f64;
                                                                {
                                                                    let source_ml_source1_string_range_position16759 = source_ml_source1_string_range_operand16757;
                                                                    {
                                                                        let source_ml_source1_string_range_position16760 = source_ml_source1_string_range_operand16758;
                                                                        source_ml_source1_string_range_operand16756.slice(source_ml_source1_string_range_position16759, source_ml_source1_string_range_position16760)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_string_unit_operand16762 = 0f64;
                                                        source_ml_source1_string_unit_operand16761.code_point_at(source_ml_source1_string_unit_operand16762)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            }
                                        },
                                    ))(
                                        source_ml_source1_string_unit_operand16754,
                                        source_ml_source1_string_unit_operand16755,
                                    )
                                }
                            }
                        }
                        _ => -1f64,
                    };
                    {
                        let ml_library_inequality_1 = a != b;
                        if ml_library_inequality_1 {
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Return(false)
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
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result {
    MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_construct_java_script_semantic_text_units_loop7(
    mut value: self::JavaScriptSemanticValue,
    mut length: f64,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result {
    loop {
        if at < length {
            {
                let source_ml_temporary_construct_java_script_semantic_text_units_ignored8 = {
                    let ml_method_owner206 = units.clone();
                    let ml_method_argument206_0 =
                        self::read_java_script_semantic_text_unit(value.clone(), at);
                    let ml_inserted = vec![ml_method_argument206_0];
                    let mut ml_method_values206 = ml_method_owner206.borrow_mut();
                    let ml_length = ml_method_values206
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values206.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        }
        return {
            self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result {
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Done,
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_property_descriptor_entry_loop9(
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut key: self::JavaScriptSemanticValue,
    mut at: f64,
) -> self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result {
    loop {
        return if at
            < ({
                let ml_reference_array207 = properties.clone();
                ml_reference_array207.borrow().len() as f64
            }) {
            {
                let property = {
                    let ml_reference_array208 = properties.clone();
                    let ml_reference_position208 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array208.borrow(),
                        self::ml_array::number_index(ml_reference_position208),
                    )
                };
                if !(match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) && self::test_java_script_semantic_same_value(
                    match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source1_data_property_value16784,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value16784 =
                                *source_ml_source1_data_property_value16784;
                            source_ml_source1_data_property_value16784.clone()
                        }
                    },
                    key.clone(),
                ) {
                    self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Return(at)
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
            self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptPropertyDefinitionJoin10 {
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Next,
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(
        std::rc::Rc<
            std::cell::RefCell<(
                std::rc::Rc<
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
                self::ml_string::Text,
                self::ml_string::Text,
                std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
            )>,
        >,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result {
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Done(f64),
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_literal_property_index_loop11(
    mut key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut value: f64,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result {
    loop {
        return if at
            < ({
                let ml_reference_array209 = key.clone();
                ml_reference_array209.borrow().len() as f64
            }) {
            {
                let digit = match {
                    let ml_reference_array210 = key.clone();
                    let ml_reference_position210 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array210.borrow(),
                        self::ml_array::number_index(ml_reference_position210),
                    )
                } {
                    Some(ml_present) => ml_present,
                    None => f64::NAN,
                } - 48f64;
                if (!(self::ml_math::is_integer(digit)) || (digit < 0f64)) || (digit > 9f64) {
                    self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Return(-1f64)
                } else {
                    {
                        let value_2 = (value * 10f64) + digit;
                        {
                            let at_2 = at + 1f64;
                            {
                                (value, at) = (value_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result {
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Done,
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(f64),
}
