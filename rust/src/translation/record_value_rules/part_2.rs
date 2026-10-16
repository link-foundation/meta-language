// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::*;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_record_field_name_loop2(
    name: &[f64],
    mut index: f64,
) -> self::MlAcceptRecordFieldNameLoop2Result {
    loop {
        return if index < (name.len() as f64) {
            {
                let unit = self::ml_array::optional_at(name, self::ml_array::number_index(index));
                if (!(match unit.clone() {
                    Some(ml_present) => self::ml_math::is_integer(ml_present),
                    None => false,
                }) || ((match unit.clone() {
                    Some(ml_present) => ml_present,
                    None => f64::NAN,
                }) < 0f64))
                    || ((match unit.clone() {
                        Some(ml_present) => ml_present,
                        None => f64::NAN,
                    }) > 65_535f64)
                {
                    self::MlAcceptRecordFieldNameLoop2Result::MlAcceptRecordFieldNameLoop2Return(
                        false,
                    )
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlAcceptRecordFieldNameLoop2Result::MlAcceptRecordFieldNameLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlCompareRecordFieldNamesLoop3Result {
    MlCompareRecordFieldNamesLoop3Done,
    MlCompareRecordFieldNamesLoop3Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_compare_record_field_names_loop3(
    left: &[f64],
    right: &[f64],
    mut end: f64,
    mut index: f64,
) -> self::MlCompareRecordFieldNamesLoop3Result {
    loop {
        return if index < end {
            if (self::read_required_number_fact(left, index))
                < (self::read_required_number_fact(right, index))
            {
                self::MlCompareRecordFieldNamesLoop3Result::MlCompareRecordFieldNamesLoop3Return(
                    -1f64,
                )
            } else if (self::read_required_number_fact(left, index))
                > (self::read_required_number_fact(right, index))
            {
                self::MlCompareRecordFieldNamesLoop3Result::MlCompareRecordFieldNamesLoop3Return(
                    1f64,
                )
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlCompareRecordFieldNamesLoop3Result::MlCompareRecordFieldNamesLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadRecordFieldPositionLoop5Result {
    MlReadRecordFieldPositionLoop5Done,
    MlReadRecordFieldPositionLoop5Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_record_field_position_loop5(
    fields: &[Vec<f64>],
    name: &[f64],
    mut index: f64,
) -> self::MlReadRecordFieldPositionLoop5Result {
    loop {
        return if index < (fields.len() as f64) {
            {
                let comparison = self::compare_record_field_names(
                    &(self::read_required_record_name(fields, index)),
                    name,
                );
                if (comparison < 0f64) && !(comparison > 0f64) {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
                {
                    self::MlReadRecordFieldPositionLoop5Result::MlReadRecordFieldPositionLoop5Return(
                        index,
                    )
                }
            }
        } else {
            self::MlReadRecordFieldPositionLoop5Result::MlReadRecordFieldPositionLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadRecordFieldPositionLoop4Result {
    MlReadRecordFieldPositionLoop4Done,
    MlReadRecordFieldPositionLoop4Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_record_field_position_loop4(
    fields: &[Vec<f64>],
    mut index: f64,
) -> self::MlReadRecordFieldPositionLoop4Result {
    loop {
        return if index < (fields.len() as f64) {
            if (self::accept_record_field_name(&(self::read_required_record_name(fields, index)))) {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                self::MlReadRecordFieldPositionLoop4Result::MlReadRecordFieldPositionLoop4Return(
                    -1f64,
                )
            }
        } else {
            self::MlReadRecordFieldPositionLoop4Result::MlReadRecordFieldPositionLoop4Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_record_sorted_field_positions_loop9(
    fields: &[Vec<f64>],
    sorted: &[f64],
    mut merged: Vec<f64>,
    mut middle: f64,
    mut end: f64,
    mut left: f64,
    mut right: f64,
) -> Vec<f64> {
    loop {
        if (left < middle) || (right < end) {
            if (right >= end)
                || ((left < middle)
                    && ((self::compare_record_field_names(
                        &(self::read_required_record_name(
                            fields,
                            self::read_required_number_fact(sorted, left),
                        )),
                        &(self::read_required_record_name(
                            fields,
                            self::read_required_number_fact(sorted, right),
                        )),
                    )) <= 0f64))
            {
                {
                    let merged_2 = {
                        let (ml_append_right, ml_append_left) =
                            (vec![self::read_required_number_fact(sorted, left)], merged);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let left_2 = left + 1f64;
                        {
                            (merged, left) = (merged_2, left_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let merged_3 = {
                        let (ml_append_right, ml_append_left) =
                            (vec![self::read_required_number_fact(sorted, right)], merged);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let right_2 = right + 1f64;
                        {
                            (merged, right) = (merged_3, right_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { merged };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_record_sorted_field_positions_loop8(
    fields: &[Vec<f64>],
    sorted: &[f64],
    mut width: f64,
    mut merged: Vec<f64>,
    mut start: f64,
) -> Vec<f64> {
    loop {
        if start < (sorted.len() as f64) {
            {
                let middle = if (start + width) < (sorted.len() as f64) {
                    start + width
                } else {
                    sorted.len() as f64
                };
                {
                    let end = if (middle + width) < (sorted.len() as f64) {
                        middle + width
                    } else {
                        sorted.len() as f64
                    };
                    {
                        let left = start;
                        {
                            let right = middle;
                            {
                                let merged_2 = self::ml_read_record_sorted_field_positions_loop9(
                                    fields, sorted, merged, middle, end, left, right,
                                );
                                {
                                    let start_2 = start + (width * 2f64);
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
        }
        return { merged };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_record_sorted_field_positions_loop7(
    fields: &[Vec<f64>],
    mut sorted: Vec<f64>,
    mut width: f64,
) -> Vec<f64> {
    loop {
        if width < (sorted.len() as f64) {
            {
                let merged = Vec::<f64>::new();
                {
                    let start = 0f64;
                    {
                        let merged_2 = self::ml_read_record_sorted_field_positions_loop8(
                            fields,
                            &(sorted),
                            width,
                            merged,
                            start,
                        );
                        {
                            let sorted_2 = merged_2;
                            {
                                let width_2 = width * 2f64;
                                {
                                    (sorted, width) = (sorted_2, width_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { sorted };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadRecordSortedFieldPositionsLoop6Result {
    MlReadRecordSortedFieldPositionsLoop6Done,
    MlReadRecordSortedFieldPositionsLoop6Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_record_sorted_field_positions_loop6(
    fields: &[Vec<f64>],
    mut index: f64,
) -> self::MlReadRecordSortedFieldPositionsLoop6Result {
    loop {
        return if index < (fields.len() as f64) {
            if (self::accept_record_field_name(&(self::read_required_record_name(fields, index)))) {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                self::MlReadRecordSortedFieldPositionsLoop6Result::MlReadRecordSortedFieldPositionsLoop6Return(vec![-1f64])
            }
        } else {
            self::MlReadRecordSortedFieldPositionsLoop6Result::MlReadRecordSortedFieldPositionsLoop6Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_record_field_order_loop10(
    fields: &[Vec<f64>],
    sorted: &[f64],
    mut selected: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (sorted.len() as f64) {
            if ((index + 1f64) >= (sorted.len() as f64))
                || ((self::compare_record_field_names(
                    &(self::read_required_record_name(
                        fields,
                        self::read_required_number_fact(sorted, index),
                    )),
                    &(self::read_required_record_name(
                        fields,
                        self::read_required_number_fact(sorted, (index + 1f64)),
                    )),
                )) < 0f64)
            {
                {
                    let selected_2 = {
                        let (ml_append_right, ml_append_left) = (
                            vec![self::read_required_number_fact(sorted, index)],
                            selected,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let index_2 = index + 1f64;
                        {
                            (selected, index) = (selected_2, index_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let index_3 = index + 1f64;
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        }
        return { selected };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_record_field_insertion_order_loop13(
    fields: &[Vec<f64>],
    first: &[f64],
    last: &[f64],
    mut selected: Vec<f64>,
    mut index: f64,
    mut lower: f64,
    mut upper: f64,
) -> Vec<f64> {
    loop {
        if lower < upper {
            {
                let middle = ((lower + upper) / 2f64).floor();
                {
                    let comparison = self::compare_record_field_names(
                        &(self::read_required_record_name(fields, index)),
                        &(self::read_required_record_name(
                            fields,
                            self::read_required_number_fact(last, middle),
                        )),
                    );
                    if comparison < 0f64 {
                        {
                            let upper_2 = middle;
                            {
                                upper = upper_2;
                                continue;
                            }
                        }
                    }
                    {
                        if comparison > 0f64 {
                            {
                                let lower_2 = middle + 1f64;
                                {
                                    lower = lower_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                let selected_3 =
                                    if ((self::read_required_number_fact(first, middle)) < index)
                                        && !((self::read_required_number_fact(first, middle))
                                            > index)
                                    {
                                        selected
                                    } else {
                                        {
                                            {
                                                let (ml_append_right, ml_append_left) = (
                                                    vec![self::read_required_number_fact(
                                                        last, middle,
                                                    )],
                                                    selected,
                                                );
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            }
                                        }
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
        }
        return { selected };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_record_field_insertion_order_loop12(
    fields: &[Vec<f64>],
    first: &[f64],
    last: &[f64],
    mut selected: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (fields.len() as f64) {
            {
                let lower = 0f64;
                {
                    let upper = last.len() as f64;
                    {
                        let selected_2 = self::ml_read_record_field_insertion_order_loop13(
                            fields, first, last, selected, index, lower, upper,
                        );
                        {
                            let index_2 = index + 1f64;
                            {
                                (selected, index) = (selected_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { selected };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadRecordFieldInsertionOrderLoop11Result {
    MlReadRecordFieldInsertionOrderLoop11Done(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_record_field_insertion_order_loop11(
    fields: &[Vec<f64>],
    sorted: &[f64],
    mut first: Vec<f64>,
    mut last: Vec<f64>,
    mut start: f64,
    mut index: f64,
) -> self::MlReadRecordFieldInsertionOrderLoop11Result {
    loop {
        if index < (sorted.len() as f64) {
            if ((index + 1f64) >= (sorted.len() as f64))
                || ((self::compare_record_field_names(
                    &(self::read_required_record_name(
                        fields,
                        self::read_required_number_fact(sorted, index),
                    )),
                    &(self::read_required_record_name(
                        fields,
                        self::read_required_number_fact(sorted, (index + 1f64)),
                    )),
                )) < 0f64)
            {
                {
                    let first_2 = {
                        let (ml_append_right, ml_append_left) =
                            (vec![self::read_required_number_fact(sorted, start)], first);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let last_2 = {
                            let (ml_append_right, ml_append_left) =
                                (vec![self::read_required_number_fact(sorted, index)], last);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let start_2 = index + 1f64;
                            {
                                let index_2 = index + 1f64;
                                {
                                    (first, last, start, index) =
                                        (first_2, last_2, start_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
            {
                {
                    let index_3 = index + 1f64;
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        }
        return {
            self::MlReadRecordFieldInsertionOrderLoop11Result::MlReadRecordFieldInsertionOrderLoop11Done(first, last)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlRecordTypeFactFieldLoop14Result {
    MlRecordTypeFactFieldLoop14Done,
    MlRecordTypeFactFieldLoop14Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_record_type_fact_field_loop14(
    labels: &[String],
    targets: &[f64],
    name: &str,
    mut end: f64,
    mut edge: f64,
) -> self::MlRecordTypeFactFieldLoop14Result {
    loop {
        return if edge < end {
            if (self::read_required_text_fact(labels, edge)) == name {
                self::MlRecordTypeFactFieldLoop14Result::MlRecordTypeFactFieldLoop14Return(
                    self::read_required_number_fact(targets, edge),
                )
            } else {
                {
                    let edge_2 = edge + 1f64;
                    {
                        edge = edge_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlRecordTypeFactFieldLoop14Result::MlRecordTypeFactFieldLoop14Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_record_type_fact_elements_loop15(
    targets: &[f64],
    mut values: Vec<f64>,
    mut end: f64,
    mut edge: f64,
) -> Vec<f64> {
    loop {
        if edge < end {
            {
                let values_2 = {
                    let (ml_append_right, ml_append_left) =
                        (vec![self::read_required_number_fact(targets, edge)], values);
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let edge_2 = edge + 1f64;
                    {
                        (values, edge) = (values_2, edge_2);
                        continue;
                    }
                }
            }
        }
        return { values };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadRecordTypeOwnerPresenceLoop16Result {
    MlReadRecordTypeOwnerPresenceLoop16Done,
    MlReadRecordTypeOwnerPresenceLoop16Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_record_type_owner_presence_loop18(
    seen: &[f64],
    mut node: f64,
    mut visited: bool,
    mut index: f64,
) -> bool {
    loop {
        if index < (seen.len() as f64) {
            if (self::read_required_number_fact(seen, index)) == node {
                {
                    let visited_2 = true;
                    {
                        let index_2 = index + 1f64;
                        {
                            (visited, index) = (visited_2, index_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let index_3 = index + 1f64;
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        }
        return { visited };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_record_type_owner_presence_loop19(
    mut pending: Vec<f64>,
    children: &[f64],
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (children.len() as f64) {
            {
                let pending_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![self::read_required_number_fact(children, index)],
                        pending,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (pending, index) = (pending_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { pending };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_record_type_owner_presence_loop20(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut pending: Vec<f64>,
    fields: &[f64],
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (fields.len() as f64) {
            {
                let pending_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![self::record_type_fact_field(
                            starts,
                            counts,
                            labels,
                            targets,
                            self::read_required_number_fact(fields, index),
                            "type",
                        )],
                        pending,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (pending, index) = (pending_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { pending };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadRecordTypeOwnerPresenceLoop17Result {
    MlReadRecordTypeOwnerPresenceLoop17Done(f64, String),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_record_type_owner_presence_loop17(
    texts: &[String],
    numbers: &[f64],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    bindings: &[f64],
    mut node: f64,
    mut kind: String,
) -> self::MlReadRecordTypeOwnerPresenceLoop17Result {
    loop {
        return if kind == "var" {
            {
                let id = self::record_type_fact_field(starts, counts, labels, targets, node, "id");
                {
                    let variable = self::read_required_number_fact(numbers, id);
                    if (!(self::ml_math::is_integer(variable)) || (variable < 0f64))
                        || (variable >= (bindings.len() as f64))
                    {
                        self::MlReadRecordTypeOwnerPresenceLoop17Result::MlReadRecordTypeOwnerPresenceLoop17Done(node, kind.clone())
                    } else {
                        {
                            let bound = self::read_required_number_fact(bindings, variable);
                            {
                                let bound_kind = self::record_type_fact_kind(
                                    texts, starts, counts, labels, targets, bound,
                                );
                                if bound_kind == "" {
                                    self::MlReadRecordTypeOwnerPresenceLoop17Result::MlReadRecordTypeOwnerPresenceLoop17Done(node, kind.clone())
                                } else {
                                    {
                                        let node_2 = bound;
                                        {
                                            let kind_2 = bound_kind.clone();
                                            {
                                                (node, kind) = (node_2, kind_2);
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
            self::MlReadRecordTypeOwnerPresenceLoop17Result::MlReadRecordTypeOwnerPresenceLoop17Done(
                node,
                kind.clone(),
            )
        };
    }
}
