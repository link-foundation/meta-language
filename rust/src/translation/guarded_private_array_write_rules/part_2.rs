// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlTemporaryAcceptGuardedWriteColumnsLoop3Result;
use super::ml_array;
use super::ml_math;
use super::read_guarded_write_count;
use super::read_guarded_write_element;
use super::read_guarded_write_field;
use super::read_guarded_write_kind;
use super::read_guarded_write_name;
use super::read_guarded_write_text_field;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_guarded_write_columns_loop3(
    rows: &[f64],
    mut size: f64,
    mut edge: f64,
) -> self::MlTemporaryAcceptGuardedWriteColumnsLoop3Result {
    loop {
        return if edge < (rows.len() as f64) {
            {
                let target =
                    (self::ml_array::optional_at(rows, self::ml_array::number_index(edge)))
                        .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_safe_integer(target)) || (target < 0f64))
                    || (target >= size)
                {
                    self::MlTemporaryAcceptGuardedWriteColumnsLoop3Result::MlTemporaryAcceptGuardedWriteColumnsLoop3Return(false)
                } else {
                    {
                        let edge_2 = edge + 1f64;
                        {
                            edge = edge_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptGuardedWriteColumnsLoop3Result::MlTemporaryAcceptGuardedWriteColumnsLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptGuardedWriteColumnsLoop2Result {
    MlTemporaryAcceptGuardedWriteColumnsLoop2Done,
    MlTemporaryAcceptGuardedWriteColumnsLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_guarded_write_columns_loop2(
    rows: &[f64],
    mut size: f64,
    mut node: f64,
) -> self::MlTemporaryAcceptGuardedWriteColumnsLoop2Result {
    loop {
        return if node < size {
            {
                let start = (self::ml_array::optional_at(rows, self::ml_array::number_index(node)))
                    .unwrap_or_else(|| -1f64);
                {
                    let count = (self::ml_array::optional_at(
                        rows,
                        self::ml_array::number_index(size + node),
                    ))
                    .unwrap_or_else(|| -1f64);
                    if (((!(self::ml_math::is_safe_integer(start)) || (start < 0f64))
                        || !(self::ml_math::is_safe_integer(count)))
                        || (count < 0f64))
                        || ((start + count) > ((rows.len() as f64) - (2f64 * size)))
                    {
                        self::MlTemporaryAcceptGuardedWriteColumnsLoop2Result::MlTemporaryAcceptGuardedWriteColumnsLoop2Return(false)
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
            self::MlTemporaryAcceptGuardedWriteColumnsLoop2Result::MlTemporaryAcceptGuardedWriteColumnsLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptGuardedWriteLiteralLoop4Result {
    MlTemporaryAcceptGuardedWriteLiteralLoop4Done,
    MlTemporaryAcceptGuardedWriteLiteralLoop4Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_guarded_write_literal_loop4(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    mut items: f64,
    mut count: f64,
    mut index: f64,
) -> self::MlTemporaryAcceptGuardedWriteLiteralLoop4Result {
    loop {
        return if index < count {
            {
                let item = self::read_guarded_write_element(columns, rows, size, items, index);
                if (self::read_guarded_write_text_field(columns, rows, size, item, "spread"))
                    == "true"
                {
                    self::MlTemporaryAcceptGuardedWriteLiteralLoop4Result::MlTemporaryAcceptGuardedWriteLiteralLoop4Return(false)
                } else {
                    {
                        let kind = self::read_guarded_write_kind(
                            columns,
                            rows,
                            size,
                            self::read_guarded_write_field(columns, rows, size, item, "value"),
                        );
                        if ((kind != "num") && (kind != "bool")) && (kind != "str") {
                            self::MlTemporaryAcceptGuardedWriteLiteralLoop4Result::MlTemporaryAcceptGuardedWriteLiteralLoop4Return(false)
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
                }
            }
        } else {
            self::MlTemporaryAcceptGuardedWriteLiteralLoop4Result::MlTemporaryAcceptGuardedWriteLiteralLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptGuardedWriteScalarLoop5Result {
    MlTemporaryAcceptGuardedWriteScalarLoop5Done,
    MlTemporaryAcceptGuardedWriteScalarLoop5Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAcceptGuardedWriteScalarLoop6Result {
    MlTemporaryAcceptGuardedWriteScalarLoop6Done(Vec<f64>),
    MlTemporaryAcceptGuardedWriteScalarLoop6Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_guarded_write_scalar_loop6(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    mut pending: Vec<f64>,
    mut args: f64,
    mut count: f64,
    mut index: f64,
) -> self::MlTemporaryAcceptGuardedWriteScalarLoop6Result {
    loop {
        return if index < count {
            {
                let arg = self::read_guarded_write_element(columns, rows, size, args, index);
                if (self::read_guarded_write_text_field(columns, rows, size, arg, "spread"))
                    == "true"
                {
                    self::MlTemporaryAcceptGuardedWriteScalarLoop6Result::MlTemporaryAcceptGuardedWriteScalarLoop6Return(false)
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
            self::MlTemporaryAcceptGuardedWriteScalarLoop6Result::MlTemporaryAcceptGuardedWriteScalarLoop6Done(pending)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_guarded_write_scalar_loop5(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    mut pending: Vec<f64>,
    mut cursor: f64,
) -> self::MlTemporaryAcceptGuardedWriteScalarLoop5Result {
    loop {
        return if cursor < (pending.len() as f64) {
            if cursor > size {
                self::MlTemporaryAcceptGuardedWriteScalarLoop5Result::MlTemporaryAcceptGuardedWriteScalarLoop5Return(false)
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
                                if (self::read_guarded_write_name(columns, rows, size, node))
                                    .is_empty()
                                {
                                    self::MlTemporaryAcceptGuardedWriteScalarLoop5Result::MlTemporaryAcceptGuardedWriteScalarLoop5Return(false)
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
                                                self::MlTemporaryAcceptGuardedWriteScalarLoop5Result::MlTemporaryAcceptGuardedWriteScalarLoop5Return(false)
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
                                            ))
                                            .is_empty()
                                            {
                                                self::MlTemporaryAcceptGuardedWriteScalarLoop5Result::MlTemporaryAcceptGuardedWriteScalarLoop5Return(false)
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
                                                        self::MlTemporaryAcceptGuardedWriteScalarLoop5Result::MlTemporaryAcceptGuardedWriteScalarLoop5Return(false)
                                                    } else {
                                                        {
                                                            let index = 0f64;
                                                            {
                                                                let source_ml_source1_s12 = self::source_ml_temporary_accept_guarded_write_scalar_loop6(columns, rows, size, pending, args, count, index);
                                                                match source_ml_source1_s12 {
                                                                            self::MlTemporaryAcceptGuardedWriteScalarLoop6Result::MlTemporaryAcceptGuardedWriteScalarLoop6Done(pending_5) => {
                                                                                {
                                                                                    (pending, cursor) = (pending_5, cursor_2);
                                                                                    continue;
                                                                                }
                                                                            }
                                                                            self::MlTemporaryAcceptGuardedWriteScalarLoop6Result::MlTemporaryAcceptGuardedWriteScalarLoop6Return(source_ml_temporary_result6) => {
                                                                                self::MlTemporaryAcceptGuardedWriteScalarLoop5Result::MlTemporaryAcceptGuardedWriteScalarLoop5Return(source_ml_temporary_result6)
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
                                            self::MlTemporaryAcceptGuardedWriteScalarLoop5Result::MlTemporaryAcceptGuardedWriteScalarLoop5Return(false)
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
            self::MlTemporaryAcceptGuardedWriteScalarLoop5Result::MlTemporaryAcceptGuardedWriteScalarLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptGuardedWriteBoundsLoop7Result {
    MlTemporaryAcceptGuardedWriteBoundsLoop7Done(bool, bool, bool),
    MlTemporaryAcceptGuardedWriteBoundsLoop7Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_accept_guarded_write_bounds_loop7(
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
) -> self::MlTemporaryAcceptGuardedWriteBoundsLoop7Result {
    loop {
        return if cursor < (pending.len() as f64) {
            if cursor > size {
                self::MlTemporaryAcceptGuardedWriteBoundsLoop7Result::MlTemporaryAcceptGuardedWriteBoundsLoop7Return(false)
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
            self::MlTemporaryAcceptGuardedWriteBoundsLoop7Result::MlTemporaryAcceptGuardedWriteBoundsLoop7Done(integer, lower, upper)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptGuardedWriteUsesLoop9Result {
    MlTemporaryAcceptGuardedWriteUsesLoop9Done,
    MlTemporaryAcceptGuardedWriteUsesLoop9Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAcceptGuardedWriteUsesLoop10Result {
    MlTemporaryAcceptGuardedWriteUsesLoop10Done(f64),
    MlTemporaryAcceptGuardedWriteUsesLoop10Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAcceptGuardedWriteUsesLoop11Result {
    MlTemporaryAcceptGuardedWriteUsesLoop11Done(f64),
    MlTemporaryAcceptGuardedWriteUsesLoop11Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_accept_guarded_write_uses_loop11(
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
) -> self::MlTemporaryAcceptGuardedWriteUsesLoop11Result {
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
                                                    self::MlTemporaryAcceptGuardedWriteUsesLoop11Result::MlTemporaryAcceptGuardedWriteUsesLoop11Return(false)
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
            self::MlTemporaryAcceptGuardedWriteUsesLoop11Result::MlTemporaryAcceptGuardedWriteUsesLoop11Done(uses)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_guarded_write_uses_loop10(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    candidates: &[f64],
    mut node: f64,
    mut uses: f64,
    mut parent: f64,
) -> self::MlTemporaryAcceptGuardedWriteUsesLoop10Result {
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
                            let source_ml_source1_s13 =
                                self::source_ml_temporary_accept_guarded_write_uses_loop11(
                                    columns, rows, size, candidates, node, uses, parent, start,
                                    count, offset,
                                );
                            match source_ml_source1_s13 {
                                self::MlTemporaryAcceptGuardedWriteUsesLoop11Result::MlTemporaryAcceptGuardedWriteUsesLoop11Done(uses_2) => {
                                    {
                                        let parent_2 = parent + 1f64;
                                        {
                                            (uses, parent) = (uses_2, parent_2);
                                            continue;
                                        }
                                    }
                                }
                                self::MlTemporaryAcceptGuardedWriteUsesLoop11Result::MlTemporaryAcceptGuardedWriteUsesLoop11Return(source_ml_temporary_result11) => {
                                    self::MlTemporaryAcceptGuardedWriteUsesLoop10Result::MlTemporaryAcceptGuardedWriteUsesLoop10Return(source_ml_temporary_result11)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptGuardedWriteUsesLoop10Result::MlTemporaryAcceptGuardedWriteUsesLoop10Done(uses)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_guarded_write_uses_loop9(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    owner: &str,
    candidates: &[f64],
    mut node: f64,
) -> self::MlTemporaryAcceptGuardedWriteUsesLoop9Result {
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
                            let source_ml_source1_s14 =
                                self::source_ml_temporary_accept_guarded_write_uses_loop10(
                                    columns, rows, size, candidates, node, uses, parent,
                                );
                            match source_ml_source1_s14 {
                                self::MlTemporaryAcceptGuardedWriteUsesLoop10Result::MlTemporaryAcceptGuardedWriteUsesLoop10Done(uses_2) => {
                                    if uses_2 == 0f64 {
                                        self::MlTemporaryAcceptGuardedWriteUsesLoop9Result::MlTemporaryAcceptGuardedWriteUsesLoop9Return(false)
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
                                self::MlTemporaryAcceptGuardedWriteUsesLoop10Result::MlTemporaryAcceptGuardedWriteUsesLoop10Return(source_ml_temporary_result10) => {
                                    self::MlTemporaryAcceptGuardedWriteUsesLoop9Result::MlTemporaryAcceptGuardedWriteUsesLoop9Return(source_ml_temporary_result10)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptGuardedWriteUsesLoop9Result::MlTemporaryAcceptGuardedWriteUsesLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptGuardedWriteUsesLoop8Result {
    MlTemporaryAcceptGuardedWriteUsesLoop8Done,
    MlTemporaryAcceptGuardedWriteUsesLoop8Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_guarded_write_uses_loop8(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    owner: &str,
    mut allocation: f64,
    mut node: f64,
) -> self::MlTemporaryAcceptGuardedWriteUsesLoop8Result {
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
                        self::MlTemporaryAcceptGuardedWriteUsesLoop8Result::MlTemporaryAcceptGuardedWriteUsesLoop8Return(false)
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
            self::MlTemporaryAcceptGuardedWriteUsesLoop8Result::MlTemporaryAcceptGuardedWriteUsesLoop8Done
        };
    }
}
