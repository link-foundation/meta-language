// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlTemporaryAcceptConstructedGuardedArrayUsesLoop16Result;
use super::MlTemporaryAcceptConstructedGuardedArrayUsesLoop17Result;
use super::MlTemporaryAcceptGuardedWriteBoundsLoop7Result;
use super::MlTemporaryAcceptGuardedWriteColumnsLoop2Result;
use super::MlTemporaryAcceptGuardedWriteLiteralLoop4Result;
use super::MlTemporaryAcceptGuardedWriteScalarLoop5Result;
use super::MlTemporaryAcceptGuardedWriteUsesLoop8Result;
use super::MlTemporaryAcceptGuardedWriteUsesLoop9Result;
use super::MlTemporaryReadConstructedGuardedArrayAppendLoop15Result;
use super::source_ml_temporary_accept_constructed_guarded_array_uses_loop16;
use super::source_ml_temporary_accept_constructed_guarded_array_uses_loop17;
use super::source_ml_temporary_accept_guarded_write_bounds_loop7;
use super::source_ml_temporary_accept_guarded_write_columns_loop2;
use super::source_ml_temporary_accept_guarded_write_columns_loop3;
use super::source_ml_temporary_accept_guarded_write_literal_loop4;
use super::source_ml_temporary_accept_guarded_write_scalar_loop5;
use super::source_ml_temporary_accept_guarded_write_uses_loop8;
use super::source_ml_temporary_accept_guarded_write_uses_loop9;
use super::source_ml_temporary_plan_constructed_guarded_array_writes_loop20;
use super::source_ml_temporary_plan_guarded_private_array_writes_loop12;
use super::source_ml_temporary_read_constructed_guarded_array_append_loop15;

/// The Math functions that Rust's f64 methods do not share with JavaScript.
pub mod ml_math {
    /// Math.round: the nearest integer, the one towards +Infinity on a tie,
    /// with the sign of a zero result from the argument.
    #[must_use]
    pub fn round(x: f64) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        if x > 0.0 && x < 0.5 {
            return 0.0;
        }
        if (-0.5..0.0).contains(&x) {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 { floor + 1.0 } else { floor }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where `f64::signum` does not.
    #[must_use]
    pub fn sign(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            x
        }
    }

    /// Math.max of two Numbers: NaN when either is, and 0 above -0.
    #[must_use]
    pub fn max(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else if b > a {
            b
        } else if a == b {
            if a.is_sign_negative() { b } else { a }
        } else {
            f64::NAN
        }
    }

    /// Math.min of two Numbers: NaN when either is, and -0 below 0.
    #[must_use]
    pub fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else if b < a {
            b
        } else if a == b {
            if a.is_sign_negative() { a } else { b }
        } else {
            f64::NAN
        }
    }

    #[must_use]
    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    #[must_use]
    pub fn min_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| min(a, b))
    }

    #[must_use]
    pub fn is_integer(x: f64) -> bool {
        x.is_finite() && x.trunc() == x
    }

    #[must_use]
    pub fn is_safe_integer(x: f64) -> bool {
        is_integer(x) && x.abs() <= 9_007_199_254_740_991.0
    }
}

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    #[must_use]
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    #[must_use]
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        index
            .and_then(|index| values.get(index))
            .map_or_else(|| panic!("array index out of range"), Clone::clone)
    }

    /// The index a Number names: a non-negative integer, -0 included.
    #[must_use]
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9_007_199_254_740_992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    #[must_use]
    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    const fn integer(value: f64) -> f64 {
        if value.is_nan() { 0.0 } else { value.trunc() }
    }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 {
            (length as f64 + value).max(0.0) as usize
        } else {
            value.min(length as f64) as usize
        }
    }
    #[must_use]
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len());
        let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    #[must_use]
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> f64 {
        if values.is_empty() {
            return -1.0;
        }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) {
                return -1.0;
            }
            let start = if from < 0.0 {
                (values.len() as f64 + from) as usize
            } else {
                from.min((values.len() - 1) as f64) as usize
            };
            (0..=start)
                .rev()
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 {
                (values.len() as f64 + from).max(0.0) as usize
            } else {
                from.min(values.len() as f64) as usize
            };
            (start..values.len())
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        }
    }
    #[must_use]
    pub fn includes<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    #[must_use]
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values {
            if !out.iter().any(|previous| equal(previous, &value)) {
                out.push(value);
            }
        }
        out
    }
    #[must_use]
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) {
                out[index].1 = value;
            } else {
                out.push((key, value));
            }
        }
        out
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_guarded_write_field(
    columns: &[String],
    rows: &[f64],
    size: f64,
    node: f64,
    label: &str,
) -> f64 {
    if (!(self::ml_math::is_safe_integer(node)) || (node < 0f64)) || (node >= size) {
        -1f64
    } else {
        {
            let start = (self::ml_array::optional_at(rows, self::ml_array::number_index(node)))
                .unwrap_or_else(|| -1f64);
            {
                let count =
                    (self::ml_array::optional_at(rows, self::ml_array::number_index(size + node)))
                        .unwrap_or_else(|| -1f64);
                {
                    let offset = 0f64;
                    {
                        let source_ml_source1_s1 =
                            self::source_ml_temporary_read_guarded_write_field_loop1(
                                columns, rows, size, label, start, count, offset,
                            );
                        match source_ml_source1_s1 {
                            self::MlTemporaryReadGuardedWriteFieldLoop1Result::MlTemporaryReadGuardedWriteFieldLoop1Done => {
                                -1f64
                            }
                            self::MlTemporaryReadGuardedWriteFieldLoop1Result::MlTemporaryReadGuardedWriteFieldLoop1Return(source_ml_temporary_result1) => {
                                source_ml_temporary_result1
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
pub fn read_guarded_write_text(columns: &[String], size: f64, node: f64) -> String {
    if (node >= 0f64) && (node < size) {
        (self::ml_array::optional_at(columns, self::ml_array::number_index(size + node)))
            .unwrap_or_default()
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_guarded_write_text_field(
    columns: &[String],
    rows: &[f64],
    size: f64,
    node: f64,
    label: &str,
) -> String {
    self::read_guarded_write_text(
        columns,
        size,
        self::read_guarded_write_field(columns, rows, size, node, label),
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_guarded_write_kind(columns: &[String], rows: &[f64], size: f64, node: f64) -> String {
    self::read_guarded_write_text_field(columns, rows, size, node, "k")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_guarded_write_count(columns: &[String], rows: &[f64], size: f64, node: f64) -> f64 {
    if ((node >= 0f64) && (node < size))
        && (((self::ml_array::optional_at(columns, self::ml_array::number_index(node)))
            .unwrap_or_default())
            == "array")
    {
        (self::ml_array::optional_at(rows, self::ml_array::number_index(size + node)))
            .unwrap_or_else(|| -1f64)
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_guarded_write_element(
    columns: &[String],
    rows: &[f64],
    size: f64,
    node: f64,
    index: f64,
) -> f64 {
    {
        let count = self::read_guarded_write_count(columns, rows, size, node);
        if (!(self::ml_math::is_safe_integer(index)) || (index < 0f64)) || (index >= count) {
            -1f64
        } else {
            (self::ml_array::optional_at(
                rows,
                self::ml_array::number_index(
                    ((2f64 * size)
                        + (self::ml_array::optional_at(rows, self::ml_array::number_index(node)))
                            .unwrap_or_else(|| -1f64))
                        + index,
                ),
            ))
            .unwrap_or_else(|| -1f64)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_guarded_write_name(columns: &[String], rows: &[f64], size: f64, node: f64) -> String {
    if (self::read_guarded_write_kind(columns, rows, size, node)) == "name" {
        {
            let path = self::read_guarded_write_field(columns, rows, size, node, "path");
            if (self::read_guarded_write_count(columns, rows, size, path)) == 1f64 {
                self::read_guarded_write_text(
                    columns,
                    size,
                    self::read_guarded_write_element(columns, rows, size, path, 0f64),
                )
            } else {
                String::new()
            }
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_guarded_write_columns(
    columns: &[String],
    rows: &[f64],
    size: f64,
    root: f64,
) -> bool {
    if (((((!(self::ml_math::is_safe_integer(size)) || (size <= 0f64))
        || !(self::ml_math::is_safe_integer(root)))
        || (root < 0f64))
        || (root >= size))
        || ((columns.len() as f64) < (2f64 * size)))
        || ((rows.len() as f64) != (columns.len() as f64))
    {
        false
    } else {
        {
            let node = 0f64;
            {
                let source_ml_source1_s2 =
                    self::source_ml_temporary_accept_guarded_write_columns_loop2(rows, size, node);
                match source_ml_source1_s2 {
                    self::MlTemporaryAcceptGuardedWriteColumnsLoop2Result::MlTemporaryAcceptGuardedWriteColumnsLoop2Done => {
                        {
                            let edge = 2f64 * size;
                            {
                                let source_ml_source1_s3 = self::source_ml_temporary_accept_guarded_write_columns_loop3(rows, size, edge);
                                match source_ml_source1_s3 {
                                    self::MlTemporaryAcceptGuardedWriteColumnsLoop3Result::MlTemporaryAcceptGuardedWriteColumnsLoop3Done => {
                                        ((self::ml_array::optional_at(columns, self::ml_array::number_index(root))).unwrap_or_default()) == "object"
                                    }
                                    self::MlTemporaryAcceptGuardedWriteColumnsLoop3Result::MlTemporaryAcceptGuardedWriteColumnsLoop3Return(source_ml_temporary_result3) => {
                                        source_ml_temporary_result3
                                    }
                                }
                            }
                        }
                    }
                    self::MlTemporaryAcceptGuardedWriteColumnsLoop2Result::MlTemporaryAcceptGuardedWriteColumnsLoop2Return(source_ml_temporary_result3_2) => {
                        source_ml_temporary_result3_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_guarded_write_literal(
    columns: &[String],
    rows: &[f64],
    size: f64,
    node: f64,
) -> bool {
    {
        let ml_library_inequality_1 =
            (self::read_guarded_write_kind(columns, rows, size, node)) != "array";
        if ml_library_inequality_1 {
            false
        } else {
            {
                let items = self::read_guarded_write_field(columns, rows, size, node, "items");
                {
                    let count = self::read_guarded_write_count(columns, rows, size, items);
                    if count <= 0f64 {
                        false
                    } else {
                        {
                            let index = 0f64;
                            {
                                let source_ml_source1_s4 =
                                    self::source_ml_temporary_accept_guarded_write_literal_loop4(
                                        columns, rows, size, items, count, index,
                                    );
                                match source_ml_source1_s4 {
                                self::MlTemporaryAcceptGuardedWriteLiteralLoop4Result::MlTemporaryAcceptGuardedWriteLiteralLoop4Done => {
                                    true
                                }
                                self::MlTemporaryAcceptGuardedWriteLiteralLoop4Result::MlTemporaryAcceptGuardedWriteLiteralLoop4Return(source_ml_temporary_result4) => {
                                    source_ml_temporary_result4
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
pub fn accept_guarded_write_scalar(columns: &[String], rows: &[f64], size: f64, root: f64) -> bool {
    {
        let pending = vec![root];
        {
            let cursor = 0f64;
            {
                let source_ml_source1_s5 =
                    self::source_ml_temporary_accept_guarded_write_scalar_loop5(
                        columns, rows, size, pending, cursor,
                    );
                match source_ml_source1_s5 {
                    self::MlTemporaryAcceptGuardedWriteScalarLoop5Result::MlTemporaryAcceptGuardedWriteScalarLoop5Done => {
                        true
                    }
                    self::MlTemporaryAcceptGuardedWriteScalarLoop5Result::MlTemporaryAcceptGuardedWriteScalarLoop5Return(source_ml_temporary_result5) => {
                        source_ml_temporary_result5
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_guarded_write_bounds(
    columns: &[String],
    rows: &[f64],
    size: f64,
    root: f64,
    owner: &str,
    position: &str,
) -> bool {
    {
        let pending = vec![root];
        {
            let cursor = 0f64;
            {
                let integer = false;
                {
                    let lower = false;
                    {
                        let upper = false;
                        {
                            let source_ml_source1_s6 =
                                self::source_ml_temporary_accept_guarded_write_bounds_loop7(
                                    columns, rows, size, owner, position, pending, cursor, integer,
                                    lower, upper,
                                );
                            match source_ml_source1_s6 {
                                self::MlTemporaryAcceptGuardedWriteBoundsLoop7Result::MlTemporaryAcceptGuardedWriteBoundsLoop7Done(integer_2, lower_2, upper_2) => {
                                    (integer_2 && lower_2) && upper_2
                                }
                                self::MlTemporaryAcceptGuardedWriteBoundsLoop7Result::MlTemporaryAcceptGuardedWriteBoundsLoop7Return(source_ml_temporary_result7) => {
                                    source_ml_temporary_result7
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
pub fn read_guarded_write_source_node(
    columns: &[String],
    rows: &[f64],
    size: f64,
    statement: f64,
) -> f64 {
    {
        let kind = self::read_guarded_write_text_field(columns, rows, size, statement, "s");
        if kind == "assignReference" {
            statement
        } else if kind == "expr" {
            {
                let expression =
                    self::read_guarded_write_field(columns, rows, size, statement, "expr");
                if (self::read_guarded_write_kind(columns, rows, size, expression))
                    == "referenceWrite"
                {
                    expression
                } else {
                    -1f64
                }
            }
        } else {
            -1f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_guarded_write_candidate(
    columns: &[String],
    rows: &[f64],
    size: f64,
    statement: f64,
    owner: &str,
) -> f64 {
    if (self::read_guarded_write_text_field(columns, rows, size, statement, "s")) == "if" {
        {
            let alternative =
                self::read_guarded_write_field(columns, rows, size, statement, "else");
            if ((alternative >= 0f64)
                && (((self::ml_array::optional_at(
                    columns,
                    self::ml_array::number_index(alternative),
                ))
                .unwrap_or_default())
                    != "null"))
                && (((self::ml_array::optional_at(
                    columns,
                    self::ml_array::number_index(alternative),
                ))
                .unwrap_or_default())
                    != "undefined")
            {
                -1f64
            } else {
                {
                    let then =
                        self::read_guarded_write_field(columns, rows, size, statement, "then");
                    if (self::read_guarded_write_count(columns, rows, size, then)) == 1f64 {
                        {
                            let write = self::read_guarded_write_source_node(
                                columns,
                                rows,
                                size,
                                self::read_guarded_write_element(columns, rows, size, then, 0f64),
                            );
                            if ((write < 0f64)
                                || ((self::read_guarded_write_text_field(
                                    columns, rows, size, write, "op",
                                )) != "="))
                                || ((self::read_guarded_write_name(
                                    columns,
                                    rows,
                                    size,
                                    self::read_guarded_write_field(
                                        columns, rows, size, write, "object",
                                    ),
                                )) != owner)
                            {
                                -1f64
                            } else {
                                {
                                    let position = self::read_guarded_write_name(
                                        columns,
                                        rows,
                                        size,
                                        self::read_guarded_write_field(
                                            columns, rows, size, write, "index",
                                        ),
                                    );
                                    {
                                        let condition = self::read_guarded_write_field(
                                            columns, rows, size, statement, "cond",
                                        );
                                        if ((((position).is_empty())
                                            || !(self::accept_guarded_write_scalar(
                                                columns, rows, size, condition,
                                            )))
                                            || !(self::accept_guarded_write_scalar(
                                                columns,
                                                rows,
                                                size,
                                                self::read_guarded_write_field(
                                                    columns, rows, size, write, "value",
                                                ),
                                            )))
                                            || !(self::accept_guarded_write_bounds(
                                                columns,
                                                rows,
                                                size,
                                                condition,
                                                owner,
                                                &(position),
                                            ))
                                        {
                                            -1f64
                                        } else {
                                            write
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        -1f64
                    }
                }
            }
        }
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_guarded_write_uses(
    columns: &[String],
    rows: &[f64],
    size: f64,
    owner: &str,
    allocation: f64,
    candidates: &[f64],
) -> bool {
    {
        let node = 0f64;
        {
            let source_ml_source1_s7 = self::source_ml_temporary_accept_guarded_write_uses_loop8(
                columns, rows, size, owner, allocation, node,
            );
            match source_ml_source1_s7 {
                self::MlTemporaryAcceptGuardedWriteUsesLoop8Result::MlTemporaryAcceptGuardedWriteUsesLoop8Done => {
                    {
                        let node_2 = 0f64;
                        {
                            let source_ml_source1_s8 = self::source_ml_temporary_accept_guarded_write_uses_loop9(columns, rows, size, owner, candidates, node_2);
                            match source_ml_source1_s8 {
                                self::MlTemporaryAcceptGuardedWriteUsesLoop9Result::MlTemporaryAcceptGuardedWriteUsesLoop9Done => {
                                    true
                                }
                                self::MlTemporaryAcceptGuardedWriteUsesLoop9Result::MlTemporaryAcceptGuardedWriteUsesLoop9Return(source_ml_temporary_result9) => {
                                    source_ml_temporary_result9
                                }
                            }
                        }
                    }
                }
                self::MlTemporaryAcceptGuardedWriteUsesLoop8Result::MlTemporaryAcceptGuardedWriteUsesLoop8Return(source_ml_temporary_result11) => {
                    source_ml_temporary_result11
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_guarded_private_array_writes(
    columns: &[String],
    rows: &[f64],
    size: f64,
    root: f64,
) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        if (self::accept_guarded_write_columns(columns, rows, size, root)) {
            {
                let statements =
                    self::read_guarded_write_field(columns, rows, size, root, "statements");
                {
                    let count = self::read_guarded_write_count(columns, rows, size, statements);
                    if count < 0f64 {
                        result
                    } else {
                        {
                            let index = 0f64;
                            {
                                self::source_ml_temporary_plan_guarded_private_array_writes_loop12(
                                    columns, rows, size, result, statements, count, index,
                                )
                            }
                        }
                    }
                }
            }
        } else {
            result
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_constructed_guarded_array_append(
    columns: &[String],
    rows: &[f64],
    size: f64,
    statement: f64,
    owner: &str,
) -> f64 {
    {
        let ml_library_inequality_1 =
            (self::read_guarded_write_text_field(columns, rows, size, statement, "s")) != "expr";
        if ml_library_inequality_1 {
            -1f64
        } else {
            {
                let expression =
                    self::read_guarded_write_field(columns, rows, size, statement, "expr");
                if (((self::read_guarded_write_kind(columns, rows, size, expression)) != "method")
                    || ((self::read_guarded_write_text_field(
                        columns, rows, size, expression, "op",
                    )) != "push"))
                    || ((self::read_guarded_write_name(
                        columns,
                        rows,
                        size,
                        self::read_guarded_write_field(columns, rows, size, expression, "object"),
                    )) != owner)
                {
                    -1f64
                } else {
                    {
                        let args =
                            self::read_guarded_write_field(columns, rows, size, expression, "args");
                        {
                            let count = self::read_guarded_write_count(columns, rows, size, args);
                            if count <= 0f64 {
                                -1f64
                            } else {
                                {
                                    let index = 0f64;
                                    {
                                        let source_ml_source1_s9 = self::source_ml_temporary_read_constructed_guarded_array_append_loop15(columns, rows, size, args, count, index);
                                        match source_ml_source1_s9 {
                                        self::MlTemporaryReadConstructedGuardedArrayAppendLoop15Result::MlTemporaryReadConstructedGuardedArrayAppendLoop15Done => {
                                            expression
                                        }
                                        self::MlTemporaryReadConstructedGuardedArrayAppendLoop15Result::MlTemporaryReadConstructedGuardedArrayAppendLoop15Return(source_ml_temporary_result15) => {
                                            source_ml_temporary_result15
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
pub fn accept_constructed_guarded_array_uses(
    columns: &[String],
    rows: &[f64],
    size: f64,
    owner: &str,
    allocation: f64,
    writes: &[f64],
    appends: &[f64],
) -> bool {
    {
        let node = 0f64;
        {
            let source_ml_source1_s10 =
                self::source_ml_temporary_accept_constructed_guarded_array_uses_loop16(
                    columns, rows, size, owner, allocation, node,
                );
            match source_ml_source1_s10 {
                self::MlTemporaryAcceptConstructedGuardedArrayUsesLoop16Result::MlTemporaryAcceptConstructedGuardedArrayUsesLoop16Done => {
                    {
                        let node_2 = 0f64;
                        {
                            let source_ml_source1_s11 = self::source_ml_temporary_accept_constructed_guarded_array_uses_loop17(columns, rows, size, owner, writes, appends, node_2);
                            match source_ml_source1_s11 {
                                self::MlTemporaryAcceptConstructedGuardedArrayUsesLoop17Result::MlTemporaryAcceptConstructedGuardedArrayUsesLoop17Done => {
                                    true
                                }
                                self::MlTemporaryAcceptConstructedGuardedArrayUsesLoop17Result::MlTemporaryAcceptConstructedGuardedArrayUsesLoop17Return(source_ml_temporary_result17) => {
                                    source_ml_temporary_result17
                                }
                            }
                        }
                    }
                }
                self::MlTemporaryAcceptConstructedGuardedArrayUsesLoop16Result::MlTemporaryAcceptConstructedGuardedArrayUsesLoop16Return(source_ml_temporary_result19) => {
                    source_ml_temporary_result19
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_constructed_guarded_array_writes(
    columns: &[String],
    rows: &[f64],
    size: f64,
    root: f64,
) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        if (self::accept_guarded_write_columns(columns, rows, size, root)) {
            {
                let statements =
                    self::read_guarded_write_field(columns, rows, size, root, "statements");
                {
                    let count = self::read_guarded_write_count(columns, rows, size, statements);
                    if count < 0f64 {
                        result
                    } else {
                        {
                            let index = 0f64;
                            {
                                self::source_ml_temporary_plan_constructed_guarded_array_writes_loop20(columns, rows, size, result, statements, count, index)
                            }
                        }
                    }
                }
            }
        } else {
            result
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadGuardedWriteFieldLoop1Result {
    MlTemporaryReadGuardedWriteFieldLoop1Done,
    MlTemporaryReadGuardedWriteFieldLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_guarded_write_field_loop1(
    columns: &[String],
    rows: &[f64],
    mut size: f64,
    label: &str,
    mut start: f64,
    mut count: f64,
    mut offset: f64,
) -> self::MlTemporaryReadGuardedWriteFieldLoop1Result {
    loop {
        return if offset < count {
            {
                let edge = start + offset;
                if ((self::ml_array::optional_at(
                    columns,
                    self::ml_array::number_index((2f64 * size) + edge),
                ))
                .unwrap_or_default())
                    == label
                {
                    self::MlTemporaryReadGuardedWriteFieldLoop1Result::MlTemporaryReadGuardedWriteFieldLoop1Return((self::ml_array::optional_at(rows, self::ml_array::number_index((2f64 * size) + edge))).unwrap_or_else(|| -1f64))
                } else {
                    {
                        let offset_2 = offset + 1f64;
                        {
                            offset = offset_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadGuardedWriteFieldLoop1Result::MlTemporaryReadGuardedWriteFieldLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptGuardedWriteColumnsLoop3Result {
    MlTemporaryAcceptGuardedWriteColumnsLoop3Done,
    MlTemporaryAcceptGuardedWriteColumnsLoop3Return(bool),
}
