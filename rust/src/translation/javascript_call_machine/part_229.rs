// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFunction;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructorRealm;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryDependencyFunction2Loop195Result;
use super::find_java_script_array_constructor_fact;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_constructor_realm_fallback;
use super::read_java_script_ordinary_value_error;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_dependency_function2_loop195(
    text: &self::ml_string::Text,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryDependencyFunction2Loop195Result {
    loop {
        return if (index < (text.length())) {
            {
                let source_ml_source1_temporary_dependency_function2_ignored196 = {
                    let ml_method_owner2684 = units.clone();
                    let ml_method_argument2684_0 = {
                        let source_ml_source2_string_unit_operand40044 = text.clone();
                        {
                            let source_ml_source2_string_unit_operand40045 = index;
                            (std::rc::Rc::new(
                                move |text_2: self::ml_string::Text, position: f64| -> f64 {
                                    {
                                        let index_2 = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if ((index_2 < (0f64)) || (index_2 >= (text_2.length()))) {
                                            f64::NAN
                                        } else {
                                            ({
                                    let source_ml_source2_string_unit_operand40051 = {
                                        let source_ml_source2_string_range_operand40046 = text_2.clone();
                                        {
                                            let source_ml_source2_string_range_operand40047 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand40048 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position40049 = source_ml_source2_string_range_operand40047;
                                                    {
                                                        let source_ml_source2_string_range_position40050 = source_ml_source2_string_range_operand40048;
                                                        source_ml_source2_string_range_operand40046.slice(source_ml_source2_string_range_position40049, source_ml_source2_string_range_position40050)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand40052 = 0f64;
                                        source_ml_source2_string_unit_operand40051.code_point_at(source_ml_source2_string_unit_operand40052)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source2_string_unit_operand40044.clone(),
                                source_ml_source2_string_unit_operand40045,
                            )
                        }
                    };
                    let ml_inserted = vec![ml_method_argument2684_0];
                    let mut ml_method_values2684 = ml_method_owner2684.borrow_mut();
                    let ml_length = ml_method_values2684
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2684.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDependencyFunction2Loop195Result::MlSource1TemporaryDependencyFunction2Loop195Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDependencyFunction2Join194 {
    MlSource1TemporaryDependencyFunction2Join194Next(self::ml_string::Text),
    MlSource1TemporaryDependencyFunction2Join194Return(Box<self::JavaScriptCallPrimitiveResult>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptCallConstructorFactLoop197Result {
    MlSource1TemporaryFindJavaScriptCallConstructorFactLoop197Done,
    MlSource1TemporaryFindJavaScriptCallConstructorFactLoop197Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_call_constructor_fact_loop197(
    mut facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptArrayConstructorFact>>>,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlSource1TemporaryFindJavaScriptCallConstructorFactLoop197Result {
    loop {
        return if (low < high) {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let fact = {
                        let ml_reference_array2685 = facts.clone();
                        let ml_reference_position2685 = middle;
                        let ml_reference_projection2685 = self::ml_array::optional_at(
                            &*ml_reference_array2685.borrow(),
                            self::ml_array::number_index(ml_reference_position2685),
                        );
                        ml_reference_projection2685
                    };
                    if match (fact.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlSource1TemporaryFindJavaScriptCallConstructorFactLoop197Result::MlSource1TemporaryFindJavaScriptCallConstructorFactLoop197Return((-1f64))
                    } else {
                        if ((match fact.clone().expect("a proven present value") {
                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                source_ml_source2_data_property_value40056,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40056.clone(),
                        }) == identity)
                        {
                            self::MlSource1TemporaryFindJavaScriptCallConstructorFactLoop197Result::MlSource1TemporaryFindJavaScriptCallConstructorFactLoop197Return(middle)
                        } else {
                            if ((match fact.clone().expect("a proven present value") {
                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                    source_ml_source2_data_property_value40058,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value40058.clone(),
                            }) < identity)
                            {
                                {
                                    let low_2 = (middle + 1f64);
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
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptCallConstructorFactLoop197Result::MlSource1TemporaryFindJavaScriptCallConstructorFactLoop197Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop198Result {
    MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop198Done,
    MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop198Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_call_code_function_loop198(
    mut program: self::JavaScriptCallProgram,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop198Result {
    loop {
        return if (low < high) {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let entry = {
                        let source_ml_source2_array_read_value40060 = match program.clone() {
                            self::JavaScriptCallProgram::CallProgram(
                                source_ml_source2_data_property_value40059,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40059.clone(),
                        };
                        {
                            let ml_reference_array2686 =
                                source_ml_source2_array_read_value40060.clone();
                            let ml_reference_position2686 = middle;
                            let ml_reference_projection2686 = self::ml_array::optional_at(
                                &*ml_reference_array2686.borrow(),
                                self::ml_array::number_index(ml_reference_position2686),
                            );
                            ml_reference_projection2686
                        }
                    };
                    if match (entry.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop198Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop198Return((-1f64))
                    } else {
                        if ((match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(
                                source_ml_source2_data_property_value40063,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40063.clone(),
                        }) == identity)
                        {
                            self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop198Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop198Return(middle)
                        } else {
                            if ((match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(
                                    source_ml_source2_data_property_value40065,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value40065.clone(),
                            }) < identity)
                            {
                                {
                                    let low_2 = (middle + 1f64);
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
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop198Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop198Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Result {
    MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Done,
    MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop219Result {
    MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop219Done,
    MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop219Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_species_directory_error_loop219(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut root: self::JavaScriptConstructorRealm,
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop219Result {
    loop {
        return if (index < (9f64)) {
            {
                let value = self::read_java_script_constructor_realm_fallback(root.clone(), index);
                if (!(match (value.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) && ((match value.clone().expect("a proven present value") {
                    self::JavaScriptSemanticValue::Object(_, _) => false,
                    _ => true,
                } || ((match value.clone().expect("a proven present value") {
                    self::JavaScriptSemanticValue::Object(
                        _,
                        source_ml_source2_data_property_value40084,
                    ) => Some((source_ml_source2_data_property_value40084,)),
                    _ => None::<(bool,)>,
                })
                .clone()
                .expect("a proven present value"))
                .0) || ((self::read_java_script_ordinary_value_error(
                    heap.clone(),
                    value.clone().expect("a proven present value"),
                )) != (self::ml_string::Text::from_units(vec![])))))
                {
                    self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop219Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop219Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 102, 97, 108, 108, 98, 97, 99, 107, 32, 114, 111, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 114, 101, 97, 108, 109, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 79, 98, 106, 101, 99, 116, 115]))
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop219Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop219Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop218Result {
    MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop218Done,
    MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop218Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_species_directory_error_loop218(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values213: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index213: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop218Result {
    loop {
        return if (source_ml_source1_temporary_index213
            < ({
                let ml_reference_array2687 = source_ml_source1_temporary_values213.clone();
                let ml_reference_projection2687 =
                    self::ml::Big::from_u128(ml_reference_array2687.borrow().len() as u128);
                ml_reference_projection2687
            })) {
            {
                let value = {
                    let ml_reference_array2688 = source_ml_source1_temporary_values213.clone();
                    let ml_reference_position2688 = source_ml_source1_temporary_index213.clone();
                    let ml_reference_projection2688 = self::ml_array::at(
                        &*ml_reference_array2688.borrow(),
                        ml_reference_position2688.to_index(),
                    );
                    ml_reference_projection2688
                };
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone()))
                    != (self::ml_string::Text::from_units(vec![])))
                {
                    self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop218Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop218Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 97, 108, 109, 32, 114, 111, 111, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 104, 101, 97, 112]))
                } else {
                    {
                        let source_ml_source1_temporary_index213_2 =
                            source_ml_source1_temporary_index213.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index213 =
                                source_ml_source1_temporary_index213_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop218Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop218Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_species_directory_error_loop217(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut directory: self::JavaScriptArraySpeciesDirectory,
    mut previous_realm: self::ml::Big,
    mut source_ml_source1_temporary_values212: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptConstructorRealm>>,
    >,
    mut source_ml_source1_temporary_index212: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Result {
    loop {
        return match directory.clone() {
            self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(
                shared_constructor_field5,
                shared_constructor_field6,
                shared_constructor_field7,
                _,
                shared_constructor_field8,
                _,
            ) => {
                let shared_constructor_field6 = *shared_constructor_field6;
                let shared_constructor_field7 = *shared_constructor_field7;
                if (source_ml_source1_temporary_index212
                    < ({
                        let ml_reference_array2689 = source_ml_source1_temporary_values212.clone();
                        let ml_reference_projection2689 =
                            self::ml::Big::from_u128(ml_reference_array2689.borrow().len() as u128);
                        ml_reference_projection2689
                    }))
                {
                    {
                        let root = {
                            let ml_reference_array2690 =
                                source_ml_source1_temporary_values212.clone();
                            let ml_reference_position2690 =
                                source_ml_source1_temporary_index212.clone();
                            let ml_reference_projection2690 = self::ml_array::at(
                                &*ml_reference_array2690.borrow(),
                                ml_reference_position2690.to_index(),
                            );
                            ml_reference_projection2690
                        };
                        match root.clone() {
                            self::JavaScriptConstructorRealm::ConstructorRealm(
                                shared_constructor_field1,
                                shared_constructor_field2,
                                shared_constructor_field3,
                                shared_constructor_field4,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let shared_constructor_field2 = *shared_constructor_field2;
                                let shared_constructor_field3 = *shared_constructor_field3;
                                let shared_constructor_field4 = *shared_constructor_field4;
                                if (((({
                                    let source_ml_source2_type_query_operand40092 =
                                        shared_constructor_field1.clone();
                                    self::ml_string::Text::from_units(vec![
                                        98, 105, 103, 105, 110, 116,
                                    ])
                                }) != (self::ml_string::Text::from_units(vec![
                                    98, 105, 103, 105, 110, 116,
                                ]))) || (shared_constructor_field1 <= previous_realm))
                                    || (shared_constructor_field1
                                        > (self::ml::Big::from_i128(18446744073709551615))))
                                {
                                    self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 97, 108, 109, 32, 114, 111, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                } else {
                                    {
                                        let previous_realm_2 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source1_temporary_values213 =
                                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                    shared_constructor_field2.clone(),
                                                    shared_constructor_field3.clone(),
                                                    shared_constructor_field4.clone(),
                                                ]));
                                            {
                                                let source_ml_source1_temporary_index213 =
                                                    self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s40101 = self::source_ml_source1_temporary_read_java_script_array_species_directory_error_loop218(heap.clone(), source_ml_source1_temporary_values213.clone(), source_ml_source1_temporary_index213.clone());
                                                    match source_ml_source2_s40101.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop218Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop218Done => {
                                                            if ((({
                                                                let source_ml_source2_s40103 = shared_constructor_field2.clone();
                                                                match source_ml_source2_s40103.clone() {
                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                        false
                                                                    }
                                                                    _ => {
                                                                        true
                                                                    }
                                                                }
                                                            } || {
                                                                let source_ml_source2_s40108 = shared_constructor_field3.clone();
                                                                match source_ml_source2_s40108.clone() {
                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                        false
                                                                    }
                                                                    _ => {
                                                                        true
                                                                    }
                                                                }
                                                            }) || {
                                                                let source_ml_source2_s40113 = shared_constructor_field4.clone();
                                                                match source_ml_source2_s40113.clone() {
                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                        false
                                                                    }
                                                                    _ => {
                                                                        true
                                                                    }
                                                                }
                                                            }) || !(((match shared_constructor_field4.clone() {
                                                                self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value40118) => {
                                                                    Some((source_ml_source2_data_property_value40118,))
                                                                }
                                                                _ => {
                                                                    None::<(bool,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0)) {
                                                                self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 97, 108, 109, 32, 114, 111, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 97, 110, 100, 32, 65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 79, 98, 106, 101, 99, 116, 115]))
                                                            } else {
                                                                {
                                                                    let index = 0f64;
                                                                    {
                                                                        let source_ml_source2_s40119 = self::source_ml_source1_temporary_read_java_script_array_species_directory_error_loop219(heap.clone(), root.clone(), index);
                                                                        match source_ml_source2_s40119.clone() {
                                                                            self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop219Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop219Done => {
                                                                                {
                                                                                    let fact = {
                                                                                        let source_ml_source2_array_read_value40124 = shared_constructor_field8.clone();
                                                                                        {
                                                                                            let source_ml_source2_array_read_index40125 = self::find_java_script_array_constructor_fact(shared_constructor_field8.clone(), (((match shared_constructor_field4.clone() {
                                                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value40123, _) => {
                                                                                                    Some((source_ml_source2_data_property_value40123.clone(),))
                                                                                                }
                                                                                                _ => {
                                                                                                    None::<(self::ml::Big,)>
                                                                                                }
                                                                                            }).clone().expect("a proven present value")).0).clone());
                                                                                            { let ml_reference_array2691 = source_ml_source2_array_read_value40124.clone(); let ml_reference_position2691 = source_ml_source2_array_read_index40125; let ml_reference_projection2691 = self::ml_array::optional_at(&*ml_reference_array2691.borrow(), self::ml_array::number_index(ml_reference_position2691)); ml_reference_projection2691 }
                                                                                        }
                                                                                    };
                                                                                    if (((match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match fact.clone().expect("a proven present value") {
                                                                                        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value40128, _, _) => {
                                                                                            source_ml_source2_data_property_value40128.clone()
                                                                                        }
                                                                                    }) != shared_constructor_field1)) || !(match fact.clone().expect("a proven present value") {
                                                                                        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, source_ml_source2_data_property_value40130, _) => {
                                                                                            source_ml_source2_data_property_value40130
                                                                                        }
                                                                                    })) || !(match fact.clone().expect("a proven present value") {
                                                                                        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value40131, _, _, _) => {
                                                                                            source_ml_source2_data_property_value40131
                                                                                        }
                                                                                    })) {
                                                                                        self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 114, 111, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 111, 119, 110, 101, 114, 32, 102, 97, 99, 116]))
                                                                                    } else {
                                                                                        if ((shared_constructor_field1 == shared_constructor_field5) && (!(self::test_java_script_semantic_same_value(shared_constructor_field4.clone(), shared_constructor_field6.clone())) || !(self::test_java_script_semantic_same_value(shared_constructor_field3.clone(), shared_constructor_field7.clone())))) {
                                                                                            self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Return(self::ml_string::Text::from_units(vec![99, 117, 114, 114, 101, 110, 116, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 97, 108, 109, 32, 114, 111, 111, 116, 115, 32, 97, 103, 114, 101, 101, 32, 119, 105, 116, 104, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 112, 101, 99, 105, 101, 115, 32, 114, 111, 111, 116, 115]))
                                                                                        } else {
                                                                                            {
                                                                                                let source_ml_source1_temporary_index212_2 = source_ml_source1_temporary_index212.add(&self::ml::Big::from_i128(1));
                                                                                                {
                                                                                                    (previous_realm, source_ml_source1_temporary_index212) = (previous_realm_2.clone(), source_ml_source1_temporary_index212_2.clone());
                                                                                                    continue;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop219Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop219Return(source_ml_source1_temporary_result219) => {
                                                                                self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Return(source_ml_source1_temporary_result219.clone())
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop218Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop218Return(source_ml_source1_temporary_result219_2) => {
                                                            self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Return(source_ml_source1_temporary_result219_2.clone())
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
                    self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop216Result {
    MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop216Done,
    MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop216Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_species_directory_error_loop216(
    mut previous_intrinsic_realm: self::ml::Big,
    mut source_ml_source1_temporary_values211: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index211: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop216Result {
    loop {
        return if (source_ml_source1_temporary_index211
            < ({
                let ml_reference_array2692 = source_ml_source1_temporary_values211.clone();
                let ml_reference_projection2692 =
                    self::ml::Big::from_u128(ml_reference_array2692.borrow().len() as u128);
                ml_reference_projection2692
            })) {
            {
                let realm = {
                    let ml_reference_array2693 = source_ml_source1_temporary_values211.clone();
                    let ml_reference_position2693 = source_ml_source1_temporary_index211.clone();
                    let ml_reference_projection2693 = self::ml_array::at(
                        &*ml_reference_array2693.borrow(),
                        ml_reference_position2693.to_index(),
                    );
                    ml_reference_projection2693
                };
                match realm.clone() {
                    self::JavaScriptSemanticValue::Integer(value) => {
                        if (value == previous_intrinsic_realm) {
                            self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop216Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop216Return(self::ml_string::Text::from_units(vec![97, 32, 114, 101, 97, 108, 109, 32, 104, 97, 115, 32, 111, 110, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
                        } else {
                            {
                                let source_ml_source1_temporary_index211_2 =
                                    source_ml_source1_temporary_index211
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (
                                        previous_intrinsic_realm,
                                        source_ml_source1_temporary_index211,
                                    ) = (
                                        value.clone(),
                                        source_ml_source1_temporary_index211_2.clone(),
                                    );
                                    continue;
                                }
                            }
                        }
                    }
                    _ => {
                        let source_ml_source1_temporary_index211_3 =
                            source_ml_source1_temporary_index211.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index211 =
                                source_ml_source1_temporary_index211_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop216Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop216Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Result {
    MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Done,
    MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Return(self::ml_string::Text),
}
