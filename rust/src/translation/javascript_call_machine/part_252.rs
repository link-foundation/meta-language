// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::accept_java_script_whitespace_unit;
use super::accept_unicode_case_property;
use super::ml_array;
use super::read_unicode_case_mapping;

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1343 {
    MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1343Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1342 {
    MlSource1TemporaryParseJavaScriptIntegerPrefixJoin1342Next(f64, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_parse_java_script_float_prefix_loop1345(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut start: f64,
) -> f64 {
    loop {
        return if ((start
            < ({
                let ml_reference_array3115 = units.clone();
                let ml_reference_projection3115 = ml_reference_array3115.borrow().len() as f64;
                ml_reference_projection3115
            }))
            && self::accept_java_script_whitespace_unit(
                ({
                    let ml_reference_array3116 = units.clone();
                    let ml_reference_position3116 = start;
                    let ml_reference_projection3116 = self::ml_array::optional_at(
                        &*ml_reference_array3116.borrow(),
                        self::ml_array::number_index(ml_reference_position3116),
                    );
                    ml_reference_projection3116
                })
                .clone()
                .unwrap_or_else(|| (-1f64)),
            )) {
            {
                let start_2 = (start + 1f64);
                {
                    start = start_2;
                    continue;
                }
            }
        } else {
            start
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_parse_java_script_float_prefix_loop1346(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
    mut infinity: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut matches: bool,
    mut index: f64,
) -> bool {
    loop {
        return if (index
            < ({
                let ml_reference_array3117 = infinity.clone();
                let ml_reference_projection3117 = ml_reference_array3117.borrow().len() as f64;
                ml_reference_projection3117
            })) {
            if ((({
                let source_ml_source2_array_read_index42033 = (at + index);
                {
                    let ml_reference_array3118 = units.clone();
                    let ml_reference_position3118 = source_ml_source2_array_read_index42033;
                    let ml_reference_projection3118 = self::ml_array::optional_at(
                        &*ml_reference_array3118.borrow(),
                        self::ml_array::number_index(ml_reference_position3118),
                    );
                    ml_reference_projection3118
                }
            })
            .clone()
            .unwrap_or_else(|| (-1f64)))
                != (({
                    let ml_reference_array3119 = infinity.clone();
                    let ml_reference_position3119 = index;
                    let ml_reference_projection3119 = self::ml_array::optional_at(
                        &*ml_reference_array3119.borrow(),
                        self::ml_array::number_index(ml_reference_position3119),
                    );
                    ml_reference_projection3119
                })
                .clone()
                .unwrap_or_else(|| (-2f64))))
            {
                {
                    let index_2 = (index + 1f64);
                    {
                        (matches, index) = (false, index_2);
                        continue;
                    }
                }
            } else {
                {
                    let index_3 = (index + 1f64);
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        } else {
            matches
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_parse_java_script_float_prefix_loop1349(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> f64 {
    loop {
        return if (((({
            let ml_reference_array3120 = units.clone();
            let ml_reference_position3120 = at;
            let ml_reference_projection3120 = self::ml_array::optional_at(
                &*ml_reference_array3120.borrow(),
                self::ml_array::number_index(ml_reference_position3120),
            );
            ml_reference_projection3120
        })
        .clone()
        .unwrap_or_else(|| (-1f64)))
            >= (48f64))
            && ((({
                let ml_reference_array3121 = units.clone();
                let ml_reference_position3121 = at;
                let ml_reference_projection3121 = self::ml_array::optional_at(
                    &*ml_reference_array3121.borrow(),
                    self::ml_array::number_index(ml_reference_position3121),
                );
                ml_reference_projection3121
            })
            .clone()
            .unwrap_or_else(|| (-1f64)))
                <= (57f64)))
        {
            {
                let at_2 = (at + 1f64);
                {
                    at = at_2;
                    continue;
                }
            }
        } else {
            at
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryParseJavaScriptFloatPrefixJoin1348 {
    MlSource1TemporaryParseJavaScriptFloatPrefixJoin1348Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryParseJavaScriptFloatPrefixLoop1350Result {
    MlSource1TemporaryParseJavaScriptFloatPrefixLoop1350Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_parse_java_script_float_prefix_loop1350(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
    mut digits: f64,
) -> self::MlSource1TemporaryParseJavaScriptFloatPrefixLoop1350Result {
    loop {
        return if (((({
            let ml_reference_array3122 = units.clone();
            let ml_reference_position3122 = at;
            let ml_reference_projection3122 = self::ml_array::optional_at(
                &*ml_reference_array3122.borrow(),
                self::ml_array::number_index(ml_reference_position3122),
            );
            ml_reference_projection3122
        })
        .clone()
        .unwrap_or_else(|| (-1f64)))
            >= (48f64))
            && ((({
                let ml_reference_array3123 = units.clone();
                let ml_reference_position3123 = at;
                let ml_reference_projection3123 = self::ml_array::optional_at(
                    &*ml_reference_array3123.borrow(),
                    self::ml_array::number_index(ml_reference_position3123),
                );
                ml_reference_projection3123
            })
            .clone()
            .unwrap_or_else(|| (-1f64)))
                <= (57f64)))
        {
            {
                let at_2 = (at + 1f64);
                {
                    let digits_2 = (digits + 1f64);
                    {
                        (at, digits) = (at_2, digits_2);
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryParseJavaScriptFloatPrefixLoop1350Result::MlSource1TemporaryParseJavaScriptFloatPrefixLoop1350Done(at, digits)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryParseJavaScriptFloatPrefixLoop1347Result {
    MlSource1TemporaryParseJavaScriptFloatPrefixLoop1347Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_parse_java_script_float_prefix_loop1347(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
    mut digits: f64,
) -> self::MlSource1TemporaryParseJavaScriptFloatPrefixLoop1347Result {
    loop {
        return if (((({
            let ml_reference_array3124 = units.clone();
            let ml_reference_position3124 = at;
            let ml_reference_projection3124 = self::ml_array::optional_at(
                &*ml_reference_array3124.borrow(),
                self::ml_array::number_index(ml_reference_position3124),
            );
            ml_reference_projection3124
        })
        .clone()
        .unwrap_or_else(|| (-1f64)))
            >= (48f64))
            && ((({
                let ml_reference_array3125 = units.clone();
                let ml_reference_position3125 = at;
                let ml_reference_projection3125 = self::ml_array::optional_at(
                    &*ml_reference_array3125.borrow(),
                    self::ml_array::number_index(ml_reference_position3125),
                );
                ml_reference_projection3125
            })
            .clone()
            .unwrap_or_else(|| (-1f64)))
                <= (57f64)))
        {
            {
                let at_2 = (at + 1f64);
                {
                    let digits_2 = (digits + 1f64);
                    {
                        (at, digits) = (at_2, digits_2);
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryParseJavaScriptFloatPrefixLoop1347Result::MlSource1TemporaryParseJavaScriptFloatPrefixLoop1347Done(at, digits)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptUnicodeCasePropertyLoop1351Result {
    MlSource1TemporaryAcceptUnicodeCasePropertyLoop1351Done,
    MlSource1TemporaryAcceptUnicodeCasePropertyLoop1351Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_unicode_case_property_loop1351(
    mut point: f64,
    mut ranges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut first: f64,
    mut last: f64,
) -> self::MlSource1TemporaryAcceptUnicodeCasePropertyLoop1351Result {
    loop {
        return if (first < last) {
            {
                let middle = ((first + last) / 2f64).floor();
                {
                    let start = ({
                        let source_ml_source2_array_read_index42061 = (middle * 2f64);
                        {
                            let ml_reference_array3126 = ranges.clone();
                            let ml_reference_position3126 = source_ml_source2_array_read_index42061;
                            let ml_reference_projection3126 = self::ml_array::optional_at(
                                &*ml_reference_array3126.borrow(),
                                self::ml_array::number_index(ml_reference_position3126),
                            );
                            ml_reference_projection3126
                        }
                    })
                    .clone()
                    .unwrap_or_else(|| (-1f64));
                    {
                        let end = ({
                            let source_ml_source2_array_read_index42063 = ((middle * 2f64) + 1f64);
                            {
                                let ml_reference_array3127 = ranges.clone();
                                let ml_reference_position3127 =
                                    source_ml_source2_array_read_index42063;
                                let ml_reference_projection3127 = self::ml_array::optional_at(
                                    &*ml_reference_array3127.borrow(),
                                    self::ml_array::number_index(ml_reference_position3127),
                                );
                                ml_reference_projection3127
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| (-1f64));
                        if (point < start) {
                            {
                                last = middle;
                                continue;
                            }
                        } else {
                            if (point > end) {
                                {
                                    let first_2 = (middle + 1f64);
                                    {
                                        first = first_2;
                                        continue;
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryAcceptUnicodeCasePropertyLoop1351Result::MlSource1TemporaryAcceptUnicodeCasePropertyLoop1351Return(true)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptUnicodeCasePropertyLoop1351Result::MlSource1TemporaryAcceptUnicodeCasePropertyLoop1351Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadUnicodeCaseMappingLoop1353Result {
    MlSource1TemporaryReadUnicodeCaseMappingLoop1353Done,
    MlSource1TemporaryReadUnicodeCaseMappingLoop1353Return(
        std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_unicode_case_mapping_loop1353(
    mut point: f64,
    mut ranges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut first: f64,
    mut last: f64,
) -> self::MlSource1TemporaryReadUnicodeCaseMappingLoop1353Result {
    loop {
        return if (first < last) {
            {
                let middle = ((first + last) / 2f64).floor();
                {
                    let start = ({
                        let source_ml_source2_array_read_index42065 = (middle * 4f64);
                        {
                            let ml_reference_array3128 = ranges.clone();
                            let ml_reference_position3128 = source_ml_source2_array_read_index42065;
                            let ml_reference_projection3128 = self::ml_array::optional_at(
                                &*ml_reference_array3128.borrow(),
                                self::ml_array::number_index(ml_reference_position3128),
                            );
                            ml_reference_projection3128
                        }
                    })
                    .clone()
                    .unwrap_or_else(|| (-1f64));
                    {
                        let end = ({
                            let source_ml_source2_array_read_index42067 = ((middle * 4f64) + 1f64);
                            {
                                let ml_reference_array3129 = ranges.clone();
                                let ml_reference_position3129 =
                                    source_ml_source2_array_read_index42067;
                                let ml_reference_projection3129 = self::ml_array::optional_at(
                                    &*ml_reference_array3129.borrow(),
                                    self::ml_array::number_index(ml_reference_position3129),
                                );
                                ml_reference_projection3129
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| (-1f64));
                        if (point < start) {
                            {
                                last = middle;
                                continue;
                            }
                        } else {
                            if (point > end) {
                                {
                                    let first_2 = (middle + 1f64);
                                    {
                                        first = first_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let stride = ({
                                        let source_ml_source2_array_read_index42069 =
                                            ((middle * 4f64) + 2f64);
                                        {
                                            let ml_reference_array3130 = ranges.clone();
                                            let ml_reference_position3130 =
                                                source_ml_source2_array_read_index42069;
                                            let ml_reference_projection3130 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array3130.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position3130,
                                                    ),
                                                );
                                            ml_reference_projection3130
                                        }
                                    })
                                    .clone()
                                    .unwrap_or_else(|| 1f64);
                                    {
                                        let delta = ({
                                            let source_ml_source2_array_read_index42071 =
                                                ((middle * 4f64) + 3f64);
                                            {
                                                let ml_reference_array3131 = ranges.clone();
                                                let ml_reference_position3131 =
                                                    source_ml_source2_array_read_index42071;
                                                let ml_reference_projection3131 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array3131.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position3131,
                                                        ),
                                                    );
                                                ml_reference_projection3131
                                            }
                                        })
                                        .clone()
                                        .unwrap_or_else(|| 0f64);
                                        self::MlSource1TemporaryReadUnicodeCaseMappingLoop1353Result::MlSource1TemporaryReadUnicodeCaseMappingLoop1353Return(std::rc::Rc::new(std::cell::RefCell::new(vec![if ((((point - start) % stride)) == (0f64)) {
                                            (point + delta)
                                        } else {
                                            point
                                        }])))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadUnicodeCaseMappingLoop1353Result::MlSource1TemporaryReadUnicodeCaseMappingLoop1353Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadUnicodeCaseMappingLoop1352Result {
    MlSource1TemporaryReadUnicodeCaseMappingLoop1352Done(f64, f64),
    MlSource1TemporaryReadUnicodeCaseMappingLoop1352Return(
        std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadUnicodeCaseMappingLoop1354Result {
    MlSource1TemporaryReadUnicodeCaseMappingLoop1354Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_unicode_case_mapping_loop1354(
    mut values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut offset: f64,
    mut count: f64,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryReadUnicodeCaseMappingLoop1354Result {
    loop {
        return if (at < count) {
            {
                let source_ml_source1_temporary_read_unicode_case_mapping_ignored1355 = {
                    let ml_method_owner3132 = result.clone();
                    let ml_method_argument3132_0 = ({
                        let source_ml_source2_array_read_index42073 = (offset + at);
                        {
                            let ml_reference_array3133 = values.clone();
                            let ml_reference_position3133 = source_ml_source2_array_read_index42073;
                            let ml_reference_projection3133 = self::ml_array::optional_at(
                                &*ml_reference_array3133.borrow(),
                                self::ml_array::number_index(ml_reference_position3133),
                            );
                            ml_reference_projection3133
                        }
                    })
                    .clone()
                    .unwrap_or_else(|| 0f64);
                    let ml_inserted = vec![ml_method_argument3132_0];
                    let mut ml_method_values3132 = ml_method_owner3132.borrow_mut();
                    let ml_length = ml_method_values3132
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values3132.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadUnicodeCaseMappingLoop1354Result::MlSource1TemporaryReadUnicodeCaseMappingLoop1354Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_unicode_case_mapping_loop1352(
    mut point: f64,
    mut indexes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut first: f64,
    mut last: f64,
) -> self::MlSource1TemporaryReadUnicodeCaseMappingLoop1352Result {
    loop {
        return if (first < last) {
            {
                let middle = ((first + last) / 2f64).floor();
                {
                    let key = ({
                        let source_ml_source2_array_read_index42075 = (middle * 3f64);
                        {
                            let ml_reference_array3134 = indexes.clone();
                            let ml_reference_position3134 = source_ml_source2_array_read_index42075;
                            let ml_reference_projection3134 = self::ml_array::optional_at(
                                &*ml_reference_array3134.borrow(),
                                self::ml_array::number_index(ml_reference_position3134),
                            );
                            ml_reference_projection3134
                        }
                    })
                    .clone()
                    .unwrap_or_else(|| (-1f64));
                    if (point < key) {
                        {
                            last = middle;
                            continue;
                        }
                    } else {
                        if (point > key) {
                            {
                                let first_2 = (middle + 1f64);
                                {
                                    first = first_2;
                                    continue;
                                }
                            }
                        } else {
                            {
                                let offset = ({
                                    let source_ml_source2_array_read_index42077 =
                                        ((middle * 3f64) + 1f64);
                                    {
                                        let ml_reference_array3135 = indexes.clone();
                                        let ml_reference_position3135 =
                                            source_ml_source2_array_read_index42077;
                                        let ml_reference_projection3135 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array3135.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position3135,
                                                ),
                                            );
                                        ml_reference_projection3135
                                    }
                                })
                                .clone()
                                .unwrap_or_else(|| 0f64);
                                {
                                    let count = ({
                                        let source_ml_source2_array_read_index42079 =
                                            ((middle * 3f64) + 2f64);
                                        {
                                            let ml_reference_array3136 = indexes.clone();
                                            let ml_reference_position3136 =
                                                source_ml_source2_array_read_index42079;
                                            let ml_reference_projection3136 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array3136.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position3136,
                                                    ),
                                                );
                                            ml_reference_projection3136
                                        }
                                    })
                                    .clone()
                                    .unwrap_or_else(|| 0f64);
                                    {
                                        let result = std::rc::Rc::new(std::cell::RefCell::new(
                                            Vec::<f64>::new(),
                                        ));
                                        {
                                            let at = 0f64;
                                            {
                                                let source_ml_source2_s42080 = self::source_ml_source1_temporary_read_unicode_case_mapping_loop1354(values.clone(), offset, count, result.clone(), at);
                                                match source_ml_source2_s42080.clone() {
                                                    self::MlSource1TemporaryReadUnicodeCaseMappingLoop1354Result::MlSource1TemporaryReadUnicodeCaseMappingLoop1354Done => {
                                                        self::MlSource1TemporaryReadUnicodeCaseMappingLoop1352Result::MlSource1TemporaryReadUnicodeCaseMappingLoop1352Return(result.clone())
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
        } else {
            self::MlSource1TemporaryReadUnicodeCaseMappingLoop1352Result::MlSource1TemporaryReadUnicodeCaseMappingLoop1352Done(first, last)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1358Result {
    MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1358Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1359Result {
    MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1359Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_map_unicode_case_utf16_columns_loop1359(
    mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut mapped: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1359Result {
    loop {
        return if (index
            < ({
                let ml_reference_array3137 = mapped.clone();
                let ml_reference_projection3137 = ml_reference_array3137.borrow().len() as f64;
                ml_reference_projection3137
            })) {
            {
                let value = ({
                    let ml_reference_array3138 = mapped.clone();
                    let ml_reference_position3138 = index;
                    let ml_reference_projection3138 = self::ml_array::optional_at(
                        &*ml_reference_array3138.borrow(),
                        self::ml_array::number_index(ml_reference_position3138),
                    );
                    ml_reference_projection3138
                })
                .clone()
                .unwrap_or_else(|| 0f64);
                if (value < (65536f64)) {
                    {
                        let source_ml_source1_temporary_map_unicode_case_utf16_columns_ignored1360 = {
                            let ml_method_owner3139 = result.clone();
                            let ml_method_argument3139_0 = value;
                            let ml_inserted = vec![ml_method_argument3139_0];
                            let mut ml_method_values3139 = ml_method_owner3139.borrow_mut();
                            let ml_length = ml_method_values3139
                                .len()
                                .checked_add(ml_inserted.len())
                                .filter(|length| *length <= 4294967295usize)
                                .expect("Invalid array length");
                            ml_method_values3139.extend(ml_inserted);
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
                    {
                        let source_ml_source1_temporary_map_unicode_case_utf16_columns_ignored1361 = {
                            let ml_method_owner3140 = result.clone();
                            let ml_method_argument3140_0 =
                                (55296f64 + ((value - 65536f64) / 1024f64).floor());
                            let ml_inserted = vec![ml_method_argument3140_0];
                            let mut ml_method_values3140 = ml_method_owner3140.borrow_mut();
                            let ml_length = ml_method_values3140
                                .len()
                                .checked_add(ml_inserted.len())
                                .filter(|length| *length <= 4294967295usize)
                                .expect("Invalid array length");
                            ml_method_values3140.extend(ml_inserted);
                            ml_length as f64
                        };
                        {
                            let source_ml_source1_temporary_map_unicode_case_utf16_columns_ignored1362 = {
                                let ml_method_owner3141 = result.clone();
                                let ml_method_argument3141_0 =
                                    (56320f64 + ((value - 65536f64) % 1024f64));
                                let ml_inserted = vec![ml_method_argument3141_0];
                                let mut ml_method_values3141 = ml_method_owner3141.borrow_mut();
                                let ml_length = ml_method_values3141
                                    .len()
                                    .checked_add(ml_inserted.len())
                                    .filter(|length| *length <= 4294967295usize)
                                    .expect("Invalid array length");
                                ml_method_values3141.extend(ml_inserted);
                                ml_length as f64
                            };
                            {
                                let index_3 = (index + 1f64);
                                {
                                    index = index_3;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1359Result::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1359Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_map_unicode_case_utf16_columns_loop1358(
    mut upper: bool,
    mut ranges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut indexes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut cased: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut ignorable: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut points: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut following: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut previous: bool,
    mut at: f64,
) -> self::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1358Result {
    loop {
        return if (at
            < ({
                let ml_reference_array3142 = points.clone();
                let ml_reference_projection3142 = ml_reference_array3142.borrow().len() as f64;
                ml_reference_projection3142
            })) {
            {
                let point = ({
                    let ml_reference_array3143 = points.clone();
                    let ml_reference_position3143 = at;
                    let ml_reference_projection3143 = self::ml_array::optional_at(
                        &*ml_reference_array3143.borrow(),
                        self::ml_array::number_index(ml_reference_position3143),
                    );
                    ml_reference_projection3143
                })
                .clone()
                .unwrap_or_else(|| 0f64);
                {
                    let final_sigma = (((!upper && (point == (931f64))) && previous)
                        && !(({
                            let source_ml_source2_array_read_index42086 = (({
                                let ml_reference_array3144 = points.clone();
                                let ml_reference_projection3144 =
                                    ml_reference_array3144.borrow().len() as f64;
                                ml_reference_projection3144
                            } - 1f64)
                                - at);
                            {
                                let ml_reference_array3145 = following.clone();
                                let ml_reference_position3145 =
                                    source_ml_source2_array_read_index42086;
                                let ml_reference_projection3145 = self::ml_array::optional_at(
                                    &*ml_reference_array3145.borrow(),
                                    self::ml_array::number_index(ml_reference_position3145),
                                );
                                ml_reference_projection3145
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| false)));
                    {
                        let mapped = if final_sigma {
                            std::rc::Rc::new(std::cell::RefCell::new(vec![962f64]))
                        } else {
                            self::read_unicode_case_mapping(
                                point,
                                ranges.clone(),
                                indexes.clone(),
                                values.clone(),
                            )
                        };
                        {
                            let index = 0f64;
                            {
                                let source_ml_source2_s42087 = self::source_ml_source1_temporary_map_unicode_case_utf16_columns_loop1359(result.clone(), mapped.clone(), index);
                                match source_ml_source2_s42087.clone() {
                                    self::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1359Result::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1359Done => {
                                        if !(self::accept_unicode_case_property(point, ignorable.clone())) {
                                            {
                                                let previous_2 = self::accept_unicode_case_property(point, cased.clone());
                                                {
                                                    let at_2 = (at + 1f64);
                                                    {
                                                        (previous, at) = (previous_2, at_2);
                                                        continue;
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                let at_3 = (at + 1f64);
                                                {
                                                    at = at_3;
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
            }
        } else {
            self::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1358Result::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1358Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1357Result {
    MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1357Done,
}
