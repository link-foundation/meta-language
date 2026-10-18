// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::accept_guarded_write_literal;
use super::accept_guarded_write_uses;
use super::ml_array;
use super::read_constructed_guarded_array_append;
use super::read_guarded_write_candidate;
use super::read_guarded_write_element;
use super::read_guarded_write_field;
use super::read_guarded_write_kind;
use super::read_guarded_write_name;
use super::read_guarded_write_text_field;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_guarded_private_array_writes_loop14(
    mut result: Vec<f64>,
    mut allocation: f64,
    candidates: &[f64],
    owners: &[f64],
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (candidates.len() as f64) {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            (self::ml_array::optional_at(owners, self::ml_array::number_index(at)))
                                .unwrap_or_else(|| -1f64),
                            (self::ml_array::optional_at(
                                candidates,
                                self::ml_array::number_index(at),
                            ))
                            .unwrap_or_else(|| -1f64),
                            allocation,
                        ],
                        result,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanGuardedPrivateArrayWritesLoop13Result {
    MlPlanGuardedPrivateArrayWritesLoop13Done(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_plan_guarded_private_array_writes_loop13(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    mut statements: f64,
    mut count: f64,
    owner: &str,
    mut candidates: Vec<f64>,
    mut owners: Vec<f64>,
    mut later: f64,
) -> self::MlPlanGuardedPrivateArrayWritesLoop13Result {
    loop {
        if later < count {
            {
                let candidate =
                    self::read_guarded_write_element(columns, rows, size, statements, later);
                {
                    let write =
                        self::read_guarded_write_candidate(columns, rows, size, candidate, owner);
                    if write >= 0f64 {
                        {
                            let candidates_2 = {
                                let (ml_append_right, ml_append_left) = (vec![write], candidates);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let owners_2 = {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![candidate], owners);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let later_2 = later + 1f64;
                                    {
                                        (candidates, owners, later) =
                                            (candidates_2, owners_2, later_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    {
                        {
                            let later_3 = later + 1f64;
                            {
                                later = later_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlPlanGuardedPrivateArrayWritesLoop13Result::MlPlanGuardedPrivateArrayWritesLoop13Done(candidates, owners)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_guarded_private_array_writes_loop12(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    mut result: Vec<f64>,
    mut statements: f64,
    mut count: f64,
    mut index: f64,
) -> Vec<f64> {
    loop {
        return if index < count {
            {
                let allocation =
                    self::read_guarded_write_element(columns, rows, size, statements, index);
                {
                    let statement =
                        self::read_guarded_write_text_field(columns, rows, size, allocation, "s");
                    if ((statement != "const") && (statement != "let"))
                        || !(self::accept_guarded_write_literal(
                            columns,
                            rows,
                            size,
                            self::read_guarded_write_field(
                                columns, rows, size, allocation, "value",
                            ),
                        ))
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
                        {
                            let owner = self::read_guarded_write_text_field(
                                columns, rows, size, allocation, "name",
                            );
                            if (owner).is_empty() {
                                {
                                    let index_3 = index + 1f64;
                                    {
                                        index = index_3;
                                        continue;
                                    }
                                }
                            }
                            {
                                {
                                    let candidates = Vec::<f64>::new();
                                    {
                                        let owners = Vec::<f64>::new();
                                        {
                                            let later = index + 1f64;
                                            {
                                                let ml_s15 = self::ml_plan_guarded_private_array_writes_loop13(columns, rows, size, statements, count, &(owner), candidates, owners, later);
                                                match ml_s15 {
                                                    self::MlPlanGuardedPrivateArrayWritesLoop13Result::MlPlanGuardedPrivateArrayWritesLoop13Done(candidates_2, owners_2) => {
                                                        if ((candidates_2.len() as f64) == 0f64) || !(self::accept_guarded_write_uses(columns, rows, size, &(owner), allocation, &(candidates_2))) {
                                                            {
                                                                let index_4 = index + 1f64;
                                                                {
                                                                    index = index_4;
                                                                    continue;
                                                                }
                                                            }
                                                        }
{
                                                            {
                                                                let at = 0f64;
                                                                {
                                                                    let result_2 = self::ml_plan_guarded_private_array_writes_loop14(result, allocation, &(candidates_2), &(owners_2), at);
                                                                    {
                                                                        let index_5 = index + 1f64;
                                                                        {
                                                                            (result, index) = (result_2, index_5);
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
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadConstructedGuardedArrayAppendLoop15Result {
    MlReadConstructedGuardedArrayAppendLoop15Done,
    MlReadConstructedGuardedArrayAppendLoop15Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_constructed_guarded_array_append_loop15(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    mut args: f64,
    mut count: f64,
    mut index: f64,
) -> self::MlReadConstructedGuardedArrayAppendLoop15Result {
    loop {
        return if index < count {
            {
                let kind = self::read_guarded_write_kind(
                    columns,
                    rows,
                    size,
                    self::read_guarded_write_element(columns, rows, size, args, index),
                );
                if ((kind != "num") && (kind != "bool")) && (kind != "str") {
                    self::MlReadConstructedGuardedArrayAppendLoop15Result::MlReadConstructedGuardedArrayAppendLoop15Return(-1f64)
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
            self::MlReadConstructedGuardedArrayAppendLoop15Result::MlReadConstructedGuardedArrayAppendLoop15Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptConstructedGuardedArrayUsesLoop17Result {
    MlAcceptConstructedGuardedArrayUsesLoop17Done,
    MlAcceptConstructedGuardedArrayUsesLoop17Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptConstructedGuardedArrayUsesLoop18Result {
    MlAcceptConstructedGuardedArrayUsesLoop18Done(f64),
    MlAcceptConstructedGuardedArrayUsesLoop18Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptConstructedGuardedArrayUsesLoop19Result {
    MlAcceptConstructedGuardedArrayUsesLoop19Done(f64),
    MlAcceptConstructedGuardedArrayUsesLoop19Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_accept_constructed_guarded_array_uses_loop19(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    writes: &[f64],
    appends: &[f64],
    mut node: f64,
    mut uses: f64,
    mut parent: f64,
    mut start: f64,
    mut count: f64,
    mut offset: f64,
) -> self::MlAcceptConstructedGuardedArrayUsesLoop19Result {
    loop {
        return if offset < count {
            {
                let edge = start + offset;
                if ((self::ml_array::optional_at(
                    rows,
                    self::ml_array::number_index((2f64 * size) + edge),
                ))
                .unwrap_or_else(|| -1f64))
                    != node
                {
                    {
                        let offset_2 = offset + 1f64;
                        {
                            offset = offset_2;
                            continue;
                        }
                    }
                }
                {
                    {
                        let uses_2 = uses + 1f64;
                        {
                            let label = (self::ml_array::optional_at(
                                columns,
                                self::ml_array::number_index((2f64 * size) + edge),
                            ))
                            .unwrap_or_default();
                            {
                                let kind =
                                    self::read_guarded_write_kind(columns, rows, size, parent);
                                {
                                    let statement = self::read_guarded_write_text_field(
                                        columns, rows, size, parent, "s",
                                    );
                                    {
                                        let read = (label == "object")
                                            && ((kind == "index")
                                                || ((kind == "field")
                                                    && ((self::read_guarded_write_text_field(
                                                        columns, rows, size, parent, "field",
                                                    )) == "length")));
                                        {
                                            let write = ((label == "object")
                                                && ((statement == "assignReference")
                                                    || (kind == "referenceWrite")))
                                                && self::ml_array::includes(
                                                    writes,
                                                    &parent,
                                                    0f64,
                                                    false,
                                                    |a: &f64, b: &f64| {
                                                        a == b || (a.is_nan() && b.is_nan())
                                                    },
                                                );
                                            {
                                                let append = (((label == "object")
                                                    && (kind == "method"))
                                                    && ((self::read_guarded_write_text_field(
                                                        columns, rows, size, parent, "op",
                                                    )) == "push"))
                                                    && self::ml_array::includes(
                                                        appends,
                                                        &parent,
                                                        0f64,
                                                        false,
                                                        |a: &f64, b: &f64| {
                                                            a == b || (a.is_nan() && b.is_nan())
                                                        },
                                                    );
                                                {
                                                    let transfer = (label == "expr")
                                                        && (statement == "return");
                                                    if ((!read && !write) && !append) && !transfer {
                                                        self::MlAcceptConstructedGuardedArrayUsesLoop19Result::MlAcceptConstructedGuardedArrayUsesLoop19Return(false)
                                                    } else {
                                                        {
                                                            let offset_3 = offset + 1f64;
                                                            {
                                                                (uses, offset) = (uses_2, offset_3);
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
        } else {
            self::MlAcceptConstructedGuardedArrayUsesLoop19Result::MlAcceptConstructedGuardedArrayUsesLoop19Done(uses)
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
pub fn ml_accept_constructed_guarded_array_uses_loop18(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    writes: &[f64],
    appends: &[f64],
    mut node: f64,
    mut uses: f64,
    mut parent: f64,
) -> self::MlAcceptConstructedGuardedArrayUsesLoop18Result {
    loop {
        return if parent < size {
            {
                let start =
                    (self::ml_array::optional_at(rows, self::ml_array::number_index(parent)))
                        .unwrap_or_else(|| -1f64);
                {
                    let count = (self::ml_array::optional_at(
                        rows,
                        self::ml_array::number_index(size + parent),
                    ))
                    .unwrap_or_else(|| -1f64);
                    {
                        let offset = 0f64;
                        {
                            let ml_s16 = self::ml_accept_constructed_guarded_array_uses_loop19(
                                columns, rows, size, writes, appends, node, uses, parent, start,
                                count, offset,
                            );
                            match ml_s16 {
                                self::MlAcceptConstructedGuardedArrayUsesLoop19Result::MlAcceptConstructedGuardedArrayUsesLoop19Done(uses_2) => {
                                    {
                                        let parent_2 = parent + 1f64;
                                        {
                                            (uses, parent) = (uses_2, parent_2);
                                            continue;
                                        }
                                    }
                                }
                                self::MlAcceptConstructedGuardedArrayUsesLoop19Result::MlAcceptConstructedGuardedArrayUsesLoop19Return(ml_result19) => {
                                    self::MlAcceptConstructedGuardedArrayUsesLoop18Result::MlAcceptConstructedGuardedArrayUsesLoop18Return(ml_result19)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptConstructedGuardedArrayUsesLoop18Result::MlAcceptConstructedGuardedArrayUsesLoop18Done(uses)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_constructed_guarded_array_uses_loop17(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    owner: &str,
    writes: &[f64],
    appends: &[f64],
    mut node: f64,
) -> self::MlAcceptConstructedGuardedArrayUsesLoop17Result {
    loop {
        return if node < size {
            if (self::read_guarded_write_name(columns, rows, size, node)) != owner {
                {
                    let node_2 = node + 1f64;
                    {
                        node = node_2;
                        continue;
                    }
                }
            }
            {
                {
                    let uses = 0f64;
                    {
                        let parent = 0f64;
                        {
                            let ml_s17 = self::ml_accept_constructed_guarded_array_uses_loop18(
                                columns, rows, size, writes, appends, node, uses, parent,
                            );
                            match ml_s17 {
                                self::MlAcceptConstructedGuardedArrayUsesLoop18Result::MlAcceptConstructedGuardedArrayUsesLoop18Done(uses_2) => {
                                    if uses_2 == 0f64 {
                                        self::MlAcceptConstructedGuardedArrayUsesLoop17Result::MlAcceptConstructedGuardedArrayUsesLoop17Return(false)
                                    } else {
                                        {
                                            let node_3 = node + 1f64;
                                            {
                                                node = node_3;
                                                continue;
                                            }
                                        }
                                    }
                                }
                                self::MlAcceptConstructedGuardedArrayUsesLoop18Result::MlAcceptConstructedGuardedArrayUsesLoop18Return(ml_result18) => {
                                    self::MlAcceptConstructedGuardedArrayUsesLoop17Result::MlAcceptConstructedGuardedArrayUsesLoop17Return(ml_result18)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptConstructedGuardedArrayUsesLoop17Result::MlAcceptConstructedGuardedArrayUsesLoop17Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptConstructedGuardedArrayUsesLoop16Result {
    MlAcceptConstructedGuardedArrayUsesLoop16Done,
    MlAcceptConstructedGuardedArrayUsesLoop16Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_constructed_guarded_array_uses_loop16(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    owner: &str,
    mut allocation: f64,
    mut node: f64,
) -> self::MlAcceptConstructedGuardedArrayUsesLoop16Result {
    loop {
        return if node < size {
            {
                let kind = self::read_guarded_write_kind(columns, rows, size, node);
                {
                    let statement =
                        self::read_guarded_write_text_field(columns, rows, size, node, "s");
                    if (((kind == "lambda") || (statement == "block")) || (statement == "switch"))
                        || (((((statement == "assign") || (statement == "const"))
                            || (statement == "let"))
                            && (node != allocation))
                            && ((self::read_guarded_write_text_field(
                                columns, rows, size, node, "name",
                            )) == owner))
                    {
                        self::MlAcceptConstructedGuardedArrayUsesLoop16Result::MlAcceptConstructedGuardedArrayUsesLoop16Return(false)
                    } else {
                        {
                            let node_2 = node + 1f64;
                            {
                                node = node_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptConstructedGuardedArrayUsesLoop16Result::MlAcceptConstructedGuardedArrayUsesLoop16Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_constructed_guarded_array_writes_loop23(
    mut result: Vec<f64>,
    mut allocation: f64,
    writes: &[f64],
    conditionals: &[f64],
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (writes.len() as f64) {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            0f64,
                            (self::ml_array::optional_at(
                                conditionals,
                                self::ml_array::number_index(at),
                            ))
                            .unwrap_or_else(|| -1f64),
                            (self::ml_array::optional_at(writes, self::ml_array::number_index(at)))
                                .unwrap_or_else(|| -1f64),
                            allocation,
                        ],
                        result,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_constructed_guarded_array_writes_loop24(
    mut result: Vec<f64>,
    mut allocation: f64,
    appends: &[f64],
    append_statements: &[f64],
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (appends.len() as f64) {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            1f64,
                            (self::ml_array::optional_at(
                                append_statements,
                                self::ml_array::number_index(at),
                            ))
                            .unwrap_or_else(|| -1f64),
                            (self::ml_array::optional_at(
                                appends,
                                self::ml_array::number_index(at),
                            ))
                            .unwrap_or_else(|| -1f64),
                            allocation,
                        ],
                        result,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanConstructedGuardedArrayWritesLoop22Result {
    MlPlanConstructedGuardedArrayWritesLoop22Done(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_constructed_guarded_array_writes_loop22(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    owner: &str,
    mut appends: Vec<f64>,
    mut append_statements: Vec<f64>,
    mut candidate: f64,
) -> self::MlPlanConstructedGuardedArrayWritesLoop22Result {
    loop {
        if candidate < size {
            {
                let append = self::read_constructed_guarded_array_append(
                    columns, rows, size, candidate, owner,
                );
                if append >= 0f64 {
                    {
                        let append_statements_2 = {
                            let (ml_append_right, ml_append_left) =
                                (vec![candidate], append_statements);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let appends_2 = {
                                let (ml_append_right, ml_append_left) = (vec![append], appends);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let candidate_2 = candidate + 1f64;
                                {
                                    (appends, append_statements, candidate) =
                                        (appends_2, append_statements_2, candidate_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
                {
                    {
                        let candidate_3 = candidate + 1f64;
                        {
                            candidate = candidate_3;
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlPlanConstructedGuardedArrayWritesLoop22Result::MlPlanConstructedGuardedArrayWritesLoop22Done(appends, append_statements)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanConstructedGuardedArrayWritesLoop21Result {
    MlPlanConstructedGuardedArrayWritesLoop21Done(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_plan_constructed_guarded_array_writes_loop21(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    mut statements: f64,
    mut count: f64,
    owner: &str,
    mut writes: Vec<f64>,
    mut conditionals: Vec<f64>,
    mut later: f64,
) -> self::MlPlanConstructedGuardedArrayWritesLoop21Result {
    loop {
        if later < count {
            {
                let conditional =
                    self::read_guarded_write_element(columns, rows, size, statements, later);
                {
                    let write =
                        self::read_guarded_write_candidate(columns, rows, size, conditional, owner);
                    if write >= 0f64 {
                        {
                            let conditionals_2 = {
                                let (ml_append_right, ml_append_left) =
                                    (vec![conditional], conditionals);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let writes_2 = {
                                    let (ml_append_right, ml_append_left) = (vec![write], writes);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let later_2 = later + 1f64;
                                    {
                                        (writes, conditionals, later) =
                                            (writes_2, conditionals_2, later_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    {
                        {
                            let later_3 = later + 1f64;
                            {
                                later = later_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlPlanConstructedGuardedArrayWritesLoop21Result::MlPlanConstructedGuardedArrayWritesLoop21Done(writes, conditionals)
        };
    }
}
