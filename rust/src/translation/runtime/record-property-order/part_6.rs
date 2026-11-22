
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRecordTypeOwnerPresenceLoop17Result {
    MlTemporaryReadRecordTypeOwnerPresenceLoop17Done,
    MlTemporaryReadRecordTypeOwnerPresenceLoop17Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_type_owner_presence_loop19(seen: &[f64], mut node: f64, mut visited: bool, mut index: f64) -> bool {
    loop {
        return if (index < ((seen.len() as f64))) {
            if ((self::read_required_number_fact(seen, index)) == node) {
                {
                    let visited_2 = true;
                    {
                        let index_2 = (index + 1f64);
                        {
                            (visited, index) = (visited_2, index_2);
                            continue;
                        }
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
            visited
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_type_owner_presence_loop20(mut pending: Vec<f64>, children: &[f64], mut index: f64) -> Vec<f64> {
    loop {
        return if (index < ((children.len() as f64))) {
            {
                let pending_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_number_fact(children, index)], pending); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = (index + 1f64);
                    {
                        (pending, index) = (pending_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            pending
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_type_owner_presence_loop21(starts: &[f64], counts: &[f64], labels: &[self::ml_string::Text], targets: &[f64], mut pending: Vec<f64>, fields: &[f64], mut index: f64) -> Vec<f64> {
    loop {
        return if (index < ((fields.len() as f64))) {
            {
                let pending_2 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, self::read_required_number_fact(fields, index), &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])))], pending); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = (index + 1f64);
                    {
                        (pending, index) = (pending_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            pending
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRecordTypeOwnerPresenceLoop18Result {
    MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(f64, self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_type_owner_presence_loop18(texts: &[self::ml_string::Text], numbers: &[f64], starts: &[f64], counts: &[f64], labels: &[self::ml_string::Text], targets: &[f64], bindings: &[f64], mut node: f64, mut kind: self::ml_string::Text) -> self::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result {
    loop {
        return if (kind == (self::ml_string::Text::from_units(vec![118, 97, 114]))) {
            {
                let id = self::record_type_fact_field(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![105, 100])));
                {
                    let variable = self::read_required_number_fact(numbers, id);
                    if ((!(self::ml_math::is_integer(variable)) || (variable < (0f64))) || (variable >= ((bindings.len() as f64)))) {
                        self::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result::MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(node, kind.clone())
                    } else {
                        {
                            let bound = self::read_required_number_fact(bindings, variable);
                            {
                                let bound_kind = self::record_type_fact_kind(texts, starts, counts, labels, targets, bound);
                                if (bound_kind == (self::ml_string::Text::from_units(vec![]))) {
                                    self::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result::MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(node, kind.clone())
                                } else {
                                    {
                                        let node_2 = bound;
                                        {
                                            let kind_2 = bound_kind.clone();
                                            {
                                                (node, kind) = (node_2, kind_2.clone());
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
            self::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result::MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(node, kind.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_type_owner_presence_loop17(texts: &[self::ml_string::Text], numbers: &[f64], starts: &[f64], counts: &[f64], labels: &[self::ml_string::Text], targets: &[f64], bindings: &[f64], mut owner: f64, mut pending: Vec<f64>, mut seen: Vec<f64>, mut position: f64) -> self::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result {
    loop {
        return if (position < ((pending.len() as f64))) {
            {
                let node = self::read_required_number_fact(&(pending), position);
                {
                    let kind = self::record_type_fact_kind(texts, starts, counts, labels, targets, node);
                    {
                        let source_ml_source1_s27 = self::source_ml_temporary_read_record_type_owner_presence_loop18(texts, numbers, starts, counts, labels, targets, bindings, node, kind.clone());
                        match source_ml_source1_s27.clone() {
                            self::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result::MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(node_2, kind_2) => {
                                if (node_2 == owner) {
                                    self::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result::MlTemporaryReadRecordTypeOwnerPresenceLoop17Return(true)
                                } else {
                                    {
                                        let visited = false;
                                        {
                                            let index = 0f64;
                                            {
                                                let visited_2 = self::source_ml_temporary_read_record_type_owner_presence_loop19(&(seen), node_2, visited, index);
                                                if visited_2 {
                                                    {
                                                        let position_2 = (position + 1f64);
                                                        {
                                                            position = position_2;
                                                            continue;
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let seen_2 = { let (ml_append_right, ml_append_left) = (vec![node_2], seen); self::ml_array::append(ml_append_left, ml_append_right) };
                                                        if ((((kind_2 == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) || (kind_2 == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))) || (kind_2 == (self::ml_string::Text::from_units(vec![115, 101, 116])))) || (kind_2 == (self::ml_string::Text::from_units(vec![99, 101, 108, 108])))) {
                                                            {
                                                                let pending_2 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, &(self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])))], pending); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    let position_3 = (position + 1f64);
                                                                    {
                                                                        (pending, seen, position) = (pending_2, seen_2, position_3);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if (kind_2 == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                                                                {
                                                                    let pending_3 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, &(self::ml_string::Text::from_units(vec![107, 101, 121])))], pending); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                    {
                                                                        let pending_4 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))], pending_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                        {
                                                                            let position_4 = (position + 1f64);
                                                                            {
                                                                                (pending, seen, position) = (pending_4, seen_2, position_4);
                                                                                continue;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if (((kind_2 == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))) || (kind_2 == (self::ml_string::Text::from_units(vec![117, 110, 105, 111, 110])))) || (kind_2 == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))) {
                                                                    {
                                                                        let field = if (kind_2 == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))) {
                                                                            self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 115])
                                                                        } else {
                                                                            if (kind_2 == (self::ml_string::Text::from_units(vec![117, 110, 105, 111, 110]))) {
                                                                                self::ml_string::Text::from_units(vec![97, 108, 116, 101, 114, 110, 97, 116, 105, 118, 101, 115])
                                                                            } else {
                                                                                self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 115])
                                                                            }
                                                                        };
                                                                        {
                                                                            let sequence = self::record_type_fact_field(starts, counts, labels, targets, node_2, &(field));
                                                                            {
                                                                                let children = self::record_type_fact_elements(starts, counts, targets, sequence);
                                                                                {
                                                                                    let index_2 = 0f64;
                                                                                    {
                                                                                        let pending_5 = self::source_ml_temporary_read_record_type_owner_presence_loop20(pending, &(children), index_2);
                                                                                        if (kind_2 == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
                                                                                            {
                                                                                                let pending_6 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, &(self::ml_string::Text::from_units(vec![114, 101, 116])))], pending_5); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                {
                                                                                                    let position_5 = (position + 1f64);
                                                                                                    {
                                                                                                        (pending, seen, position) = (pending_6, seen_2, position_5);
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            {
                                                                                                let position_6 = (position + 1f64);
                                                                                                {
                                                                                                    (pending, seen, position) = (pending_5, seen_2, position_6);
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
                                                                    if (kind_2 == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))) {
                                                                        {
                                                                            let sequence_2 = self::record_type_fact_field(starts, counts, labels, targets, node_2, &(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])));
                                                                            {
                                                                                let fields = self::record_type_fact_elements(starts, counts, targets, sequence_2);
                                                                                {
                                                                                    let index_3 = 0f64;
                                                                                    {
                                                                                        let pending_7 = self::source_ml_temporary_read_record_type_owner_presence_loop21(starts, counts, labels, targets, pending, &(fields), index_3);
                                                                                        {
                                                                                            let position_7 = (position + 1f64);
                                                                                            {
                                                                                                (pending, seen, position) = (pending_7, seen_2, position_7);
                                                                                                continue;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let position_8 = (position + 1f64);
                                                                            {
                                                                                (seen, position) = (seen_2, position_8);
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
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result::MlTemporaryReadRecordTypeOwnerPresenceLoop17Done
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
pub fn source_ml_temporary_read_java_script_literal_property_index_loop22(key: &[f64], mut value: f64, mut at: f64) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result {
    loop {
        return if (at < ((key.len() as f64))) {
            {
                let digit = (match self::ml_array::optional_at(key, self::ml_array::number_index(at)) { Some(ml_present) => ml_present, None => f64::NAN } - 48f64);
                if ((!(self::ml_math::is_integer(digit)) || (digit < (0f64))) || (digit > (9f64))) {
                    self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Return((-1f64))
                } else {
                    {
                        let value_2 = ((value * 10f64) + digit);
                        {
                            let at_2 = (at + 1f64);
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result {
    MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Done,
    MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_record_order_names_loop23(names: &[Vec<f64>], mut index: f64) -> self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result {
    loop {
        return if (index < ((names.len() as f64))) {
            {
                let name = self::read_java_script_record_order_name(names, index);
                if !(self::accept_record_field_name(&(name))) {
                    self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Return(false)
                } else {
                    if ((index > (0f64)) && ((self::compare_record_field_names(&(self::read_java_script_record_order_name(names, (index - 1f64))), &(name))) >= (0f64))) {
                        self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Return(false)
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
            }
        } else {
            self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Result {
    MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Done,
    MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Result {
    MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Done,
    MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_record_order_positions_loop25(order: &[f64], mut index: f64, mut value: f64, mut previous: f64) -> self::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Result {
    loop {
        return if (previous < index) {
            if ((self::read_java_script_record_order_position(order, previous)) == value) {
                self::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Result::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Return(false)
            } else {
                {
                    let previous_2 = (previous + 1f64);
                    {
                        previous = previous_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Result::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_record_order_positions_loop24(mut count: f64, order: &[f64], mut index: f64) -> self::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Result {
    loop {
        return if (index < ((order.len() as f64))) {
            {
                let value = self::read_java_script_record_order_position(order, index);
                if (((!(self::ml_math::is_integer(value)) || (value < (0f64))) || (value >= count)) || ((value == (0f64)) && (((1f64 / value)) < (0f64)))) {
                    self::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Return(false)
                } else {
                    {
                        let previous = 0f64;
                        {
                            let source_ml_source1_s28 = self::source_ml_temporary_accept_java_script_record_order_positions_loop25(order, index, value, previous);
                            match source_ml_source1_s28.clone() {
                                self::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Result::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Done => {
                                    {
                                        let index_2 = (index + 1f64);
                                        {
                                            index = index_2;
                                            continue;
                                        }
                                    }
                                }
                                self::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Result::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop25Return(source_ml_temporary_result25) => {
                                    self::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Return(source_ml_temporary_result25)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_sort_java_script_record_property_positions_loop27(names: &[Vec<f64>], numeric: &[f64], mut key: f64, mut position: f64) -> f64 {
    loop {
        return if ((position < ((numeric.len() as f64))) && ((self::read_java_script_literal_property_index(&(self::read_java_script_record_order_name(names, self::read_java_script_record_order_position(numeric, position))))) < key)) {
            {
                let position_2 = (position + 1f64);
                {
                    position = position_2;
                    continue;
                }
            }
        } else {
            position
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_sort_java_script_record_property_positions_loop26(names: &[Vec<f64>], insertion: &[f64], mut numeric: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < ((insertion.len() as f64))) {
            {
                let field = self::read_java_script_record_order_position(insertion, index);
                {
                    let key = self::read_java_script_literal_property_index(&(self::read_java_script_record_order_name(names, field)));
                    if (key < (0f64)) {
                        {
                            let index_2 = (index + 1f64);
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            let position = 0f64;
                            {
                                let position_2 = self::source_ml_temporary_sort_java_script_record_property_positions_loop27(names, &(numeric), key, position);
                                {
                                    let numeric_2 = {
                                        let source_ml_source1_concat_value29 = self::ml_array::slice(&numeric, 0f64, position_2);
                                        {
                                            let source_ml_source1_concat_value30 = vec![field];
                                            {
                                                let source_ml_source1_concat_value31 = self::ml_array::slice(&numeric, position_2, f64::INFINITY);
                                                self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value29), source_ml_source1_concat_value30), source_ml_source1_concat_value31)
                                            }
                                        }
                                    };
                                    {
                                        let index_3 = (index + 1f64);
                                        {
                                            (numeric, index) = (numeric_2, index_3);
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
            numeric
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_sort_java_script_record_property_positions_loop28(names: &[Vec<f64>], insertion: &[f64], mut result: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < ((insertion.len() as f64))) {
            {
                let field = self::read_java_script_record_order_position(insertion, index);
                if ((self::read_java_script_literal_property_index(&(self::read_java_script_record_order_name(names, field)))) < (0f64)) {
                    {
                        let result_2 = {
                            let source_ml_source1_concat_value32 = result;
                            {
                                let source_ml_source1_concat_value33 = vec![field];
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value32), source_ml_source1_concat_value33)
                            }
                        };
                        {
                            let index_2 = (index + 1f64);
                            {
                                (result, index) = (result_2, index_2);
                                continue;
                            }
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
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Result {
    MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_create_java_script_present_record_property_order_loop29(insertion: &[f64], present: &[bool], mut index: f64) -> self::MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Result {
    loop {
        return if (index < ((present.len() as f64))) {
            if {
                let source_ml_source1_comparison_operand34 = self::ml_array::optional_at(present, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand35 = self::ml_array::includes(insertion, &index, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()));
                    !(match (source_ml_source1_comparison_operand34.clone(), source_ml_source1_comparison_operand35) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            } {
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 115, 101, 113, 117, 101, 110, 99, 101])).0, message_present: true }).raise()
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Result::MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Result {
    MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Done,
    MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_present_record_property_order_loop30(order: &[f64], expected: &[f64], mut index: f64) -> self::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Result {
    loop {
        return if (index < ((order.len() as f64))) {
            if ((self::read_java_script_record_order_position(order, index)) != (self::read_java_script_record_order_position(expected, index))) {
                self::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Result::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Result::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Result {
    MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Done,
    MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_record_property_order_loop31(order: &[f64], expected: &[f64], mut index: f64) -> self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Result {
    loop {
        return if (index < ((order.len() as f64))) {
            if ((self::read_java_script_record_order_position(order, index)) != (self::read_java_script_record_order_position(expected, index))) {
                self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Result {
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Done(Vec<f64>),
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result {
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Done(Vec<f64>),
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_order_merge_error_loop33(names: &[Vec<f64>], mut insertion: Vec<f64>, values: &[f64], mut index: f64) -> self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result {
    loop {
        return if (index < ((values.len() as f64))) {
            {
                let field = self::read_java_script_record_order_position(values, index);
                if (((!(self::ml_math::is_integer(field)) || (field < (0f64))) || (field >= ((names.len() as f64)))) || ((field == (0f64)) && (((1f64 / field)) < (0f64)))) {
                    self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Return(self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 111, 117, 114, 99, 101, 32, 102, 105, 101, 108, 100]))
                } else {
                    if !(self::ml_array::includes(&insertion, &field, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                        {
                            let insertion_2 = {
                                let source_ml_source1_concat_value36 = insertion;
                                {
                                    let source_ml_source1_concat_value37 = vec![field];
                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value36), source_ml_source1_concat_value37)
                                }
                            };
                            {
                                let index_2 = (index + 1f64);
                                {
                                    (insertion, index) = (insertion_2, index_2);
                                    continue;
                                }
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
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Done(insertion)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_order_merge_error_loop32(names: &[Vec<f64>], chunks: &[Vec<f64>], mut insertion: Vec<f64>, mut chunk: f64) -> self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Result {
    loop {
        return if (chunk < ((chunks.len() as f64))) {
            {
                let values = self::read_java_script_record_order_name(chunks, chunk);
                {
                    let index = 0f64;
                    {
                        let source_ml_source1_s38 = self::source_ml_temporary_read_java_script_record_order_merge_error_loop33(names, insertion, &(values), index);
                        match source_ml_source1_s38.clone() {
                            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Done(insertion_2) => {
                                {
                                    let chunk_2 = (chunk + 1f64);
                                    {
                                        (insertion, chunk) = (insertion_2, chunk_2);
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Return(source_ml_temporary_result33) => {
                                self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Return(source_ml_temporary_result33.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Done(insertion)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_merge_java_script_record_property_order_loop35(mut insertion: Vec<f64>, values: &[f64], mut index: f64) -> Vec<f64> {
    loop {
        return if (index < ((values.len() as f64))) {
            {
                let field = self::read_java_script_record_order_position(values, index);
                if !(self::ml_array::includes(&insertion, &field, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                    {
                        let insertion_2 = {
                            let source_ml_source1_concat_value39 = insertion;
                            {
                                let source_ml_source1_concat_value40 = vec![field];
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value39), source_ml_source1_concat_value40)
                            }
                        };
                        {
                            let index_2 = (index + 1f64);
                            {
                                (insertion, index) = (insertion_2, index_2);
                                continue;
                            }
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
            }
        } else {
            insertion
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_merge_java_script_record_property_order_loop34(chunks: &[Vec<f64>], mut insertion: Vec<f64>, mut chunk: f64) -> Vec<f64> {
    loop {
        return if (chunk < ((chunks.len() as f64))) {
            {
                let values = self::read_java_script_record_order_name(chunks, chunk);
                {
                    let index = 0f64;
                    {
                        let insertion_2 = self::source_ml_temporary_merge_java_script_record_property_order_loop35(insertion, &(values), index);
                        {
                            let chunk_2 = (chunk + 1f64);
                            {
                                (insertion, chunk) = (insertion_2, chunk_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            insertion
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_merge_java_script_present_record_property_order_loop37(mut insertion: Vec<f64>, positions: &[f64], mut index: f64) -> Vec<f64> {
    loop {
        return if (index < ((positions.len() as f64))) {
            {
                let field = self::read_java_script_record_order_position(positions, index);
                if !(self::ml_array::includes(&insertion, &field, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                    {
                        let insertion_2 = {
                            let source_ml_source1_concat_value41 = insertion;
                            {
                                let source_ml_source1_concat_value42 = vec![field];
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value41), source_ml_source1_concat_value42)
                            }
                        };
                        {
                            let index_2 = (index + 1f64);
                            {
                                (insertion, index) = (insertion_2, index_2);
                                continue;
                            }
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
            }
        } else {
            insertion
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_merge_java_script_present_record_property_order_loop36(names: &[Vec<f64>], chunks: &[Vec<f64>], mut insertion: Vec<f64>, mut chunk: f64) -> Vec<f64> {
    loop {
        return if (chunk < ((chunks.len() as f64))) {
            {
                let positions = self::read_java_script_record_order_name(chunks, chunk);
                if !(self::accept_java_script_record_order_positions((names.len() as f64), &(positions))) {
                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 111, 117, 114, 99, 101, 32, 102, 105, 101, 108, 100])).0, message_present: true }).raise()
                } else {
                    {
                        let index = 0f64;
                        {
                            let insertion_2 = self::source_ml_temporary_merge_java_script_present_record_property_order_loop37(insertion, &(positions), index);
                            {
                                let chunk_2 = (chunk + 1f64);
                                {
                                    (insertion, chunk) = (insertion_2, chunk_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            insertion
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Result {
    MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Done,
    MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_order_rebase_error_loop38(target: &[Vec<f64>], source: &[Vec<f64>], mut index: f64) -> self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Result {
    loop {
        return if (index < ((source.len() as f64))) {
            if ((self::read_record_field_position(target, &(self::read_java_script_record_order_name(source, index)))) < (0f64)) {
                self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 112, 114, 101, 97, 100, 32, 102, 105, 101, 108, 100, 32, 105, 115, 32, 109, 105, 115, 115, 105, 110, 103, 32, 102, 114, 111, 109, 32, 105, 116, 115, 32, 116, 97, 114, 103, 101, 116]))
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Done
        };
    }
}
