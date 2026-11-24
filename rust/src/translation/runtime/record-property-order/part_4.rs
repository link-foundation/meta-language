
#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReplaceUtf16TextLoop24Result {
    MlTemporaryReplaceUtf16TextLoop24Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop26(text: &[f64], mut result: Vec<f64>, mut found: f64, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < found) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop28(mut result: Vec<f64>, mut last: f64, source: &[f64], mut at: f64) -> Vec<f64> {
    loop {
        return if (at < last) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(source, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop27(text: &[f64], search: &[f64], replacement: &[f64], mut result: Vec<f64>, plan: &[f64], mut found: f64, mut instruction: f64) -> Vec<f64> {
    loop {
        return if (instruction < ((plan.len() as f64))) {
            {
                let kind = (self::ml_array::optional_at(plan, self::ml_array::number_index(instruction))).clone().unwrap_or_else(|| 0f64);
                {
                    let first = if (kind == (0f64)) {
                        (self::ml_array::optional_at(plan, self::ml_array::number_index((instruction + 1f64)))).clone().unwrap_or_else(|| 0f64)
                    } else {
                        if (kind == (2f64)) {
                            0f64
                        } else {
                            if (kind == (1f64)) {
                                found
                            } else {
                                (found + (search.len() as f64))
                            }
                        }
                    };
                    {
                        let last = if (kind == (0f64)) {
                            (self::ml_array::optional_at(plan, self::ml_array::number_index((instruction + 2f64)))).clone().unwrap_or_else(|| 0f64)
                        } else {
                            if (kind == (2f64)) {
                                found
                            } else {
                                if (kind == (1f64)) {
                                    (found + (search.len() as f64))
                                } else {
                                    (text.len() as f64)
                                }
                            }
                        };
                        {
                            let source = if (kind == (0f64)) {
                                replacement.to_vec()
                            } else {
                                text.to_vec()
                            };
                            {
                                let at = first;
                                {
                                    let result_2 = self::source_ml_temporary_replace_utf16_text_loop28(result, last, &(source), at);
                                    {
                                        let instruction_2 = (instruction + 3f64);
                                        {
                                            (result, instruction) = (result_2, instruction_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop24(text: &[f64], search: &[f64], replacement: &[f64], mut all: bool, mut result: Vec<f64>, plan: &[f64], mut start: f64, mut from: f64, mut searching: bool) -> self::MlTemporaryReplaceUtf16TextLoop24Result {
    loop {
        return if searching {
            {
                let found = self::index_utf16_text(text, search, from);
                if (found < (0f64)) {
                    {
                        let searching_2 = false;
                        {
                            searching = searching_2;
                            continue;
                        }
                    }
                } else {
                    {
                        let at = start;
                        {
                            let result_2 = self::source_ml_temporary_replace_utf16_text_loop26(text, result, found, at);
                            {
                                let instruction = 0f64;
                                {
                                    let result_3 = self::source_ml_temporary_replace_utf16_text_loop27(text, search, replacement, result_2, plan, found, instruction);
                                    {
                                        let start_2 = (found + (search.len() as f64));
                                        {
                                            let searching_3 = (all && !(((((search.len() as f64)) == (0f64)) && (found == ((text.len() as f64))))));
                                            {
                                                let from_2 = (start_2 + if (((search.len() as f64)) == (0f64)) {
                                                    1f64
                                                } else {
                                                    0f64
                                                });
                                                {
                                                    (result, start, from, searching) = (result_3, start_2, from_2, searching_3);
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
            self::MlTemporaryReplaceUtf16TextLoop24Result::MlTemporaryReplaceUtf16TextLoop24Done(result, start)
        };
    }
}
}


/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> self::ml_string::Text {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 116, 101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 66, 111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_number_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[self::ml_string::Text], index: f64) -> self::ml_string::Text {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 116, 101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 66, 111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_boolean_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_record_name() -> Vec<f64> {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 110, 97, 109, 101, 32, 114, 111, 119])).0, message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_record_name(names: &[Vec<f64>], index: f64) -> Vec<f64> {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((names.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 110, 97, 109, 101, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(names, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_record_name())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_value_order(count: f64, reverse: bool) -> Vec<f64> {
    if (!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) {
        vec![(-1f64)]
    } else {
        {
            let order = Vec::<f64>::new();
            {
                let index = 0f64;
                {
                    let order_2 = self::source_ml_temporary_read_record_value_order_loop1(count, reverse, order, index);
                    order_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_projection() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![99, 104, 101, 99, 107, 101, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 118, 97, 108, 117, 101, 115])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_projection_error(projection: &self::ml_string::Text, names: &[Vec<f64>], entries: &[bool]) -> self::ml_string::Text {
    if ((*projection) != (self::read_record_projection())) {
        self::ml_string::Text::from_units(vec![117, 110, 107, 110, 111, 119, 110, 32, 99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110])
    } else {
        if (((names.len() as f64)) != ((entries.len() as f64))) {
            self::ml_string::Text::from_units(vec![99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 32, 104, 97, 115, 32, 105, 110, 99, 111, 110, 115, 105, 115, 116, 101, 110, 116, 32, 101, 110, 116, 114, 121, 32, 102, 97, 99, 116, 115])
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source1_s1 = self::source_ml_temporary_read_record_projection_error_loop2(names, entries, index);
                    match source_ml_source1_s1.clone() {
                        self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Done => {
                            self::ml_string::Text::from_units(vec![])
                        }
                        self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Return(source_ml_temporary_result2) => {
                            source_ml_temporary_result2.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_record_field_name(name: &[f64]) -> bool {
    {
        let index = 0f64;
        {
            let source_ml_source1_s2 = self::source_ml_temporary_accept_record_field_name_loop3(name, index);
            match source_ml_source1_s2.clone() {
                self::MlTemporaryAcceptRecordFieldNameLoop3Result::MlTemporaryAcceptRecordFieldNameLoop3Done => {
                    true
                }
                self::MlTemporaryAcceptRecordFieldNameLoop3Result::MlTemporaryAcceptRecordFieldNameLoop3Return(source_ml_temporary_result3) => {
                    source_ml_temporary_result3
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_record_field_names(left: &[f64], right: &[f64]) -> f64 {
    {
        let end = if (((left.len() as f64)) < ((right.len() as f64))) {
            (left.len() as f64)
        } else {
            (right.len() as f64)
        };
        {
            let index = 0f64;
            {
                let source_ml_source1_s3 = self::source_ml_temporary_compare_record_field_names_loop4(left, right, end, index);
                match source_ml_source1_s3.clone() {
                    self::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Done => {
                        if (((left.len() as f64)) < ((right.len() as f64))) {
                            (-1f64)
                        } else {
                            if (((left.len() as f64)) > ((right.len() as f64))) {
                                1f64
                            } else {
                                0f64
                            }
                        }
                    }
                    self::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Return(source_ml_temporary_result4) => {
                        source_ml_temporary_result4
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_position(fields: &[Vec<f64>], name: &[f64]) -> f64 {
    if !(self::accept_record_field_name(name)) {
        (-1f64)
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s4 = self::source_ml_temporary_read_record_field_position_loop5(fields, index);
                match source_ml_source1_s4.clone() {
                    self::MlTemporaryReadRecordFieldPositionLoop5Result::MlTemporaryReadRecordFieldPositionLoop5Done => {
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source1_s5 = self::source_ml_temporary_read_record_field_position_loop6(fields, name, index_2);
                                match source_ml_source1_s5.clone() {
                                    self::MlTemporaryReadRecordFieldPositionLoop6Result::MlTemporaryReadRecordFieldPositionLoop6Done => {
                                        (-1f64)
                                    }
                                    self::MlTemporaryReadRecordFieldPositionLoop6Result::MlTemporaryReadRecordFieldPositionLoop6Return(source_ml_temporary_result6) => {
                                        source_ml_temporary_result6
                                    }
                                }
                            }
                        }
                    }
                    self::MlTemporaryReadRecordFieldPositionLoop5Result::MlTemporaryReadRecordFieldPositionLoop5Return(source_ml_temporary_result6_2) => {
                        source_ml_temporary_result6_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_sorted_field_positions(fields: &[Vec<f64>]) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let source_ml_source1_s6 = self::source_ml_temporary_read_record_sorted_field_positions_loop7(fields, index);
            match source_ml_source1_s6.clone() {
                self::MlTemporaryReadRecordSortedFieldPositionsLoop7Result::MlTemporaryReadRecordSortedFieldPositionsLoop7Done => {
                    {
                        let sorted = self::read_record_value_order((fields.len() as f64), false);
                        {
                            let width = 1f64;
                            {
                                let sorted_2 = self::source_ml_temporary_read_record_sorted_field_positions_loop8(fields, sorted, width);
                                sorted_2
                            }
                        }
                    }
                }
                self::MlTemporaryReadRecordSortedFieldPositionsLoop7Result::MlTemporaryReadRecordSortedFieldPositionsLoop7Return(source_ml_temporary_result10) => {
                    source_ml_temporary_result10
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_order(fields: &[Vec<f64>]) -> Vec<f64> {
    {
        let sorted = self::read_record_sorted_field_positions(fields);
        if ((((sorted.len() as f64)) > (0f64)) && ((self::read_required_number_fact(&(sorted), 0f64)) < (0f64))) {
            vec![(-1f64)]
        } else {
            {
                let selected = Vec::<f64>::new();
                {
                    let index = 0f64;
                    {
                        let selected_2 = self::source_ml_temporary_read_record_field_order_loop11(fields, &(sorted), selected, index);
                        selected_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_insertion_order(fields: &[Vec<f64>]) -> Vec<f64> {
    {
        let sorted = self::read_record_sorted_field_positions(fields);
        if ((((sorted.len() as f64)) > (0f64)) && ((self::read_required_number_fact(&(sorted), 0f64)) < (0f64))) {
            vec![(-1f64)]
        } else {
            {
                let first = Vec::<f64>::new();
                {
                    let last = Vec::<f64>::new();
                    {
                        let start = 0f64;
                        {
                            let index = 0f64;
                            {
                                let source_ml_source1_s7 = self::source_ml_temporary_read_record_field_insertion_order_loop12(fields, &(sorted), first, last, start, index);
                                match source_ml_source1_s7.clone() {
                                    self::MlTemporaryReadRecordFieldInsertionOrderLoop12Result::MlTemporaryReadRecordFieldInsertionOrderLoop12Done(first_2, last_2) => {
                                        {
                                            let selected = Vec::<f64>::new();
                                            {
                                                let index_2 = 0f64;
                                                {
                                                    let selected_2 = self::source_ml_temporary_read_record_field_insertion_order_loop13(fields, &(first_2), &(last_2), selected, index_2);
                                                    selected_2
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
pub fn read_record_spread_role(kind: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*kind) == (self::ml_string::Text::from_units(vec![118, 97, 114]))) {
        self::ml_string::Text::from_units(vec![111, 112, 101, 110])
    } else {
        if ((*kind) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) {
            self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])
        } else {
            self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_type_constraint_step(left: &[Vec<f64>], right: &[Vec<f64>], left_open: bool, right_open: bool, phase: f64, index: f64) -> Vec<f64> {
    if (phase < (1f64)) {
        if (index >= ((left.len() as f64))) {
            vec![5f64, (-1f64), (-1f64), 1f64, 0f64]
        } else {
            {
                let matching = self::read_record_field_position(right, &(self::read_required_record_name(left, index)));
                if (matching >= (0f64)) {
                    vec![0f64, index, matching, 0f64, (index + 1f64)]
                } else {
                    vec![if right_open {
                        1f64
                    } else {
                        3f64
                    }, index, (-1f64), 0f64, (index + 1f64)]
                }
            }
        }
    } else {
        if (index >= ((right.len() as f64))) {
            vec![6f64, (-1f64), (-1f64), 1f64, index]
        } else {
            {
                let matching_2 = self::read_record_field_position(left, &(self::read_required_record_name(right, index)));
                if (matching_2 >= (0f64)) {
                    vec![5f64, matching_2, index, 1f64, (index + 1f64)]
                } else {
                    vec![if left_open {
                        2f64
                    } else {
                        4f64
                    }, (-1f64), index, 1f64, (index + 1f64)]
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_type_constraint_error(action: f64, name: &self::ml_string::Text) -> self::ml_string::Text {
    if ((action == (3f64)) || (action == (4f64))) {
        ((self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32])).concat(name)).concat(&self::ml_string::Text::from_units(vec![32, 105, 115, 32, 109, 105, 115, 115, 105, 110, 103]))
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_method_error(kind: &self::ml_string::Text, operation: &self::ml_string::Text, parameters: f64, arguments_count: f64) -> self::ml_string::Text {
    if ((*kind) != (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
        (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 32, 104, 97, 115, 32, 110, 111, 32, 109, 101, 116, 104, 111, 100, 32])).concat(operation)
    } else {
        if (arguments_count != parameters) {
            ((((self::ml_string::Text::from_units(vec![46])).concat(operation)).concat(&self::ml_string::Text::from_units(vec![40, 41, 32, 101, 120, 112, 101, 99, 116, 115, 32]))).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(parameters)))).concat(&self::ml_string::Text::from_units(vec![32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_inference_step(fields: &[Vec<f64>], name: &[f64], open: bool) -> Vec<f64> {
    {
        let index = self::read_record_field_position(fields, name);
        if (index >= (0f64)) {
            vec![0f64, index]
        } else {
            vec![if open {
                1f64
            } else {
                2f64
            }, (-1f64)]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn record_type_fact_field(starts: &[f64], counts: &[f64], labels: &[self::ml_string::Text], targets: &[f64], owner: f64, name: &self::ml_string::Text) -> f64 {
    if (owner < (0f64)) {
        (-1f64)
    } else {
        {
            let start = self::read_required_number_fact(starts, owner);
            {
                let end = (start + self::read_required_number_fact(counts, owner));
                {
                    let edge = start;
                    {
                        let source_ml_source1_s8 = self::source_ml_temporary_record_type_fact_field_loop15(labels, targets, name, end, edge);
                        match source_ml_source1_s8.clone() {
                            self::MlTemporaryRecordTypeFactFieldLoop15Result::MlTemporaryRecordTypeFactFieldLoop15Done => {
                                (-1f64)
                            }
                            self::MlTemporaryRecordTypeFactFieldLoop15Result::MlTemporaryRecordTypeFactFieldLoop15Return(source_ml_temporary_result15) => {
                                source_ml_temporary_result15
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
pub fn record_type_fact_kind(texts: &[self::ml_string::Text], starts: &[f64], counts: &[f64], labels: &[self::ml_string::Text], targets: &[f64], owner: f64) -> self::ml_string::Text {
    {
        let kind = self::record_type_fact_field(starts, counts, labels, targets, owner, &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])));
        if (kind < (0f64)) {
            self::ml_string::Text::from_units(vec![])
        } else {
            self::read_required_text_fact(texts, kind)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn record_type_fact_elements(starts: &[f64], counts: &[f64], targets: &[f64], owner: f64) -> Vec<f64> {
    {
        let values = Vec::<f64>::new();
        if (owner < (0f64)) {
            values
        } else {
            {
                let start = self::read_required_number_fact(starts, owner);
                {
                    let end = (start + self::read_required_number_fact(counts, owner));
                    {
                        let edge = start;
                        {
                            let values_2 = self::source_ml_temporary_record_type_fact_elements_loop16(targets, values, end, edge);
                            values_2
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_type_owner_presence(texts: &[self::ml_string::Text], numbers: &[f64], starts: &[f64], counts: &[f64], labels: &[self::ml_string::Text], targets: &[f64], bindings: &[f64], root: f64, owner: f64) -> bool {
    {
        let pending = vec![root];
        {
            let seen = Vec::<f64>::new();
            {
                let position = 0f64;
                {
                    let source_ml_source1_s9 = self::source_ml_temporary_read_record_type_owner_presence_loop17(texts, numbers, starts, counts, labels, targets, bindings, owner, pending, seen, position);
                    match source_ml_source1_s9.clone() {
                        self::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result::MlTemporaryReadRecordTypeOwnerPresenceLoop17Done => {
                            false
                        }
                        self::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result::MlTemporaryReadRecordTypeOwnerPresenceLoop17Return(source_ml_temporary_result17) => {
                            source_ml_temporary_result17
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_literal_property_index(key: &[f64]) -> f64 {
    if ((((key.len() as f64)) == (0f64)) || (((key.len() as f64)) > (10f64))) {
        (-1f64)
    } else {
        if ((((key.len() as f64)) > (1f64)) && {
            let source_ml_source1_comparison_operand10 = self::ml_array::optional_at(key, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand11 = 48f64;
                match (source_ml_source1_comparison_operand10.clone(), source_ml_source1_comparison_operand11) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) {
            (-1f64)
        } else {
            {
                let value = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s12 = self::source_ml_temporary_read_java_script_literal_property_index_loop22(key, value, at);
                        match source_ml_source1_s12.clone() {
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Done(value_2) => {
                                if (value_2 <= (4294967294f64)) {
                                    value_2
                                } else {
                                    (-1f64)
                                }
                            }
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Return(source_ml_temporary_result22) => {
                                source_ml_temporary_result22
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
pub fn read_java_script_literal_property_form(kind: &self::ml_string::Text, index: f64) -> self::ml_string::Text {
    if (((((*kind) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) || ((*kind) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])))) || ((*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) && (index >= (0f64))) {
        self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])
    } else {
        self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_index_number_form(kind: &self::ml_string::Text, element: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
        self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
    } else {
        if (((*kind) == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || ((*kind) == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
            self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])
        } else {
            if (((*kind) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) || (((*kind) == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && ((*element) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))))) {
                self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 116])
            } else {
                if (((*kind) == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && ((((*element) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) || ((*element) == (self::ml_string::Text::from_units(vec![105, 110, 116])))) || ((*element) == (self::ml_string::Text::from_units(vec![110, 97, 116]))))) {
                    self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108])
                } else {
                    self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_record_order_name() -> Vec<f64> {
    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 110, 101, 101, 100, 115, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 32, 110, 97, 109, 101])).0, message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_record_order_position() -> f64 {
    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 110, 101, 101, 100, 115, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 102, 105, 101, 108, 100, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_name(names: &[Vec<f64>], index: f64) -> Vec<f64> {
    (self::ml_array::optional_at(names, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_java_script_record_order_name())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_position(positions: &[f64], index: f64) -> f64 {
    (self::ml_array::optional_at(positions, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_java_script_record_order_position())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_record_order_names(names: &[Vec<f64>]) -> bool {
    {
        let index = 0f64;
        {
            let source_ml_source1_s13 = self::source_ml_temporary_accept_java_script_record_order_names_loop23(names, index);
            match source_ml_source1_s13.clone() {
                self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Done => {
                    true
                }
                self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Return(source_ml_temporary_result23) => {
                    source_ml_temporary_result23
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_record_order_permutation(count: f64, order: &[f64]) -> bool {
    ((((order.len() as f64)) == count) && self::accept_java_script_record_order_positions(count, order))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_record_order_positions(count: f64, order: &[f64]) -> bool {
    if (((!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) || ((count == (0f64)) && (((1f64 / count)) < (0f64)))) || (((order.len() as f64)) > count)) {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s14 = self::source_ml_temporary_accept_java_script_record_order_positions_loop24(count, order, index);
                match source_ml_source1_s14.clone() {
                    self::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Done => {
                        true
                    }
                    self::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Return(source_ml_temporary_result24) => {
                        source_ml_temporary_result24
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_creation_error(names: &[Vec<f64>], insertion: &[f64]) -> self::ml_string::Text {
    if (!(self::accept_java_script_record_order_names(names)) || !(self::accept_java_script_record_order_permutation((names.len() as f64), insertion))) {
        self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 115, 101, 113, 117, 101, 110, 99, 101])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_record_property_order(names: &[Vec<f64>], insertion: &[f64]) -> Vec<f64> {
    {
        let error = self::read_java_script_record_order_creation_error(names, insertion);
        if !error.0.is_empty() {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (error.clone()).0, message_present: true }).raise()
        } else {
            self::sort_java_script_record_property_positions(names, insertion)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn sort_java_script_record_property_positions(names: &[Vec<f64>], insertion: &[f64]) -> Vec<f64> {
    {
        let numeric = Vec::<f64>::new();
        {
            let index = 0f64;
            {
                let numeric_2 = self::source_ml_temporary_sort_java_script_record_property_positions_loop26(names, insertion, numeric, index);
                {
                    let result = numeric_2;
                    {
                        let index_2 = 0f64;
                        {
                            let result_2 = self::source_ml_temporary_sort_java_script_record_property_positions_loop28(names, insertion, result, index_2);
                            result_2
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_present_record_property_order(names: &[Vec<f64>], insertion: &[f64], present: &[bool]) -> Vec<f64> {
    if ((!(self::accept_java_script_record_order_names(names)) || (((present.len() as f64)) != ((names.len() as f64)))) || !(self::accept_java_script_record_order_positions((names.len() as f64), insertion))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 115, 101, 113, 117, 101, 110, 99, 101])).0, message_present: true }).raise()
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s15 = self::source_ml_temporary_create_java_script_present_record_property_order_loop29(insertion, present, index);
                match source_ml_source1_s15.clone() {
                    self::MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Result::MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Done => {
                        self::sort_java_script_record_property_positions(names, insertion)
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_present_record_property_order(names: &[Vec<f64>], order: &[f64]) -> bool {
    if (!(self::accept_java_script_record_order_names(names)) || !(self::accept_java_script_record_order_positions((names.len() as f64), order))) {
        false
    } else {
        {
            let expected = self::sort_java_script_record_property_positions(names, order);
            {
                let index = 0f64;
                {
                    let source_ml_source1_s16 = self::source_ml_temporary_accept_java_script_present_record_property_order_loop30(order, &(expected), index);
                    match source_ml_source1_s16.clone() {
                        self::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Result::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Done => {
                            true
                        }
                        self::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Result::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Return(source_ml_temporary_result30) => {
                            source_ml_temporary_result30
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_record_property_order(names: &[Vec<f64>], order: &[f64]) -> bool {
    if (!(self::accept_java_script_record_order_names(names)) || !(self::accept_java_script_record_order_permutation((names.len() as f64), order))) {
        false
