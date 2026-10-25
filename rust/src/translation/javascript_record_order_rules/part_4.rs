// Generated from complete js/src/translation/javascript-record-order-rules.js. Do not edit by hand.
use super::MlTemporaryRecordTypeFactFieldLoop15Result;
use super::accept_record_field_name;
use super::compare_record_field_names;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_literal_property_index;
use super::read_java_script_record_order_name;
use super::read_java_script_record_order_position;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::record_type_fact_elements;
use super::record_type_fact_field;
use super::record_type_fact_kind;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_record_type_fact_field_loop15(
    labels: &[self::ml_string::Text],
    targets: &[f64],
    name: &self::ml_string::Text,
    mut end: f64,
    mut edge: f64,
) -> self::MlTemporaryRecordTypeFactFieldLoop15Result {
    loop {
        return if edge < end {
            if (self::read_required_text_fact(labels, edge)) == (*name) {
                self::MlTemporaryRecordTypeFactFieldLoop15Result::MlTemporaryRecordTypeFactFieldLoop15Return(self::read_required_number_fact(targets, edge))
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
            self::MlTemporaryRecordTypeFactFieldLoop15Result::MlTemporaryRecordTypeFactFieldLoop15Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_record_type_fact_elements_loop16(
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
pub enum MlTemporaryReadRecordTypeOwnerPresenceLoop17Result {
    MlTemporaryReadRecordTypeOwnerPresenceLoop17Done,
    MlTemporaryReadRecordTypeOwnerPresenceLoop17Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_type_owner_presence_loop19(
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
pub fn source_ml_temporary_read_record_type_owner_presence_loop20(
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
pub fn source_ml_temporary_read_record_type_owner_presence_loop21(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
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
                            &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])),
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
pub enum MlTemporaryReadRecordTypeOwnerPresenceLoop18Result {
    MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(f64, self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_read_record_type_owner_presence_loop18(
    texts: &[self::ml_string::Text],
    numbers: &[f64],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    bindings: &[f64],
    mut node: f64,
    mut kind: self::ml_string::Text,
) -> self::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result {
    loop {
        return if kind == (self::ml_string::Text::from_units(vec![118, 97, 114])) {
            {
                let id = self::record_type_fact_field(
                    starts,
                    counts,
                    labels,
                    targets,
                    node,
                    &(self::ml_string::Text::from_units(vec![105, 100])),
                );
                {
                    let variable = self::read_required_number_fact(numbers, id);
                    if (!(self::ml_math::is_integer(variable)) || (variable < 0f64))
                        || (variable >= (bindings.len() as f64))
                    {
                        self::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result::MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(node, kind)
                    } else {
                        {
                            let bound = self::read_required_number_fact(bindings, variable);
                            {
                                let bound_kind = self::record_type_fact_kind(
                                    texts, starts, counts, labels, targets, bound,
                                );
                                if bound_kind == (self::ml_string::Text::from_units(vec![])) {
                                    self::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result::MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(node, kind)
                                } else {
                                    {
                                        let node_2 = bound;
                                        {
                                            let kind_2 = bound_kind;
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
            self::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result::MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(node, kind)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_read_record_type_owner_presence_loop17(
    texts: &[self::ml_string::Text],
    numbers: &[f64],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    bindings: &[f64],
    mut owner: f64,
    mut pending: Vec<f64>,
    mut seen: Vec<f64>,
    mut position: f64,
) -> self::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result {
    loop {
        return if position < (pending.len() as f64) {
            {
                let node = self::read_required_number_fact(&(pending), position);
                {
                    let kind =
                        self::record_type_fact_kind(texts, starts, counts, labels, targets, node);
                    {
                        let source_ml_source1_s23 =
                            self::source_ml_temporary_read_record_type_owner_presence_loop18(
                                texts, numbers, starts, counts, labels, targets, bindings, node,
                                kind,
                            );
                        match source_ml_source1_s23 {
                            self::MlTemporaryReadRecordTypeOwnerPresenceLoop18Result::MlTemporaryReadRecordTypeOwnerPresenceLoop18Done(node_2, kind_2) => {
                                if node_2 == owner {
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
                                                        let position_2 = position + 1f64;
                                                        {
                                                            position = position_2;
                                                            continue;
                                                        }
                                                    }
                                                }
{
                                                    {
                                                        let seen_2 = { let (ml_append_right, ml_append_left) = (vec![node_2], seen); self::ml_array::append(ml_append_left, ml_append_right) };
                                                        if (((kind_2 == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) || (kind_2 == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))) || (kind_2 == (self::ml_string::Text::from_units(vec![115, 101, 116])))) || (kind_2 == (self::ml_string::Text::from_units(vec![99, 101, 108, 108]))) {
                                                            {
                                                                let pending_2 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, &(self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])))], pending); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    let position_3 = position + 1f64;
                                                                    {
                                                                        (pending, seen, position) = (pending_2, seen_2, position_3);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
{
                                                            if kind_2 == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
                                                                {
                                                                    let pending_3 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, &(self::ml_string::Text::from_units(vec![107, 101, 121])))], pending); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                    {
                                                                        let pending_4 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))], pending_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                        {
                                                                            let position_4 = position + 1f64;
                                                                            {
                                                                                (pending, seen, position) = (pending_4, seen_2, position_4);
                                                                                continue;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
{
                                                                if ((kind_2 == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))) || (kind_2 == (self::ml_string::Text::from_units(vec![117, 110, 105, 111, 110])))) || (kind_2 == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
                                                                    {
                                                                        let field = if kind_2 == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])) {
                                                                            self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 115])
                                                                        } else if kind_2 == (self::ml_string::Text::from_units(vec![117, 110, 105, 111, 110])) {
                                                                                self::ml_string::Text::from_units(vec![97, 108, 116, 101, 114, 110, 97, 116, 105, 118, 101, 115])
                                                                            } else {
                                                                                self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 115])
                                                                            };
                                                                        {
                                                                            let sequence = self::record_type_fact_field(starts, counts, labels, targets, node_2, &(field));
                                                                            {
                                                                                let children = self::record_type_fact_elements(starts, counts, targets, sequence);
                                                                                {
                                                                                    let index_2 = 0f64;
                                                                                    {
                                                                                        let pending_5 = self::source_ml_temporary_read_record_type_owner_presence_loop20(pending, &(children), index_2);
                                                                                        if kind_2 == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])) {
                                                                                            {
                                                                                                let pending_6 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, &(self::ml_string::Text::from_units(vec![114, 101, 116])))], pending_5); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                {
                                                                                                    let position_5 = position + 1f64;
                                                                                                    {
                                                                                                        (pending, seen, position) = (pending_6, seen_2, position_5);
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
{
                                                                                            {
                                                                                                let position_6 = position + 1f64;
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
                                                                }
{
                                                                    if kind_2 == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])) {
                                                                        {
                                                                            let sequence_2 = self::record_type_fact_field(starts, counts, labels, targets, node_2, &(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])));
                                                                            {
                                                                                let fields = self::record_type_fact_elements(starts, counts, targets, sequence_2);
                                                                                {
                                                                                    let index_3 = 0f64;
                                                                                    {
                                                                                        let pending_7 = self::source_ml_temporary_read_record_type_owner_presence_loop21(starts, counts, labels, targets, pending, &(fields), index_3);
                                                                                        {
                                                                                            let position_7 = position + 1f64;
                                                                                            {
                                                                                                (pending, seen, position) = (pending_7, seen_2, position_7);
                                                                                                continue;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
{
                                                                        {
                                                                            let position_8 = position + 1f64;
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
pub fn source_ml_temporary_read_java_script_literal_property_index_loop22(
    key: &[f64],
    mut value: f64,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result {
    loop {
        return if at < (key.len() as f64) {
            {
                let digit = (self::ml_array::optional_at(key, self::ml_array::number_index(at)))
                    .unwrap_or(f64::NAN)
                    - 48f64;
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result {
    MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Done,
    MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_record_order_names_loop23(
    names: &[Vec<f64>],
    mut index: f64,
) -> self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result {
    loop {
        return if index < (names.len() as f64) {
            {
                let name = self::read_java_script_record_order_name(names, index);
                if (self::accept_record_field_name(&(name))) {
                    if (index > 0f64)
                        && ((self::compare_record_field_names(
                            &(self::read_java_script_record_order_name(names, (index - 1f64))),
                            &(name),
                        )) >= 0f64)
                    {
                        self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Return(false)
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
                    self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Return(false)
                }
            }
        } else {
            self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result {
    MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Done,
    MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Result {
    MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Done,
    MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_record_order_permutation_loop25(
    order: &[f64],
    mut index: f64,
    mut value: f64,
    mut previous: f64,
) -> self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Result {
    loop {
        return if previous < index {
            if (self::read_java_script_record_order_position(order, previous)) == value {
                self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Return(false)
            } else {
                {
                    let previous_2 = previous + 1f64;
                    {
                        previous = previous_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_record_order_permutation_loop24(
    mut count: f64,
    order: &[f64],
    mut index: f64,
) -> self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result {
    loop {
        return if index < (order.len() as f64) {
            {
                let value = self::read_java_script_record_order_position(order, index);
                if ((!(self::ml_math::is_integer(value)) || (value < 0f64)) || (value >= count))
                    || ((value == 0f64) && ((1f64 / value) < 0f64))
                {
                    self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Return(false)
                } else {
                    {
                        let previous = 0f64;
                        {
                            let source_ml_source1_s24 = self::source_ml_temporary_accept_java_script_record_order_permutation_loop25(order, index, value, previous);
                            match source_ml_source1_s24 {
                                self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Done => {
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            index = index_2;
                                            continue;
                                        }
                                    }
                                }
                                self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop25Return(source_ml_temporary_result25) => {
                                    self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Return(source_ml_temporary_result25)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_create_java_script_record_property_order_loop27(
    names: &[Vec<f64>],
    numeric: &[f64],
    mut key: f64,
    mut position: f64,
) -> f64 {
    loop {
        if (position < (numeric.len() as f64))
            && ((self::read_java_script_literal_property_index(
                &(self::read_java_script_record_order_name(
                    names,
                    self::read_java_script_record_order_position(numeric, position),
                )),
            )) < key)
        {
            {
                let position_2 = position + 1f64;
                {
                    position = position_2;
                    continue;
                }
            }
        }
        return { position };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_create_java_script_record_property_order_loop26(
    names: &[Vec<f64>],
    mut numeric: Vec<f64>,
    mut field: f64,
) -> Vec<f64> {
    loop {
        if field < (names.len() as f64) {
            {
                let key = self::read_java_script_literal_property_index(
                    &(self::read_java_script_record_order_name(names, field)),
                );
                if key < 0f64 {
                    {
                        let field_2 = field + 1f64;
                        {
                            field = field_2;
                            continue;
                        }
                    }
                }
                {
                    {
                        let position = 0f64;
                        {
                            let position_2 = self::source_ml_temporary_create_java_script_record_property_order_loop27(names, &(numeric), key, position);
                            {
                                let numeric_2 = {
                                    let source_ml_source1_concat_value25 =
                                        self::ml_array::slice(&numeric, 0f64, position_2);
                                    {
                                        let source_ml_source1_concat_value26 = vec![field];
                                        {
                                            let source_ml_source1_concat_value27 =
                                                self::ml_array::slice(
                                                    &numeric,
                                                    position_2,
                                                    f64::INFINITY,
                                                );
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source1_concat_value25,
                                                    ),
                                                    source_ml_source1_concat_value26,
                                                ),
                                                source_ml_source1_concat_value27,
                                            )
                                        }
                                    }
                                };
                                {
                                    let field_3 = field + 1f64;
                                    {
                                        (numeric, field) = (numeric_2, field_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { numeric };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_create_java_script_record_property_order_loop28(
    names: &[Vec<f64>],
    insertion: &[f64],
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (insertion.len() as f64) {
            {
                let field = self::read_java_script_record_order_position(insertion, index);
                if (self::read_java_script_literal_property_index(
                    &(self::read_java_script_record_order_name(names, field)),
                )) < 0f64
                {
                    {
                        let result_2 = {
                            let source_ml_source1_concat_value28 = result;
                            {
                                let source_ml_source1_concat_value29 = vec![field];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value28,
                                    ),
                                    source_ml_source1_concat_value29,
                                )
                            }
                        };
                        {
                            let index_2 = index + 1f64;
                            {
                                (result, index) = (result_2, index_2);
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
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result {
    MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Done,
    MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_record_property_order_loop29(
    order: &[f64],
    expected: &[f64],
    mut index: f64,
) -> self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result {
    loop {
        return if index < (order.len() as f64) {
            if (self::read_java_script_record_order_position(order, index))
                == (self::read_java_script_record_order_position(expected, index))
            {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Return(false)
            }
        } else {
            self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result {
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Done(Vec<f64>),
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result {
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Done(Vec<f64>),
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_order_merge_error_loop31(
    names: &[Vec<f64>],
    mut insertion: Vec<f64>,
    values: &[f64],
    mut index: f64,
) -> self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result {
    loop {
        return if index < (values.len() as f64) {
            {
                let field = self::read_java_script_record_order_position(values, index);
                if ((!(self::ml_math::is_integer(field)) || (field < 0f64))
                    || (field >= (names.len() as f64)))
                    || ((field == 0f64) && ((1f64 / field) < 0f64))
                {
                    self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Return(self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 111, 117, 114, 99, 101, 32, 102, 105, 101, 108, 100]))
                } else {
                    if !(self::ml_array::includes(
                        &insertion,
                        &field,
                        0f64,
                        false,
                        |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                    )) {
                        {
                            let insertion_2 = {
                                let source_ml_source1_concat_value30 = insertion;
                                {
                                    let source_ml_source1_concat_value31 = vec![field];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source1_concat_value30,
                                        ),
                                        source_ml_source1_concat_value31,
                                    )
                                }
                            };
                            {
                                let index_2 = index + 1f64;
                                {
                                    (insertion, index) = (insertion_2, index_2);
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
            }
        } else {
            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Done(insertion)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_order_merge_error_loop30(
    names: &[Vec<f64>],
    chunks: &[Vec<f64>],
    mut insertion: Vec<f64>,
    mut chunk: f64,
) -> self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result {
    loop {
        return if chunk < (chunks.len() as f64) {
            {
                let values = self::read_java_script_record_order_name(chunks, chunk);
                {
                    let index = 0f64;
                    {
                        let source_ml_source1_s32 = self::source_ml_temporary_read_java_script_record_order_merge_error_loop31(names, insertion, &(values), index);
                        match source_ml_source1_s32 {
                            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Done(insertion_2) => {
                                {
                                    let chunk_2 = chunk + 1f64;
                                    {
                                        (insertion, chunk) = (insertion_2, chunk_2);
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Return(source_ml_temporary_result31) => {
                                self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Return(source_ml_temporary_result31)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Done(insertion)
        };
    }
}
