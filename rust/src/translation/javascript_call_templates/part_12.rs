// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallTemplate;
use super::JavaScriptClosureAllocation;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::MlTemporaryAllocateJavaScriptCallClosureLoop173Result;
use super::MlTemporaryAllocateJavaScriptCallClosureLoop174Result;
use super::create_java_script_call_closure_object;
use super::find_java_script_call_template;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_math;
use super::ml_string;
use super::read_java_script_call_template_directory_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_semantic_text_unit;
use super::source_ml_temporary_allocate_java_script_call_closure_loop173;
use super::source_ml_temporary_allocate_java_script_call_closure_loop174;
use super::source_ml_temporary_allocate_java_script_call_closure_loop175;
use super::test_java_script_ordinary_symbol;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn allocate_java_script_call_closure(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    owner: self::ml::Big,
    ordinal: f64,
    captures: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    receiver: self::JavaScriptSemanticValue,
    new_target: Option<self::JavaScriptSemanticValue>,
) -> self::JavaScriptClosureAllocation {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_3 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_3 {
                self::JavaScriptClosureAllocation::ClosureAllocationInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                {
                    let directory = self::read_java_script_call_template_directory_error(
                        program.clone(),
                        heap.clone(),
                    );
                    {
                        let ml_library_inequality_2 =
                            directory != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_2 {
                            self::JavaScriptClosureAllocation::ClosureAllocationInvalid(
                                Box::new(heap.clone()),
                                directory.clone(),
                            )
                        } else {
                            {
                                let template = {
                                    let source_ml_source1_array_read_value17555 = (match program
                                        .clone()
                                    {
                                        self::JavaScriptCallProgram::CallProgram(
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value17554,
                                            _,
                                            _,
                                        ) => ((source_ml_source1_data_property_value17554
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<
                                                    std::rc::Rc<
                                                        std::cell::RefCell<
                                                            Vec<self::JavaScriptCallTemplate>,
                                                        >,
                                                    >,
                                                >,)
                                            }))
                                        .0)
                                            .clone(),
                                    })
                                    .clone()
                                    .unwrap_or_else(|| {
                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                            self::JavaScriptCallTemplate,
                                        >::new(
                                        )))
                                    });
                                    {
                                        let source_ml_source1_array_read_index17556 =
                                            self::find_java_script_call_template(
                                                program.clone(),
                                                owner.clone(),
                                                ordinal,
                                            );
                                        {
                                            let ml_reference_array88 =
                                                source_ml_source1_array_read_value17555;
                                            let ml_reference_position88 =
                                                source_ml_source1_array_read_index17556;
                                            self::ml_array::optional_at(
                                                &*ml_reference_array88.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position88,
                                                ),
                                            )
                                        }
                                    }
                                };
                                if match (template.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } || (({
                                    let ml_reference_array89 = captures.clone();
                                    ml_reference_array89.borrow().len() as f64
                                }) != (match template
                                    .clone()
                                    .expect("a proven present value")
                                {
                                    self::JavaScriptCallTemplate::CallTemplate(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value17558,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value17558,
                                })) {
                                    self::JavaScriptClosureAllocation::ClosureAllocationInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            99, 108, 111, 115, 117, 114, 101, 32, 97, 108, 108,
                                            111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113,
                                            117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                            116, 117, 97, 108, 32, 116, 101, 109, 112, 108, 97,
                                            116, 101, 32, 97, 110, 100, 32, 101, 120, 97, 99, 116,
                                            32, 99, 97, 112, 116, 117, 114, 101, 32, 118, 101, 99,
                                            116, 111, 114,
                                        ]),
                                    )
                                } else if (((identity <= (self::ml::Big::from_i128(0)))
                                    || (identity
                                        > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
                                    || ((self::find_java_script_ordinary_object(
                                        match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                source_ml_source1_data_property_value17560,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value17560.clone(),
                                        },
                                        identity.clone(),
                                    )) >= 0f64))
                                    || self::test_java_script_ordinary_symbol(
                                        match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                _,
                                                source_ml_source1_data_property_value17561,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value17561.clone(),
                                        },
                                        identity.clone(),
                                    )
                                {
                                    self::JavaScriptClosureAllocation::ClosureAllocationInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            99, 108, 111, 115, 117, 114, 101, 32, 97, 108, 108,
                                            111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97,
                                            105, 110, 115, 32, 97, 32, 103, 101, 110, 117, 105,
                                            110, 101, 108, 121, 32, 102, 114, 101, 115, 104, 32,
                                            101, 120, 116, 101, 114, 110, 97, 108, 32, 103, 108,
                                            111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105,
                                            116, 121,
                                        ]),
                                    )
                                } else {
                                    {
                                        let capture_count = {
                                            let ml_reference_array90 = captures.clone();
                                            ml_reference_array90.borrow().len() as f64
                                        };
                                        {
                                            let properties = 2f64;
                                            {
                                                let source_ml_temporary_values170 = match heap
                                                    .clone()
                                                {
                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                        source_ml_source1_data_property_value17562,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                    ) => source_ml_source1_data_property_value17562
                                                        .clone(),
                                                };
                                                {
                                                    let source_ml_temporary_index170 =
                                                        self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s17563 = self::source_ml_temporary_allocate_java_script_call_closure_loop173(capture_count, properties, source_ml_temporary_values170, source_ml_temporary_index170);
                                                        match source_ml_source1_s17563 {
                                                        self::MlTemporaryAllocateJavaScriptCallClosureLoop173Result::MlTemporaryAllocateJavaScriptCallClosureLoop173Done(capture_count_2, properties_2) => {
                                                            if ((({ let ml_reference_array91 = match heap.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17565, _, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value17565.clone()
                                                                }
                                                            }; ml_reference_array91.borrow().len() as f64 }) >= 4096f64) || (capture_count_2 > 1_048_576f64)) || (properties_2 > 16_384f64) {
                                                                self::JavaScriptClosureAllocation::ClosureAllocationResourceLimit(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 111, 98, 106, 101, 99, 116, 44, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 111, 114, 32, 99, 97, 112, 116, 117, 114, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                            } else {
                                                                {
                                                                    let source_ml_temporary_values171 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![receiver.clone()], { captures.borrow().clone() })));
                                                                    {
                                                                        let source_ml_temporary_index171 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let source_ml_source1_s17566 = self::source_ml_temporary_allocate_java_script_call_closure_loop174(heap.clone(), source_ml_temporary_values171, source_ml_temporary_index171);
                                                                            match source_ml_source1_s17566 {
                                                                                self::MlTemporaryAllocateJavaScriptCallClosureLoop174Result::MlTemporaryAllocateJavaScriptCallClosureLoop174Done => {
                                                                                    {
                                                                                        let closure = self::create_java_script_call_closure_object(template.clone().expect("a proven present value"), identity.clone(), owner.clone(), ordinal, captures.clone(), receiver.clone(), new_target.clone());
                                                                                        {
                                                                                            let at = 0f64;
                                                                                            {
                                                                                                let at_2 = self::source_ml_temporary_allocate_java_script_call_closure_loop175(heap.clone(), identity.clone(), at);
                                                                                                {
                                                                                                    let next = {
                                                                                                        let source_ml_source1_nominal_field17575 = match heap.clone() {
                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17576, _, _, _, _, _) => {
                                                                                                                source_ml_source1_data_property_value17576.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source1_nominal_field17577 = match heap.clone() {
                                                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value17578, _, _, _, _) => {
                                                                                                                    source_ml_source1_data_property_value17578.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_nominal_field17579 = match heap.clone() {
                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, source_ml_source1_data_property_value17580, _, _, _) => {
                                                                                                                        source_ml_source1_data_property_value17580.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_field17581 = match heap.clone() {
                                                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value17582, _, _) => {
                                                                                                                            source_ml_source1_data_property_value17582.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source1_nominal_field17583 = match heap.clone() {
                                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source1_data_property_value17584, _) => {
                                                                                                                                source_ml_source1_data_property_value17584.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source1_nominal_field17585 = match heap.clone() {
                                                                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source1_data_property_value17586) => {
                                                                                                                                    source_ml_source1_data_property_value17586.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source1_nominal_override17587 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                    let source_ml_source1_concat_value17571 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ (match heap.clone() {
                                                                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17568, _, _, _, _, _) => {
                                                                                                                                            source_ml_source1_data_property_value17568.clone()
                                                                                                                                        }
                                                                                                                                    }).borrow().clone() }, 0f64, at_2)));
                                                                                                                                    {
                                                                                                                                        let source_ml_source1_concat_value17572 = std::rc::Rc::new(std::cell::RefCell::new(vec![closure]));
                                                                                                                                        {
                                                                                                                                            let source_ml_source1_concat_value17573 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ (match heap.clone() {
                                                                                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17570, _, _, _, _, _) => {
                                                                                                                                                    source_ml_source1_data_property_value17570.clone()
                                                                                                                                                }
                                                                                                                                            }).borrow().clone() }, at_2, f64::INFINITY)));
                                                                                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), { source_ml_source1_concat_value17571.borrow().clone() }), { source_ml_source1_concat_value17572.borrow().clone() }), { source_ml_source1_concat_value17573.borrow().clone() })
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }));
                                                                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_nominal_override17587, source_ml_source1_nominal_field17577, source_ml_source1_nominal_field17579, source_ml_source1_nominal_field17581, source_ml_source1_nominal_field17583, source_ml_source1_nominal_field17585)
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let invalid = self::read_java_script_ordinary_heap_error(next.clone());
                                                                                                        {
let ml_library_inequality_1 = invalid != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                                                                                            self::JavaScriptClosureAllocation::ClosureAllocationInvalid(Box::new(heap.clone()), invalid.clone())
                                                                                                        } else {
                                                                                                            self::JavaScriptClosureAllocation::ClosureAllocationNormal(Box::new(next.clone()), Box::new(self::JavaScriptSemanticValue::Object(identity.clone(), true)))
                                                                                                        }
}
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlTemporaryAllocateJavaScriptCallClosureLoop174Result::MlTemporaryAllocateJavaScriptCallClosureLoop174Return(source_ml_temporary_result175) => {
                                                                                    let source_ml_temporary_result175 = *source_ml_temporary_result175;
                                                                                    source_ml_temporary_result175.clone()
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
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if at
            < ({
                let ml_reference_array98 = units.clone();
                ml_reference_array98.borrow().len() as f64
            }) {
            {
                let unit = ({
                    let ml_reference_array99 = units.clone();
                    let ml_reference_position99 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array99.borrow(),
                        self::ml_array::number_index(ml_reference_position99),
                    )
                })
                .clone()
                .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
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
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done,
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_text_equal_loop2(
    mut left: self::JavaScriptSemanticValue,
    mut right: self::JavaScriptSemanticValue,
    mut length: f64,
    mut at: f64,
) -> self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    loop {
        return if at < length {
            {
                let a = match left.clone() {
                    self::JavaScriptSemanticValue::Text(units) => ({
                        let ml_reference_array100 = units.clone();
                        let ml_reference_position100 = at;
                        self::ml_array::optional_at(
                            &*ml_reference_array100.borrow(),
                            self::ml_array::number_index(ml_reference_position100),
                        )
                    })
                    .clone()
                    .unwrap_or_else(|| -1f64),
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_f17594) => {
                        let text = source_ml_source1_f17594.clone();
                        {
                            let source_ml_source1_string_unit_operand17648 = text;
                            {
                                let source_ml_source1_string_unit_operand17649 = at;
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
                                                let source_ml_source1_string_unit_operand17655 = {
                                                    let source_ml_source1_string_range_operand17650 = text_2.clone();
                                                    {
                                                        let source_ml_source1_string_range_operand17651 = index;
                                                        {
                                                            let source_ml_source1_string_range_operand17652 = index + 1f64;
                                                            {
                                                                let source_ml_source1_string_range_position17653 = source_ml_source1_string_range_operand17651;
                                                                {
                                                                    let source_ml_source1_string_range_position17654 = source_ml_source1_string_range_operand17652;
                                                                    source_ml_source1_string_range_operand17650.slice(source_ml_source1_string_range_position17653, source_ml_source1_string_range_position17654)
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_string_unit_operand17656 = 0f64;
                                                    source_ml_source1_string_unit_operand17655.code_point_at(source_ml_source1_string_unit_operand17656)
                                                }
                                            }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        }
                                    },
                                ))(
                                    source_ml_source1_string_unit_operand17648,
                                    source_ml_source1_string_unit_operand17649,
                                )
                            }
                        }
                    }
                    _ => -1f64,
                };
                {
                    let b = match right.clone() {
                        self::JavaScriptSemanticValue::Text(units_2) => ({
                            let ml_reference_array101 = units_2.clone();
                            let ml_reference_position101 = at;
                            self::ml_array::optional_at(
                                &*ml_reference_array101.borrow(),
                                self::ml_array::number_index(ml_reference_position101),
                            )
                        })
                        .clone()
                        .unwrap_or_else(|| -1f64),
                        self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_f17677) => {
                            let text_2 = source_ml_source1_f17677.clone();
                            {
                                let source_ml_source1_string_unit_operand17731 = text_2.clone();
                                {
                                    let source_ml_source1_string_unit_operand17732 = at;
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
                                                    let source_ml_source1_string_unit_operand17738 = {
                                                        let source_ml_source1_string_range_operand17733 = text_3.clone();
                                                        {
                                                            let source_ml_source1_string_range_operand17734 = index;
                                                            {
                                                                let source_ml_source1_string_range_operand17735 = index + 1f64;
                                                                {
                                                                    let source_ml_source1_string_range_position17736 = source_ml_source1_string_range_operand17734;
                                                                    {
                                                                        let source_ml_source1_string_range_position17737 = source_ml_source1_string_range_operand17735;
                                                                        source_ml_source1_string_range_operand17733.slice(source_ml_source1_string_range_position17736, source_ml_source1_string_range_position17737)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_string_unit_operand17739 = 0f64;
                                                        source_ml_source1_string_unit_operand17738.code_point_at(source_ml_source1_string_unit_operand17739)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            }
                                        },
                                    ))(
                                        source_ml_source1_string_unit_operand17731,
                                        source_ml_source1_string_unit_operand17732,
                                    )
                                }
                            }
                        }
                        _ => -1f64,
                    };
                    {
                        let ml_library_inequality_1 = a != b;
                        if ml_library_inequality_1 {
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(false)
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
            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result {
    MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_construct_java_script_semantic_text_units_loop5(
    mut value: self::JavaScriptSemanticValue,
    mut length: f64,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result {
    loop {
        if at < length {
            {
                let source_ml_temporary_construct_java_script_semantic_text_units_ignored6 = {
                    let ml_method_owner102 = units.clone();
                    let ml_method_argument102_0 =
                        self::read_java_script_semantic_text_unit(value.clone(), at);
                    let ml_inserted = vec![ml_method_argument102_0];
                    let mut ml_method_values102 = ml_method_owner102.borrow_mut();
                    let ml_length = ml_method_values102
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values102.extend(ml_inserted);
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
            self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptCallCodeFunctionLoop8Result {
    MlTemporaryFindJavaScriptCallCodeFunctionLoop8Done,
    MlTemporaryFindJavaScriptCallCodeFunctionLoop8Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_call_code_function_loop8(
    mut program: self::JavaScriptCallProgram,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlTemporaryFindJavaScriptCallCodeFunctionLoop8Result {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let entry = {
                        let source_ml_source1_array_read_value17766 = match program.clone() {
                            self::JavaScriptCallProgram::CallProgram(
                                source_ml_source1_data_property_value17765,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17765.clone(),
                        };
                        {
                            let ml_reference_array103 = source_ml_source1_array_read_value17766;
                            let ml_reference_position103 = middle;
                            self::ml_array::optional_at(
                                &*ml_reference_array103.borrow(),
                                self::ml_array::number_index(ml_reference_position103),
                            )
                        }
                    };
                    if match (entry.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlTemporaryFindJavaScriptCallCodeFunctionLoop8Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop8Return(-1f64)
                    } else if (match entry.clone().expect("a proven present value") {
                        self::JavaScriptCallFunction::CallFunction(
                            source_ml_source1_data_property_value17769,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17769.clone(),
                    }) == identity
                    {
                        self::MlTemporaryFindJavaScriptCallCodeFunctionLoop8Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop8Return(middle)
                    } else {
                        if (match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(
                                source_ml_source1_data_property_value17771,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17771.clone(),
                        }) < identity
                        {
                            {
                                let low_2 = middle + 1f64;
                                {
                                    low = low_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                high = middle;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptCallCodeFunctionLoop8Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop8Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_weak_key_eligibility_loop19(
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
                        let ml_reference_array104 = facts.clone();
                        let ml_reference_position104 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array104.borrow(),
                            self::ml_array::number_index(ml_reference_position104),
                        )
                    };
                    if !(match (row.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && {
                        let source_ml_source1_comparison_operand17793 =
                            match row.clone().expect("a proven present value") {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                    source_ml_source1_data_property_value17790,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value17790.clone(),
                            };
                        {
                            let source_ml_source1_comparison_operand17794 = match value.clone() {
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
                                    source_ml_source1_data_property_value17791,
                                ) => Some(source_ml_source1_data_property_value17791.clone()),
                                self::JavaScriptSemanticValue::Object(
                                    source_ml_source1_data_property_value17792,
                                    _,
                                ) => Some(source_ml_source1_data_property_value17792.clone()),
                            };
                            if match (
                                source_ml_source1_comparison_operand17794.clone(),
                                None::<self::ml::Big>,
                            ) {
                                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                (None, None) => true,
                                _ => false,
                            } {
                                {
                                    let source_ml_source1_comparison_order17795 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,source_ml_source1_comparison_operand17793.clone());
                                    (source_ml_source1_comparison_order17795 == 1f64)
                                }
                            } else {
                                source_ml_source1_comparison_operand17793
                                    < (source_ml_source1_comparison_operand17794
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop20Result {
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop20Done,
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop20Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_property_descriptor_entry_loop20(
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut key: self::JavaScriptSemanticValue,
    mut at: f64,
) -> self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop20Result {
    loop {
        return if at
            < ({
                let ml_reference_array105 = properties.clone();
                ml_reference_array105.borrow().len() as f64
            }) {
            {
                let property = {
                    let ml_reference_array106 = properties.clone();
                    let ml_reference_position106 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array106.borrow(),
                        self::ml_array::number_index(ml_reference_position106),
                    )
                };
                if !(match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) && self::test_java_script_semantic_same_value(
                    match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source1_data_property_value17798,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value17798 =
                                *source_ml_source1_data_property_value17798;
                            source_ml_source1_data_property_value17798.clone()
                        }
                    },
                    key.clone(),
                ) {
                    self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop20Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop20Return(at)
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
            self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop20Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop20Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result {
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Done(f64),
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_literal_property_index_loop22(
    mut key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut value: f64,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result {
    loop {
        return if at
            < ({
                let ml_reference_array107 = key.clone();
                ml_reference_array107.borrow().len() as f64
            }) {
            {
                let digit = match {
                    let ml_reference_array108 = key.clone();
                    let ml_reference_position108 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array108.borrow(),
                        self::ml_array::number_index(ml_reference_position108),
                    )
                } {
                    Some(ml_present) => ml_present,
                    None => f64::NAN,
                } - 48f64;
                if (!(self::ml_math::is_integer(digit)) || (digit < 0f64)) || (digit > 9f64) {
                    self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Return(-1f64)
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
            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Done(value)
        };
    }
}
