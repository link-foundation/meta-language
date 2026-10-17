// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlAcceptGuardedWriteScalarLoop5Result;
use super::ml_array;
use super::read_guarded_write_candidate;
use super::read_guarded_write_count;
use super::read_guarded_write_element;
use super::read_guarded_write_field;
use super::read_guarded_write_kind;
use super::read_guarded_write_name;
use super::read_guarded_write_text_field;

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptGuardedWriteScalarLoop6Result {
    MlAcceptGuardedWriteScalarLoop6Done(Vec<f64>),
    MlAcceptGuardedWriteScalarLoop6Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_guarded_write_scalar_loop6(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    mut pending: Vec<f64>,
    mut args: f64,
    mut count: f64,
    mut index: f64,
) -> self::MlAcceptGuardedWriteScalarLoop6Result {
    loop {
        return if index < count {
            {
                let arg = self::read_guarded_write_element(columns, rows, size, args, index);
                if (self::read_guarded_write_text_field(columns, rows, size, arg, "spread"))
                    == "true"
                {
                    self::MlAcceptGuardedWriteScalarLoop6Result::MlAcceptGuardedWriteScalarLoop6Return(false)
                } else {
                    {
                        let pending_2 = {
                            let (ml_append_right, ml_append_left) = (
                                vec![self::read_guarded_write_field(
                                    columns, rows, size, arg, "value",
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
            }
        } else {
            self::MlAcceptGuardedWriteScalarLoop6Result::MlAcceptGuardedWriteScalarLoop6Done(
                pending,
            )
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_guarded_write_scalar_loop5(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    mut pending: Vec<f64>,
    mut cursor: f64,
) -> self::MlAcceptGuardedWriteScalarLoop5Result {
    loop {
        return if cursor < (pending.len() as f64) {
            if cursor > size {
                self::MlAcceptGuardedWriteScalarLoop5Result::MlAcceptGuardedWriteScalarLoop5Return(
                    false,
                )
            } else {
                {
                    let node = (self::ml_array::optional_at(
                        &pending,
                        self::ml_array::number_index(cursor),
                    ))
                    .unwrap_or_else(|| -1f64);
                    {
                        let cursor_2 = cursor + 1f64;
                        {
                            let kind = self::read_guarded_write_kind(columns, rows, size, node);
                            if kind == "name" {
                                if (self::read_guarded_write_name(columns, rows, size, node)) == ""
                                {
                                    self::MlAcceptGuardedWriteScalarLoop5Result::MlAcceptGuardedWriteScalarLoop5Return(false)
                                } else {
                                    {
                                        cursor = cursor_2;
                                        continue;
                                    }
                                }
                            } else {
                                if kind == "binary" {
                                    {
                                        let pending_2 = {
                                            let (ml_append_right, ml_append_left) = (
                                                vec![
                                                    self::read_guarded_write_field(
                                                        columns, rows, size, node, "left",
                                                    ),
                                                    self::read_guarded_write_field(
                                                        columns, rows, size, node, "right",
                                                    ),
                                                ],
                                                pending,
                                            );
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            (pending, cursor) = (pending_2, cursor_2);
                                            continue;
                                        }
                                    }
                                }
                                {
                                    if kind == "unary" {
                                        {
                                            let pending_3 = {
                                                let (ml_append_right, ml_append_left) = (
                                                    vec![self::read_guarded_write_field(
                                                        columns, rows, size, node, "arg",
                                                    )],
                                                    pending,
                                                );
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            };
                                            {
                                                (pending, cursor) = (pending_3, cursor_2);
                                                continue;
                                            }
                                        }
                                    }
                                    {
                                        if kind == "field" {
                                            if ((self::read_guarded_write_text_field(
                                                columns, rows, size, node, "field",
                                            )) != "length")
                                                || ((self::read_guarded_write_name(
                                                    columns,
                                                    rows,
                                                    size,
                                                    self::read_guarded_write_field(
                                                        columns, rows, size, node, "object",
                                                    ),
                                                ))
                                                .is_empty())
                                            {
                                                self::MlAcceptGuardedWriteScalarLoop5Result::MlAcceptGuardedWriteScalarLoop5Return(false)
                                            } else {
                                                {
                                                    cursor = cursor_2;
                                                    continue;
                                                }
                                            }
                                        } else if kind == "index" {
                                            if (self::read_guarded_write_name(
                                                columns,
                                                rows,
                                                size,
                                                self::read_guarded_write_field(
                                                    columns, rows, size, node, "object",
                                                ),
                                            )) == ""
                                            {
                                                self::MlAcceptGuardedWriteScalarLoop5Result::MlAcceptGuardedWriteScalarLoop5Return(false)
                                            } else {
                                                {
                                                    let pending_4 = {
                                                        let (ml_append_right, ml_append_left) = (
                                                            vec![self::read_guarded_write_field(
                                                                columns, rows, size, node, "index",
                                                            )],
                                                            pending,
                                                        );
                                                        self::ml_array::append(
                                                            ml_append_left,
                                                            ml_append_right,
                                                        )
                                                    };
                                                    {
                                                        (pending, cursor) = (pending_4, cursor_2);
                                                        continue;
                                                    }
                                                }
                                            }
                                        } else if kind == "math" {
                                            {
                                                let args = self::read_guarded_write_field(
                                                    columns, rows, size, node, "args",
                                                );
                                                {
                                                    let count = self::read_guarded_write_count(
                                                        columns, rows, size, args,
                                                    );
                                                    if count < 0f64 {
                                                        self::MlAcceptGuardedWriteScalarLoop5Result::MlAcceptGuardedWriteScalarLoop5Return(false)
                                                    } else {
                                                        {
                                                            let index = 0f64;
                                                            {
                                                                let ml_s9 = self::ml_accept_guarded_write_scalar_loop6(columns, rows, size, pending, args, count, index);
                                                                match ml_s9 {
                                                                            self::MlAcceptGuardedWriteScalarLoop6Result::MlAcceptGuardedWriteScalarLoop6Done(pending_5) => {
                                                                                {
                                                                                    (pending, cursor) = (pending_5, cursor_2);
                                                                                    continue;
                                                                                }
                                                                            }
                                                                            self::MlAcceptGuardedWriteScalarLoop6Result::MlAcceptGuardedWriteScalarLoop6Return(ml_result6) => {
                                                                                self::MlAcceptGuardedWriteScalarLoop5Result::MlAcceptGuardedWriteScalarLoop5Return(ml_result6)
                                                                            }
                                                                        }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else if ((((kind != "num") && (kind != "bool"))
                                            && (kind != "str"))
                                            && (kind != "none"))
                                            && (kind != "unit")
                                        {
                                            self::MlAcceptGuardedWriteScalarLoop5Result::MlAcceptGuardedWriteScalarLoop5Return(false)
                                        } else {
                                            {
                                                cursor = cursor_2;
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
            self::MlAcceptGuardedWriteScalarLoop5Result::MlAcceptGuardedWriteScalarLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptGuardedWriteBoundsLoop7Result {
    MlAcceptGuardedWriteBoundsLoop7Done(bool, bool, bool),
    MlAcceptGuardedWriteBoundsLoop7Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_accept_guarded_write_bounds_loop7(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    owner: &str,
    position: &str,
    mut pending: Vec<f64>,
    mut cursor: f64,
    mut integer: bool,
    mut lower: bool,
    mut upper: bool,
) -> self::MlAcceptGuardedWriteBoundsLoop7Result {
    loop {
        return if cursor < (pending.len() as f64) {
            if cursor > size {
                self::MlAcceptGuardedWriteBoundsLoop7Result::MlAcceptGuardedWriteBoundsLoop7Return(
                    false,
                )
            } else {
                {
                    let node = (self::ml_array::optional_at(
                        &pending,
                        self::ml_array::number_index(cursor),
                    ))
                    .unwrap_or_else(|| -1f64);
                    {
                        let cursor_2 = cursor + 1f64;
                        {
                            let kind = self::read_guarded_write_kind(columns, rows, size, node);
                            {
                                let op = self::read_guarded_write_text_field(
                                    columns, rows, size, node, "op",
                                );
                                if (kind == "binary") && (op == "and") {
                                    {
                                        let pending_2 = {
                                            let (ml_append_right, ml_append_left) = (
                                                vec![
                                                    self::read_guarded_write_field(
                                                        columns, rows, size, node, "left",
                                                    ),
                                                    self::read_guarded_write_field(
                                                        columns, rows, size, node, "right",
                                                    ),
                                                ],
                                                pending,
                                            );
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            (pending, cursor) = (pending_2, cursor_2);
                                            continue;
                                        }
                                    }
                                }
                                {
                                    if (kind == "math")
                                        && ((op == "isInteger") || (op == "isSafeInteger"))
                                    {
                                        {
                                            let args = self::read_guarded_write_field(
                                                columns, rows, size, node, "args",
                                            );
                                            if (self::read_guarded_write_count(
                                                columns, rows, size, args,
                                            )) == 1f64
                                            {
                                                {
                                                    let arg = self::read_guarded_write_element(
                                                        columns, rows, size, args, 0f64,
                                                    );
                                                    if ((self::read_guarded_write_text_field(
                                                        columns, rows, size, arg, "spread",
                                                    )) != "true")
                                                        && ((self::read_guarded_write_name(
                                                            columns,
                                                            rows,
                                                            size,
                                                            self::read_guarded_write_field(
                                                                columns, rows, size, arg, "value",
                                                            ),
                                                        )) == position)
                                                    {
                                                        {
                                                            let integer_2 = true;
                                                            {
                                                                (cursor, integer) =
                                                                    (cursor_2, integer_2);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                    {
                                                        {
                                                            cursor = cursor_2;
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                            {
                                                {
                                                    cursor = cursor_2;
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    {
                                        if (kind == "binary")
                                            && ((self::read_guarded_write_name(
                                                columns,
                                                rows,
                                                size,
                                                self::read_guarded_write_field(
                                                    columns, rows, size, node, "left",
                                                ),
                                            )) == position)
                                        {
                                            {
                                                let right = self::read_guarded_write_field(
                                                    columns, rows, size, node, "right",
                                                );
                                                {
                                                    let lower_3 = if ((op == "ge")
                                                        && ((self::read_guarded_write_kind(
                                                            columns, rows, size, right,
                                                        )) == "num"))
                                                        && ((self::read_guarded_write_text_field(
                                                            columns, rows, size, right, "value",
                                                        )) == "0")
                                                    {
                                                        { true }
                                                    } else {
                                                        lower
                                                    };
                                                    if (((op == "lt")
                                                        && ((self::read_guarded_write_kind(
                                                            columns, rows, size, right,
                                                        )) == "field"))
                                                        && ((self::read_guarded_write_text_field(
                                                            columns, rows, size, right, "field",
                                                        )) == "length"))
                                                        && ((self::read_guarded_write_name(
                                                            columns,
                                                            rows,
                                                            size,
                                                            self::read_guarded_write_field(
                                                                columns, rows, size, right,
                                                                "object",
                                                            ),
                                                        )) == owner)
                                                    {
                                                        {
                                                            let upper_2 = true;
                                                            {
                                                                (cursor, lower, upper) =
                                                                    (cursor_2, lower_3, upper_2);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                    {
                                                        {
                                                            (cursor, lower) = (cursor_2, lower_3);
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            {
                                                cursor = cursor_2;
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
            self::MlAcceptGuardedWriteBoundsLoop7Result::MlAcceptGuardedWriteBoundsLoop7Done(
                integer, lower, upper,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptGuardedWriteUsesLoop9Result {
    MlAcceptGuardedWriteUsesLoop9Done,
    MlAcceptGuardedWriteUsesLoop9Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptGuardedWriteUsesLoop10Result {
    MlAcceptGuardedWriteUsesLoop10Done(f64),
    MlAcceptGuardedWriteUsesLoop10Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptGuardedWriteUsesLoop11Result {
    MlAcceptGuardedWriteUsesLoop11Done(f64),
    MlAcceptGuardedWriteUsesLoop11Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_accept_guarded_write_uses_loop11(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    candidates: &[f64],
    mut node: f64,
    mut uses: f64,
    mut parent: f64,
    mut start: f64,
    mut count: f64,
    mut offset: f64,
) -> self::MlAcceptGuardedWriteUsesLoop11Result {
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
                            .unwrap_or_else(|| String::new());
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
                                                    candidates,
                                                    &parent,
                                                    0f64,
                                                    false,
                                                    |a: &f64, b: &f64| {
                                                        a == b || (a.is_nan() && b.is_nan())
                                                    },
                                                );
                                            {
                                                let transfer =
                                                    (label == "expr") && (statement == "return");
                                                if (!read && !write) && !transfer {
                                                    self::MlAcceptGuardedWriteUsesLoop11Result::MlAcceptGuardedWriteUsesLoop11Return(false)
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
        } else {
            self::MlAcceptGuardedWriteUsesLoop11Result::MlAcceptGuardedWriteUsesLoop11Done(uses)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_guarded_write_uses_loop10(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    candidates: &[f64],
    mut node: f64,
    mut uses: f64,
    mut parent: f64,
) -> self::MlAcceptGuardedWriteUsesLoop10Result {
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
                            let ml_s10 = self::ml_accept_guarded_write_uses_loop11(
                                columns, rows, size, candidates, node, uses, parent, start, count,
                                offset,
                            );
                            match ml_s10 {
                                self::MlAcceptGuardedWriteUsesLoop11Result::MlAcceptGuardedWriteUsesLoop11Done(uses_2) => {
                                    {
                                        let parent_2 = parent + 1f64;
                                        {
                                            (uses, parent) = (uses_2, parent_2);
                                            continue;
                                        }
                                    }
                                }
                                self::MlAcceptGuardedWriteUsesLoop11Result::MlAcceptGuardedWriteUsesLoop11Return(ml_result11) => {
                                    self::MlAcceptGuardedWriteUsesLoop10Result::MlAcceptGuardedWriteUsesLoop10Return(ml_result11)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptGuardedWriteUsesLoop10Result::MlAcceptGuardedWriteUsesLoop10Done(uses)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_guarded_write_uses_loop9(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    owner: &str,
    candidates: &[f64],
    mut node: f64,
) -> self::MlAcceptGuardedWriteUsesLoop9Result {
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
                            let ml_s11 = self::ml_accept_guarded_write_uses_loop10(
                                columns, rows, size, candidates, node, uses, parent,
                            );
                            match ml_s11 {
                                self::MlAcceptGuardedWriteUsesLoop10Result::MlAcceptGuardedWriteUsesLoop10Done(uses_2) => {
                                    if uses_2 == 0f64 {
                                        self::MlAcceptGuardedWriteUsesLoop9Result::MlAcceptGuardedWriteUsesLoop9Return(false)
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
                                self::MlAcceptGuardedWriteUsesLoop10Result::MlAcceptGuardedWriteUsesLoop10Return(ml_result10) => {
                                    self::MlAcceptGuardedWriteUsesLoop9Result::MlAcceptGuardedWriteUsesLoop9Return(ml_result10)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptGuardedWriteUsesLoop9Result::MlAcceptGuardedWriteUsesLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptGuardedWriteUsesLoop8Result {
    MlAcceptGuardedWriteUsesLoop8Done,
    MlAcceptGuardedWriteUsesLoop8Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_guarded_write_uses_loop8(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    owner: &str,
    mut allocation: f64,
    mut node: f64,
) -> self::MlAcceptGuardedWriteUsesLoop8Result {
    loop {
        return if node < size {
            {
                let kind = self::read_guarded_write_kind(columns, rows, size, node);
                {
                    let statement =
                        self::read_guarded_write_text_field(columns, rows, size, node, "s");
                    if ((((((kind == "lambda") || (statement == "block"))
                        || (statement == "for"))
                        || (statement == "while"))
                        || (statement == "doWhile"))
                        || (statement == "switch"))
                        || (((((statement == "assign") || (statement == "const"))
                            || (statement == "let"))
                            && (node != allocation))
                            && ((self::read_guarded_write_text_field(
                                columns, rows, size, node, "name",
                            )) == owner))
                    {
                        self::MlAcceptGuardedWriteUsesLoop8Result::MlAcceptGuardedWriteUsesLoop8Return(false)
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
            self::MlAcceptGuardedWriteUsesLoop8Result::MlAcceptGuardedWriteUsesLoop8Done
        };
    }
}

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
