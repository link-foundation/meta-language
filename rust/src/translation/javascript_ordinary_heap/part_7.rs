// Generated from complete js/src/translation/javascript-ordinary-heap.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_math;
use super::ml_source_exception;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_temporary_insert_java_script_ordinary_object_loop26;
use super::source_ml_temporary_register_java_script_ordinary_symbol_loop28;
use super::test_java_script_ordinary_symbol;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn insert_java_script_ordinary_object(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    prototype: self::JavaScriptSemanticValue,
) -> (String, self::JavaScriptOrdinaryHeap, bool) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if !(heap_error).is_empty() {
            {
                let source_ml_source1_field710 = heap.clone();
                {
                    let source_ml_source1_field711 = false;
                    {
                        let source_ml_source1_field712 = heap_error.clone();
                        (
                            source_ml_source1_field712,
                            source_ml_source1_field710,
                            source_ml_source1_field711,
                        )
                    }
                }
            }
        } else if ((match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                source_ml_source1_data_property_value714,
                _,
            ) => source_ml_source1_data_property_value714,
        })
        .len() as f64)
            >= 4096f64
        {
            {
                let source_ml_source1_field715 = heap.clone();
                {
                    let source_ml_source1_field716 = false;
                    {
                        let source_ml_source1_field717 = String::from(
                            "ordinary heap identity directory exceeds its explicit resource budget",
                        );
                        (
                            source_ml_source1_field717,
                            source_ml_source1_field715,
                            source_ml_source1_field716,
                        )
                    }
                }
            }
        } else if (((identity <= (self::ml::Big::from_i128(0)))
            || (identity > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
            || ((self::find_java_script_ordinary_object(
                match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source1_data_property_value719,
                        _,
                    ) => source_ml_source1_data_property_value719,
                },
                identity.clone(),
            )) >= 0f64))
            || self::test_java_script_ordinary_symbol(
                match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        _,
                        source_ml_source1_data_property_value720,
                    ) => source_ml_source1_data_property_value720,
                },
                identity.clone(),
            )
        {
            {
                let source_ml_source1_field721 = heap.clone();
                {
                    let source_ml_source1_field722 = false;
                    {
                        let source_ml_source1_field723 = String::from(
                            "a fresh ordinary Object requires its distinct externally allocated unsigned64 global identity",
                        );
                        (
                            source_ml_source1_field723,
                            source_ml_source1_field721,
                            source_ml_source1_field722,
                        )
                    }
                }
            }
        } else if match prototype.clone() {
            self::JavaScriptSemanticValue::Undefined => true,
            self::JavaScriptSemanticValue::Null => false,
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f724) => true,
            self::JavaScriptSemanticValue::Number(source_ml_source1_f725) => true,
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f726) => true,
            self::JavaScriptSemanticValue::Text(source_ml_source1_f727) => true,
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f728) => true,
            self::JavaScriptSemanticValue::Object(
                source_ml_source1_f729,
                source_ml_source1_f730,
            ) => true,
        } && match prototype.clone() {
            self::JavaScriptSemanticValue::Undefined => true,
            self::JavaScriptSemanticValue::Null => true,
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f731) => true,
            self::JavaScriptSemanticValue::Number(source_ml_source1_f732) => true,
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f733) => true,
            self::JavaScriptSemanticValue::Text(source_ml_source1_f734) => true,
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f735) => true,
            self::JavaScriptSemanticValue::Object(
                source_ml_source1_f736,
                source_ml_source1_f737,
            ) => false,
        } {
            {
                let source_ml_source1_field738 = heap.clone();
                {
                    let source_ml_source1_field739 = false;
                    {
                        let source_ml_source1_field740 =
                            String::from("an ordinary prototype must be null or a declared Object");
                        (
                            source_ml_source1_field740,
                            source_ml_source1_field738,
                            source_ml_source1_field739,
                        )
                    }
                }
            }
        } else {
            {
                let error =
                    self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone());
                if !(error).is_empty() {
                    {
                        let source_ml_source1_field741 = heap.clone();
                        {
                            let source_ml_source1_field742 = false;
                            {
                                let source_ml_source1_field743 = error.clone();
                                (
                                    source_ml_source1_field743,
                                    source_ml_source1_field741,
                                    source_ml_source1_field742,
                                )
                            }
                        }
                    }
                } else {
                    {
                        let at = 0f64;
                        {
                            let source_ml_temporary_values25 = match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                    source_ml_source1_data_property_value744,
                                    _,
                                ) => source_ml_source1_data_property_value744,
                            };
                            {
                                let source_ml_temporary_index25 = self::ml::Big::from_i128(0);
                                {
                                    let at_2 = self::source_ml_temporary_insert_java_script_ordinary_object_loop26(identity.clone(), at, source_ml_temporary_values25, source_ml_temporary_index25);
                                    {
                                        let object = self::JavaScriptOrdinaryObject::OrdinaryObject(
                                            identity.clone(),
                                            Box::new(prototype.clone()),
                                            true,
                                            false,
                                            Vec::<self::JavaScriptOrdinaryProperty>::new(),
                                        );
                                        {
                                            let objects = {
                                                let source_ml_source1_concat_value749 = self::ml_array::slice(&match heap.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value746, _) => {
                                                                    source_ml_source1_data_property_value746
                                                                }
                                                            }, 0f64, at_2);
                                                {
                                                    let source_ml_source1_concat_value750 =
                                                        vec![object];
                                                    {
                                                        let source_ml_source1_concat_value751 = self::ml_array::slice(&match heap.clone() {
                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value748, _) => {
                                                                            source_ml_source1_data_property_value748
                                                                        }
                                                                    }, at_2, f64::INFINITY);
                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), source_ml_source1_concat_value749), source_ml_source1_concat_value750), source_ml_source1_concat_value751)
                                                    }
                                                }
                                            };
                                            {
                                                let source_ml_source1_field753 = self::JavaScriptOrdinaryHeap::OrdinaryHeap(objects, match heap.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value752) => {
                                                                    source_ml_source1_data_property_value752
                                                                }
                                                            });
                                                {
                                                    let source_ml_source1_field754 = true;
                                                    {
                                                        let source_ml_source1_field755 =
                                                            String::new();
                                                        (
                                                            source_ml_source1_field755,
                                                            source_ml_source1_field753,
                                                            source_ml_source1_field754,
                                                        )
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
pub fn register_java_script_ordinary_symbol(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
) -> (String, self::JavaScriptOrdinaryHeap, bool) {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if !(error).is_empty() {
            {
                let source_ml_source1_field756 = heap.clone();
                {
                    let source_ml_source1_field757 = false;
                    {
                        let source_ml_source1_field758 = error.clone();
                        (
                            source_ml_source1_field758,
                            source_ml_source1_field756,
                            source_ml_source1_field757,
                        )
                    }
                }
            }
        } else if ((match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                _,
                source_ml_source1_data_property_value760,
            ) => source_ml_source1_data_property_value760,
        })
        .len() as f64)
            >= 16_384f64
        {
            {
                let source_ml_source1_field761 = heap.clone();
                {
                    let source_ml_source1_field762 = false;
                    {
                        let source_ml_source1_field763 = String::from(
                            "ordinary heap identity directory exceeds its explicit resource budget",
                        );
                        (
                            source_ml_source1_field763,
                            source_ml_source1_field761,
                            source_ml_source1_field762,
                        )
                    }
                }
            }
        } else if (((identity <= (self::ml::Big::from_i128(0)))
            || (identity > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
            || ((self::find_java_script_ordinary_object(
                match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source1_data_property_value765,
                        _,
                    ) => source_ml_source1_data_property_value765,
                },
                identity.clone(),
            )) >= 0f64))
            || self::test_java_script_ordinary_symbol(
                match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        _,
                        source_ml_source1_data_property_value766,
                    ) => source_ml_source1_data_property_value766,
                },
                identity.clone(),
            )
        {
            {
                let source_ml_source1_field767 = heap.clone();
                {
                    let source_ml_source1_field768 = false;
                    {
                        let source_ml_source1_field769 = String::from(
                            "a fresh Symbol requires its distinct externally allocated unsigned64 global identity",
                        );
                        (
                            source_ml_source1_field769,
                            source_ml_source1_field767,
                            source_ml_source1_field768,
                        )
                    }
                }
            }
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_temporary_values27 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            _,
                            source_ml_source1_data_property_value770,
                        ) => source_ml_source1_data_property_value770,
                    };
                    {
                        let source_ml_temporary_index27 = self::ml::Big::from_i128(0);
                        {
                            let at_2 = self::source_ml_temporary_register_java_script_ordinary_symbol_loop28(identity.clone(), at, source_ml_temporary_values27, source_ml_temporary_index27);
                            {
                                let symbols = {
                                    let source_ml_source1_concat_value775 = self::ml_array::slice(
                                        &match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                _,
                                                source_ml_source1_data_property_value772,
                                            ) => source_ml_source1_data_property_value772,
                                        },
                                        0f64,
                                        at_2,
                                    );
                                    {
                                        let source_ml_source1_concat_value776 =
                                            vec![identity.clone()];
                                        {
                                            let source_ml_source1_concat_value777 = self::ml_array::slice(&match heap.clone() {
                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value774) => {
                                                            source_ml_source1_data_property_value774
                                                        }
                                                    }, at_2, f64::INFINITY);
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<self::ml::Big>::new(),
                                                        source_ml_source1_concat_value775,
                                                    ),
                                                    source_ml_source1_concat_value776,
                                                ),
                                                source_ml_source1_concat_value777,
                                            )
                                        }
                                    }
                                };
                                {
                                    let source_ml_source1_field779 =
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                            match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                    source_ml_source1_data_property_value778,
                                                    _,
                                                ) => source_ml_source1_data_property_value778,
                                            },
                                            symbols,
                                        );
                                    {
                                        let source_ml_source1_field780 = true;
                                        {
                                            let source_ml_source1_field781 = String::new();
                                            (
                                                source_ml_source1_field781,
                                                source_ml_source1_field779,
                                                source_ml_source1_field780,
                                            )
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
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(
    units: &[f64],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if at < (units.len() as f64) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(String::from("semantic text requires complete ordered UTF16 code units"))
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
pub enum MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Done,
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_same_value_loop2(
    left_units: &[f64],
    right_units: &[f64],
    mut at: f64,
) -> self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    loop {
        return if at < (left_units.len() as f64) {
            {
                let ml_library_inequality_1 =
                    ((self::ml_array::optional_at(left_units, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| -1f64))
                        != ((self::ml_array::optional_at(
                            right_units,
                            self::ml_array::number_index(at),
                        ))
                        .unwrap_or_else(|| -1f64));
                if ml_library_inequality_1 {
                    self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(false)
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
            self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptPropertyDefinitionJoin3 {
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Next,
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(
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
            String,
            String,
            Vec<String>,
        ),
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result {
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Done(f64),
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_literal_property_index_loop4(
    key: &[f64],
    mut value: f64,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result {
    loop {
        return if at < (key.len() as f64) {
            {
                let digit = (self::ml_array::optional_at(key, self::ml_array::number_index(at)))
                    .unwrap_or(f64::NAN)
                    - 48f64;
                if (!(self::ml_math::is_integer(digit)) || (digit < 0f64)) || (digit > 9f64) {
                    self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Return(-1f64)
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
            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result {
    MlTemporaryFindJavaScriptOrdinaryObjectLoop5Done,
    MlTemporaryFindJavaScriptOrdinaryObjectLoop5Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_ordinary_object_loop5(
    mut objects: Vec<self::JavaScriptOrdinaryObject>,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let object =
                        self::ml_array::optional_at(&objects, self::ml_array::number_index(middle));
                    if match (object.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Return(-1f64)
                    } else {
                        let ml_product_condition_2 = {
                            let source_ml_source1_nullish_field_receiver785 = (object.clone(),);
                            if match (source_ml_source1_nullish_field_receiver785.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source1_nullish_field_present786 =
                                        ((source_ml_source1_nullish_field_receiver785.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source1_nullish_field_present786 {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                                            source_ml_source1_data_property_value787,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source1_data_property_value787.clone(),
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (String::from(
                                        "Cannot read properties of undefined (reading 'identity')",
                                    ))
                                    .encode_utf16()
                                    .collect(),
                                    message_present: true,
                                })
                                .raise()
                            }
                        };
                        if ml_product_condition_2 {
                            self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Return(middle)
                        } else {
                            {
                                let ml_product_condition_1 = {
                                    let source_ml_source1_nullish_field_receiver791 =
                                        (object.clone(),);
                                    if match (source_ml_source1_nullish_field_receiver791.clone()).0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source1_nullish_field_present792 =
                                                ((source_ml_source1_nullish_field_receiver791
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source1_nullish_field_present792 {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                                    source_ml_source1_data_property_value793,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => {
                                                    source_ml_source1_data_property_value793.clone()
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'identity')")).encode_utf16().collect(), message_present: true }).raise()
                                    }
                                };
                                if ml_product_condition_1 {
                                    {
                                        let low_2 = middle + 1f64;
                                        {
                                            low = low_2;
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result {
    MlTemporaryTestJavaScriptOrdinarySymbolLoop6Done,
    MlTemporaryTestJavaScriptOrdinarySymbolLoop6Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_ordinary_symbol_loop6(
    mut symbols: Vec<self::ml::Big>,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let symbol =
                        self::ml_array::optional_at(&symbols, self::ml_array::number_index(middle));
                    if {
                        let source_ml_source1_comparison_operand794 = symbol.clone();
                        {
                            let source_ml_source1_comparison_operand795 = None::<()>;
                            match (
                                source_ml_source1_comparison_operand794,
                                source_ml_source1_comparison_operand795,
                            ) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }
                        }
                    } {
                        self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Return(false)
                    } else if {
                        let source_ml_source1_comparison_operand796 = symbol.clone();
                        {
                            let source_ml_source1_comparison_operand797 = identity.clone();
                            match (
                                source_ml_source1_comparison_operand796,
                                source_ml_source1_comparison_operand797,
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            }
                        }
                    } {
                        self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Return(true)
                    } else if {
                        let source_ml_source1_comparison_operand798 = symbol.clone();
                        {
                            let source_ml_source1_comparison_operand799 = identity.clone();
                            if match (
                                source_ml_source1_comparison_operand798.clone(),
                                None::<self::ml::Big>,
                            ) {
                                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                (None, None) => true,
                                _ => false,
                            } {
                                {
                                    let source_ml_source1_comparison_order800 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,source_ml_source1_comparison_operand799.clone());
                                    (source_ml_source1_comparison_order800 == -1f64)
                                }
                            } else {
                                (source_ml_source1_comparison_operand798
                                    .clone()
                                    .expect("a proven present value"))
                                    < source_ml_source1_comparison_operand799
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
                            let high_2 = middle;
                            {
                                high = high_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result {
    MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Done,
    MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_ordinary_property_loop7(
    mut properties: Vec<self::JavaScriptOrdinaryProperty>,
    mut key: self::JavaScriptSemanticValue,
    mut at: f64,
) -> self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result {
    loop {
        return if at < (properties.len() as f64) {
            {
                let property =
                    self::ml_array::optional_at(&properties, self::ml_array::number_index(at));
                if !(match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) && self::test_java_script_semantic_same_value(
                    {
                        let source_ml_source1_nullish_field_receiver801 = (property.clone(),);
                        if match (source_ml_source1_nullish_field_receiver801.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present802 =
                                    ((source_ml_source1_nullish_field_receiver801.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_nullish_field_present802 {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        source_ml_source1_data_property_value803,
                                        _,
                                    ) => {
                                        let source_ml_source1_data_property_value803 =
                                            *source_ml_source1_data_property_value803;
                                        source_ml_source1_data_property_value803.clone()
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
                    key.clone(),
                ) {
                    self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Return(at)
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
            self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(String),
}
