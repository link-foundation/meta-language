// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlReadRecordTypeOwnerPresenceLoop17Result;
use super::MlReadRecordTypeOwnerPresenceLoop18Result;
use super::ml_array;
use super::ml_read_record_type_owner_presence_loop18;
use super::ml_read_record_type_owner_presence_loop19;
use super::ml_read_record_type_owner_presence_loop20;
use super::ml_read_record_type_owner_presence_loop21;
use super::read_required_number_fact;
use super::record_type_fact_elements;
use super::record_type_fact_field;
use super::record_type_fact_kind;

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
    mut owner: f64,
    mut pending: Vec<f64>,
    mut seen: Vec<f64>,
    mut position: f64,
) -> self::MlReadRecordTypeOwnerPresenceLoop17Result {
    loop {
        return if position < (pending.len() as f64) {
            {
                let node = self::read_required_number_fact(&(pending), position);
                {
                    let kind =
                        self::record_type_fact_kind(texts, starts, counts, labels, targets, node);
                    {
                        let ml_s10 = self::ml_read_record_type_owner_presence_loop18(
                            texts, numbers, starts, counts, labels, targets, bindings, node, kind,
                        );
                        match ml_s10 {
                            self::MlReadRecordTypeOwnerPresenceLoop18Result::MlReadRecordTypeOwnerPresenceLoop18Done(node_2, kind_2) => {
                                if node_2 == owner {
                                    self::MlReadRecordTypeOwnerPresenceLoop17Result::MlReadRecordTypeOwnerPresenceLoop17Return(true)
                                } else {
                                    {
                                        let visited = false;
                                        {
                                            let index = 0f64;
                                            {
                                                let visited_2 = self::ml_read_record_type_owner_presence_loop19(&(seen), node_2, visited, index);
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
                                                        if (((kind_2 == "array") || (kind_2 == "option")) || (kind_2 == "set")) || (kind_2 == "cell") {
                                                            {
                                                                let pending_2 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, "element")], pending); self::ml_array::append(ml_append_left, ml_append_right) };
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
                                                            if kind_2 == "map" {
                                                                {
                                                                    let pending_3 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, "key")], pending); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                    {
                                                                        let pending_4 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, "value")], pending_3); self::ml_array::append(ml_append_left, ml_append_right) };
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
                                                                if ((kind_2 == "tuple") || (kind_2 == "union")) || (kind_2 == "function") {
                                                                    {
                                                                        let field = if kind_2 == "tuple" {
                                                                            String::from("elements")
                                                                        } else if kind_2 == "union" {
                                                                                String::from("alternatives")
                                                                            } else {
                                                                                String::from("params")
                                                                            };
                                                                        {
                                                                            let sequence = self::record_type_fact_field(starts, counts, labels, targets, node_2, &(field));
                                                                            {
                                                                                let children = self::record_type_fact_elements(starts, counts, targets, sequence);
                                                                                {
                                                                                    let index_2 = 0f64;
                                                                                    {
                                                                                        let pending_5 = self::ml_read_record_type_owner_presence_loop20(pending, &(children), index_2);
                                                                                        if kind_2 == "function" {
                                                                                            {
                                                                                                let pending_6 = { let (ml_append_right, ml_append_left) = (vec![self::record_type_fact_field(starts, counts, labels, targets, node_2, "ret")], pending_5); self::ml_array::append(ml_append_left, ml_append_right) };
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
                                                                    if kind_2 == "record" {
                                                                        {
                                                                            let sequence_2 = self::record_type_fact_field(starts, counts, labels, targets, node_2, "fields");
                                                                            {
                                                                                let fields = self::record_type_fact_elements(starts, counts, targets, sequence_2);
                                                                                {
                                                                                    let index_3 = 0f64;
                                                                                    {
                                                                                        let pending_7 = self::ml_read_record_type_owner_presence_loop21(starts, counts, labels, targets, pending, &(fields), index_3);
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
            self::MlReadRecordTypeOwnerPresenceLoop17Result::MlReadRecordTypeOwnerPresenceLoop17Done
        };
    }
}
