// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
pub fn source_ml_temporary_read_record_sorted_field_positions_loop9(
    mut fields: Vec<Vec<f64>>,
    mut sorted: Vec<f64>,
    mut width: f64,
    mut merged: Vec<f64>,
    mut start: f64,
) -> Vec<f64> {
    loop {
        return if (start < (sorted.len() as f64)) {
            {
                let middle = if ((start + width) < (sorted.len() as f64)) {
                    (start + width)
                } else {
                    (sorted.len() as f64)
                };
                {
                    let end = if ((middle + width) < (sorted.len() as f64)) {
                        (middle + width)
                    } else {
                        (sorted.len() as f64)
                    };
                    {
                        let left = start;
                        {
                            let right = middle;
                            {
                                let merged_2 = self::source_ml_source3_record_order::source_ml_temporary_read_record_sorted_field_positions_loop10(fields.clone(), sorted.clone(), merged, middle, end, left, right);
                                {
                                    let start_2 = (start + (width * 2f64));
                                    {
                                        (merged, start) = (merged_2, start_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            merged
        };
    }
}

pub fn source_ml_temporary_read_record_sorted_field_positions_loop8(
    mut fields: Vec<Vec<f64>>,
    mut sorted: Vec<f64>,
    mut width: f64,
) -> Vec<f64> {
    loop {
        return if (width < (sorted.len() as f64)) {
            {
                let merged = Vec::<f64>::new();
                {
                    let start = 0f64;
                    {
                        let merged_2 = self::source_ml_source3_record_order::source_ml_temporary_read_record_sorted_field_positions_loop9(fields.clone(), sorted, width, merged, start);
                        {
                            let sorted_2 = merged_2;
                            {
                                let width_2 = (width * 2f64);
                                {
                                    (sorted, width) = (sorted_2, width_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            sorted
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRecordSortedFieldPositionsLoop7Result {
    MlTemporaryReadRecordSortedFieldPositionsLoop7Done,
    MlTemporaryReadRecordSortedFieldPositionsLoop7Return(Vec<f64>),
}

pub fn source_ml_temporary_read_record_sorted_field_positions_loop7(
    mut fields: Vec<Vec<f64>>,
    mut index: f64,
) -> self::source_ml_source3_record_order::MlTemporaryReadRecordSortedFieldPositionsLoop7Result {
    loop {
        return if (index < (fields.len() as f64)) {
            if !(self::source_ml_source3_record_order::accept_record_field_name(
                self::source_ml_source3_record_order::read_required_record_name(
                    fields.clone(),
                    index,
                ),
            )) {
                self::source_ml_source3_record_order::MlTemporaryReadRecordSortedFieldPositionsLoop7Result::MlTemporaryReadRecordSortedFieldPositionsLoop7Return(vec![(-1f64)])
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
            self::source_ml_source3_record_order::MlTemporaryReadRecordSortedFieldPositionsLoop7Result::MlTemporaryReadRecordSortedFieldPositionsLoop7Done
        };
    }
}

pub fn source_ml_temporary_read_record_field_order_loop11(
    mut fields: Vec<Vec<f64>>,
    mut sorted: Vec<f64>,
    mut selected: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        return if (index < (sorted.len() as f64)) {
            if (((index + 1f64) >= (sorted.len() as f64))
                || ((self::source_ml_source3_record_order::compare_record_field_names(
                    self::source_ml_source3_record_order::read_required_record_name(
                        fields.clone(),
                        self::source_ml_source3_record_order::read_required_number_fact(
                            sorted.clone(),
                            index,
                        ),
                    ),
                    self::source_ml_source3_record_order::read_required_record_name(
                        fields.clone(),
                        self::source_ml_source3_record_order::read_required_number_fact(
                            sorted.clone(),
                            (index + 1f64),
                        ),
                    ),
                )) < (0f64)))
            {
                {
                    let selected_2 = {
                        let (ml_append_right, ml_append_left) = (
                            vec![
                                self::source_ml_source3_record_order::read_required_number_fact(
                                    sorted.clone(),
                                    index,
                                ),
                            ],
                            selected,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let index_2 = (index + 1f64);
                        {
                            (selected, index) = (selected_2, index_2);
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
            selected
        };
    }
}

pub fn source_ml_temporary_read_record_field_insertion_order_loop14(
    mut fields: Vec<Vec<f64>>,
    mut first: Vec<f64>,
    mut last: Vec<f64>,
    mut selected: Vec<f64>,
    mut index: f64,
    mut lower: f64,
    mut upper: f64,
) -> Vec<f64> {
    loop {
        return if (lower < upper) {
            {
                let middle = ((lower + upper) / 2f64).floor();
                {
                    let comparison =
                        self::source_ml_source3_record_order::compare_record_field_names(
                            self::source_ml_source3_record_order::read_required_record_name(
                                fields.clone(),
                                index,
                            ),
                            self::source_ml_source3_record_order::read_required_record_name(
                                fields.clone(),
                                self::source_ml_source3_record_order::read_required_number_fact(
                                    last.clone(),
                                    middle,
                                ),
                            ),
                        );
                    if (comparison < (0f64)) {
                        {
                            let upper_2 = middle;
                            {
                                upper = upper_2;
                                continue;
                            }
                        }
                    } else {
                        if (comparison > (0f64)) {
                            {
                                let lower_2 = (middle + 1f64);
                                {
                                    lower = lower_2;
                                    continue;
                                }
                            }
                        } else {
                            {
                                let selected_3 = if (!(((self::source_ml_source3_record_order::read_required_number_fact(first.clone(), middle)) < index)) && !(((self::source_ml_source3_record_order::read_required_number_fact(first.clone(), middle)) > index))) {
                                        {
                                            let selected_2 = { let (ml_append_right, ml_append_left) = (vec![self::source_ml_source3_record_order::read_required_number_fact(last.clone(), middle)], selected); self::ml_array::append(ml_append_left, ml_append_right) };
                                            selected_2
                                        }
                                    } else {
                                        selected
                                    };
                                {
                                    let lower_3 = upper;
                                    {
                                        (selected, lower) = (selected_3, lower_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            selected
        };
    }
}

pub fn source_ml_temporary_read_record_field_insertion_order_loop13(
    mut fields: Vec<Vec<f64>>,
    mut first: Vec<f64>,
    mut last: Vec<f64>,
    mut selected: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        return if (index < (fields.len() as f64)) {
            {
                let lower = 0f64;
                {
                    let upper = (last.len() as f64);
                    {
                        let selected_2 = self::source_ml_source3_record_order::source_ml_temporary_read_record_field_insertion_order_loop14(fields.clone(), first.clone(), last.clone(), selected, index, lower, upper);
                        {
                            let index_2 = (index + 1f64);
                            {
                                (selected, index) = (selected_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            selected
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRecordFieldInsertionOrderLoop12Result {
    MlTemporaryReadRecordFieldInsertionOrderLoop12Done(Vec<f64>, Vec<f64>),
}

pub fn source_ml_temporary_read_record_field_insertion_order_loop12(
    mut fields: Vec<Vec<f64>>,
    mut sorted: Vec<f64>,
    mut first: Vec<f64>,
    mut last: Vec<f64>,
    mut start: f64,
    mut index: f64,
) -> self::source_ml_source3_record_order::MlTemporaryReadRecordFieldInsertionOrderLoop12Result {
    loop {
        return if (index < (sorted.len() as f64)) {
            if (((index + 1f64) >= (sorted.len() as f64))
                || ((self::source_ml_source3_record_order::compare_record_field_names(
                    self::source_ml_source3_record_order::read_required_record_name(
                        fields.clone(),
                        self::source_ml_source3_record_order::read_required_number_fact(
                            sorted.clone(),
                            index,
                        ),
                    ),
                    self::source_ml_source3_record_order::read_required_record_name(
                        fields.clone(),
                        self::source_ml_source3_record_order::read_required_number_fact(
                            sorted.clone(),
                            (index + 1f64),
                        ),
                    ),
                )) < (0f64)))
            {
                {
                    let first_2 = {
                        let (ml_append_right, ml_append_left) = (
                            vec![
                                self::source_ml_source3_record_order::read_required_number_fact(
                                    sorted.clone(),
                                    start,
                                ),
                            ],
                            first,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let last_2 = {
                            let (ml_append_right, ml_append_left) = (
                                vec![
                                    self::source_ml_source3_record_order::read_required_number_fact(
                                        sorted.clone(),
                                        index,
                                    ),
                                ],
                                last,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let start_2 = (index + 1f64);
                            {
                                let index_2 = (index + 1f64);
                                {
                                    (first, last, start, index) =
                                        (first_2, last_2, start_2, index_2);
                                    continue;
                                }
                            }
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
            self::source_ml_source3_record_order::MlTemporaryReadRecordFieldInsertionOrderLoop12Result::MlTemporaryReadRecordFieldInsertionOrderLoop12Done(first, last)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryRecordTypeFactFieldLoop15Result {
    MlTemporaryRecordTypeFactFieldLoop15Done,
    MlTemporaryRecordTypeFactFieldLoop15Return(f64),
}

pub fn source_ml_temporary_record_type_fact_field_loop15(
    mut labels: Vec<self::ml_string::Text>,
    mut targets: Vec<f64>,
    mut name: self::ml_string::Text,
    mut end: f64,
    mut edge: f64,
) -> self::source_ml_source3_record_order::MlTemporaryRecordTypeFactFieldLoop15Result {
    loop {
        return if (edge < end) {
            if ((self::source_ml_source3_record_order::read_required_text_fact(
                labels.clone(),
                edge,
            )) == name)
            {
                self::source_ml_source3_record_order::MlTemporaryRecordTypeFactFieldLoop15Result::MlTemporaryRecordTypeFactFieldLoop15Return(self::source_ml_source3_record_order::read_required_number_fact(targets, edge))
            } else {
                {
                    let edge_2 = (edge + 1f64);
                    {
                        edge = edge_2;
                        continue;
                    }
                }
            }
        } else {
            self::source_ml_source3_record_order::MlTemporaryRecordTypeFactFieldLoop15Result::MlTemporaryRecordTypeFactFieldLoop15Done
        };
    }
}

pub fn source_ml_temporary_record_type_fact_elements_loop16(
    mut targets: Vec<f64>,
    mut values: Vec<f64>,
    mut end: f64,
    mut edge: f64,
) -> Vec<f64> {
    loop {
        return if (edge < end) {
            {
                let values_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            self::source_ml_source3_record_order::read_required_number_fact(
                                targets.clone(),
                                edge,
                            ),
                        ],
                        values,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let edge_2 = (edge + 1f64);
                    {
                        (values, edge) = (values_2, edge_2);
                        continue;
                    }
                }
            }
        } else {
            values
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRecordTypeOwnerPresenceLoop17Result {
    MlTemporaryReadRecordTypeOwnerPresenceLoop17Done,
    MlTemporaryReadRecordTypeOwnerPresenceLoop17Return(bool),
}

pub fn source_ml_temporary_read_record_type_owner_presence_loop19(
    mut seen: Vec<f64>,
    mut node: f64,
    mut visited: bool,
    mut index: f64,
) -> bool {
    loop {
        return if (index < (seen.len() as f64)) {
            if ((self::source_ml_source3_record_order::read_required_number_fact(
                seen.clone(),
                index,
            )) == node)
            {
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

pub fn source_ml_temporary_read_record_type_owner_presence_loop20(
    mut pending: Vec<f64>,
    mut children: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        return if (index < (children.len() as f64)) {
            {
                let pending_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            self::source_ml_source3_record_order::read_required_number_fact(
                                children.clone(),
                                index,
                            ),
                        ],
                        pending,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
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

pub fn source_ml_temporary_read_record_type_owner_presence_loop21(
    mut starts: Vec<f64>,
    mut counts: Vec<f64>,
    mut labels: Vec<self::ml_string::Text>,
    mut targets: Vec<f64>,
    mut pending: Vec<f64>,
    mut fields: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        return if (index < (fields.len() as f64)) {
            {
                let pending_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            self::source_ml_source3_record_order::record_type_fact_field(
                                starts.clone(),
                                counts.clone(),
                                labels.clone(),
                                targets.clone(),
                                self::source_ml_source3_record_order::read_required_number_fact(
                                    fields.clone(),
                                    index,
                                ),
                                self::ml_string::Text::from_units(vec![116, 121, 112, 101]),
                            ),
                        ],
                        pending,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
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

pub fn source_ml_temporary_read_record_type_owner_presence_loop18(
    mut texts: Vec<self::ml_string::Text>,
    mut numbers: Vec<f64>,
    mut starts: Vec<f64>,
    mut counts: Vec<f64>,
    mut labels: Vec<self::ml_string::Text>,
    mut targets: Vec<f64>,
    mut bindings: Vec<f64>,
    mut node: f64,
    mut kind: self::ml_string::Text,
) -> self::source_ml_source3_record_order::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result {
    loop {
        return if (kind == (self::ml_string::Text::from_units(vec![118, 97, 114]))) {
            {
                let id = self::source_ml_source3_record_order::record_type_fact_field(
                    starts.clone(),
                    counts.clone(),
                    labels.clone(),
                    targets.clone(),
                    node,
                    self::ml_string::Text::from_units(vec![105, 100]),
                );
                {
                    let variable = self::source_ml_source3_record_order::read_required_number_fact(
                        numbers.clone(),
                        id,
                    );
                    if ((!(self::ml_math::is_integer(variable)) || (variable < (0f64)))
                        || (variable >= (bindings.len() as f64)))
                    {
                        self::source_ml_source3_record_order::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result::MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(node, kind.clone())
                    } else {
                        {
                            let bound =
                                self::source_ml_source3_record_order::read_required_number_fact(
                                    bindings.clone(),
                                    variable,
                                );
                            {
                                let bound_kind =
                                    self::source_ml_source3_record_order::record_type_fact_kind(
                                        texts.clone(),
                                        starts.clone(),
                                        counts.clone(),
                                        labels.clone(),
                                        targets.clone(),
                                        bound,
                                    );
                                if (bound_kind == (self::ml_string::Text::from_units(vec![]))) {
                                    self::source_ml_source3_record_order::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result::MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(node, kind.clone())
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
            self::source_ml_source3_record_order::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result::MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(node, kind.clone())
        };
    }
}

pub fn source_ml_temporary_read_record_type_owner_presence_loop17(
    mut texts: Vec<self::ml_string::Text>,
    mut numbers: Vec<f64>,
    mut starts: Vec<f64>,
    mut counts: Vec<f64>,
    mut labels: Vec<self::ml_string::Text>,
    mut targets: Vec<f64>,
    mut bindings: Vec<f64>,
    mut owner: f64,
    mut pending: Vec<f64>,
    mut seen: Vec<f64>,
    mut position: f64,
) -> self::source_ml_source3_record_order::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result {
    loop {
        return if (position < (pending.len() as f64)) {
            {
                let node = self::source_ml_source3_record_order::read_required_number_fact(
                    pending.clone(),
                    position,
                );
                {
                    let kind = self::source_ml_source3_record_order::record_type_fact_kind(
                        texts.clone(),
                        starts.clone(),
                        counts.clone(),
                        labels.clone(),
                        targets.clone(),
                        node,
                    );
                    {
                        let source_ml_source1_s27 = self::source_ml_source3_record_order::source_ml_temporary_read_record_type_owner_presence_loop18(texts.clone(), numbers.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), bindings.clone(), node, kind.clone());
                        match source_ml_source1_s27.clone() {
                                self::source_ml_source3_record_order::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result::MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(node_2, kind_2) => {
                                    if (node_2 == owner) {
                                        self::source_ml_source3_record_order::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result::MlTemporaryReadRecordTypeOwnerPresenceLoop17Return(true)
                                    } else {
                                        {
                                            let visited = false;
                                            {
                                                let index = 0f64;
                                                {
                                                    let visited_2 = self::source_ml_source3_record_order::source_ml_temporary_read_record_type_owner_presence_loop19(seen.clone(), node_2, visited, index);
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
                                                                    let pending_2 = { let (ml_append_right, ml_append_left) = (vec![self::source_ml_source3_record_order::record_type_fact_field(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node_2, self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116]))], pending); self::ml_array::append(ml_append_left, ml_append_right) };
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
                                                                        let pending_3 = { let (ml_append_right, ml_append_left) = (vec![self::source_ml_source3_record_order::record_type_fact_field(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node_2, self::ml_string::Text::from_units(vec![107, 101, 121]))], pending); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                        {
                                                                            let pending_4 = { let (ml_append_right, ml_append_left) = (vec![self::source_ml_source3_record_order::record_type_fact_field(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node_2, self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))], pending_3); self::ml_array::append(ml_append_left, ml_append_right) };
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
                                                                                let sequence = self::source_ml_source3_record_order::record_type_fact_field(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node_2, field.clone());
                                                                                {
                                                                                    let children = self::source_ml_source3_record_order::record_type_fact_elements(starts.clone(), counts.clone(), targets.clone(), sequence);
                                                                                    {
                                                                                        let index_2 = 0f64;
                                                                                        {
                                                                                            let pending_5 = self::source_ml_source3_record_order::source_ml_temporary_read_record_type_owner_presence_loop20(pending, children, index_2);
                                                                                            if (kind_2 == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
                                                                                                {
                                                                                                    let pending_6 = { let (ml_append_right, ml_append_left) = (vec![self::source_ml_source3_record_order::record_type_fact_field(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node_2, self::ml_string::Text::from_units(vec![114, 101, 116]))], pending_5); self::ml_array::append(ml_append_left, ml_append_right) };
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
                                                                                let sequence_2 = self::source_ml_source3_record_order::record_type_fact_field(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node_2, self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115]));
                                                                                {
                                                                                    let fields = self::source_ml_source3_record_order::record_type_fact_elements(starts.clone(), counts.clone(), targets.clone(), sequence_2);
                                                                                    {
                                                                                        let index_3 = 0f64;
                                                                                        {
                                                                                            let pending_7 = self::source_ml_source3_record_order::source_ml_temporary_read_record_type_owner_presence_loop21(starts.clone(), counts.clone(), labels.clone(), targets.clone(), pending, fields, index_3);
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
            self::source_ml_source3_record_order::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result::MlTemporaryReadRecordTypeOwnerPresenceLoop17Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result {
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Done(f64),
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Return(f64),
}

pub fn source_ml_temporary_read_java_script_literal_property_index_loop22(
    mut key: Vec<f64>,
    mut value: f64,
    mut at: f64,
) -> self::source_ml_source3_record_order::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result
{
    loop {
        return if (at < (key.len() as f64)) {
            {
                let digit =
                    (match self::ml_array::optional_at(&key, self::ml_array::number_index(at)) {
                        Some(ml_present) => ml_present,
                        None => f64::NAN,
                    } - 48f64);
                if ((!(self::ml_math::is_integer(digit)) || (digit < (0f64))) || (digit > (9f64))) {
                    self::source_ml_source3_record_order::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Return((-1f64))
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
            self::source_ml_source3_record_order::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result {
    MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Done,
    MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Return(bool),
}

pub fn source_ml_temporary_accept_java_script_record_order_names_loop23(
    mut names: Vec<Vec<f64>>,
    mut index: f64,
) -> self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result {
    loop {
        return if (index < (names.len() as f64)) {
            {
                let name = self::source_ml_source3_record_order::read_java_script_record_order_name(
                    names.clone(),
                    index,
                );
                if !(self::source_ml_source3_record_order::accept_record_field_name(name.clone())) {
                    self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Return(false)
                } else {
                    if ((index > (0f64)) && ((self::source_ml_source3_record_order::compare_record_field_names(self::source_ml_source3_record_order::read_java_script_record_order_name(names.clone(), (index - 1f64)), name)) >= (0f64))) {
                            self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Return(false)
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
            self::source_ml_source3_record_order::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Result {
    MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Done,
    MlTemporaryAcceptJavaScriptRecordOrderPositionsLoop24Return(bool),
}
