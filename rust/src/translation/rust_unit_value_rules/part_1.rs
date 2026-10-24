// Generated from complete js/src/translation/rust-unit-value-rules.js. Do not edit by hand.
use super::source_ml_temporary_read_rust_function_completion_presence_loop5;

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
pub fn accept_rust_completion_model(model: &str) -> bool {
    ((model).is_empty()) || (model == "closed guarded function returns 1")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_return_ownership(proven: bool, indirected: bool) -> bool {
    proven && !indirected
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_conditional_completion(
    then_kind: &str,
    otherwise_kind: &str,
    has_else: bool,
    continuation: bool,
) -> String {
    if (((then_kind == "returned") && ((otherwise_kind).is_empty())) && (has_else == (false)))
        && (continuation == (true))
    {
        String::from("guarded continuation")
    } else if (((then_kind == "returned") && (otherwise_kind == "returned"))
        && (has_else == (true)))
        && (continuation == (false))
    {
        String::from("returned conditional")
    } else if (((then_kind == "value") && (otherwise_kind == "value")) && (has_else == (true)))
        && (continuation == (false))
    {
        String::from("conditional value")
    } else if (((then_kind == "returned") && (otherwise_kind == "returned"))
        && (has_else == (true)))
        && (continuation == (true))
    {
        String::from("unreachable continuation")
    } else if (((then_kind == "value") && (otherwise_kind == "value")) && (has_else == (true)))
        && (continuation == (true))
    {
        String::from("discarded conditional value")
    } else if (((then_kind == "returned") && (otherwise_kind == "aborted")) && (has_else == (true)))
        && (continuation == (false))
    {
        String::from("returned conditional")
    } else if (((then_kind == "returned") && (otherwise_kind == "aborted")) && (has_else == (true)))
        && (continuation == (true))
    {
        String::from("unreachable continuation")
    } else if (((then_kind == "aborted") && (otherwise_kind == "returned")) && (has_else == (true)))
        && (continuation == (false))
    {
        String::from("returned conditional")
    } else if (((then_kind == "aborted") && (otherwise_kind == "returned")) && (has_else == (true)))
        && (continuation == (true))
    {
        String::from("unreachable continuation")
    } else if (((then_kind == "aborted") && (otherwise_kind == "aborted")) && (has_else == (true)))
        && (continuation == (false))
    {
        String::from("aborted conditional")
    } else if (((then_kind == "aborted") && (otherwise_kind == "aborted")) && (has_else == (true)))
        && (continuation == (true))
    {
        String::from("unreachable continuation")
    } else if (((then_kind == "value") && (otherwise_kind == "aborted")) && (has_else == (true)))
        && (continuation == (false))
    {
        String::from("conditional value")
    } else if (((then_kind == "value") && (otherwise_kind == "aborted")) && (has_else == (true)))
        && (continuation == (true))
    {
        String::from("discarded conditional value")
    } else if (((then_kind == "aborted") && (otherwise_kind == "value")) && (has_else == (true)))
        && (continuation == (false))
    {
        String::from("conditional value")
    } else if (((then_kind == "aborted") && (otherwise_kind == "value")) && (has_else == (true)))
        && (continuation == (true))
    {
        String::from("discarded conditional value")
    } else if (((then_kind == "aborted") && ((otherwise_kind).is_empty())) && (has_else == (false)))
        && (continuation == (true))
    {
        String::from("guarded continuation")
    } else {
        String::from("unsupported completion")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_completion_kind(action: &str, then_kind: &str, otherwise_kind: &str) -> String {
    if ((action == "returned conditional") && (then_kind == "returned"))
        && (otherwise_kind == "returned")
    {
        String::from("returned")
    } else if ((action == "conditional value") && (then_kind == "value"))
        && (otherwise_kind == "value")
    {
        String::from("value")
    } else if (action == "returned conditional")
        && (((then_kind == "returned") && (otherwise_kind == "aborted"))
            || ((then_kind == "aborted") && (otherwise_kind == "returned")))
    {
        String::from("returned")
    } else if ((action == "aborted conditional") && (then_kind == "aborted"))
        && (otherwise_kind == "aborted")
    {
        String::from("aborted")
    } else if ((((action == "conditional value") && (then_kind == "value"))
        && (otherwise_kind == "aborted"))
        || (((action == "conditional value") && (then_kind == "aborted"))
            && (otherwise_kind == "value")))
        || (((action == "guarded continuation") && (then_kind == "returned"))
            && (otherwise_kind == "value"))
    {
        String::from("value")
    } else if ((action == "guarded continuation") && (then_kind == "returned"))
        && ((otherwise_kind == "returned") || (otherwise_kind == "aborted"))
    {
        String::from("returned")
    } else if ((action == "guarded continuation") && (then_kind == "aborted"))
        && (otherwise_kind == "value")
    {
        String::from("value")
    } else if ((action == "guarded continuation") && (then_kind == "aborted"))
        && (otherwise_kind == "returned")
    {
        String::from("returned")
    } else if ((action == "guarded continuation") && (then_kind == "aborted"))
        && (otherwise_kind == "aborted")
    {
        String::from("aborted")
    } else {
        String::from("unsupported completion")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_completion_panic_arguments(
    model: &str,
    macro_name: &str,
    first_kind: &str,
    first_value: &str,
    second_kind: &str,
    count: &str,
) -> bool {
    (((((model == "closed guarded function returns 1") && (macro_name == "panic"))
        && (first_kind == "str"))
        && (first_value == "{}"))
        && (second_kind == "str"))
        && (count == "2")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_ordinary_completion_profiles(profiles: &[String]) -> bool {
    {
        let index = 0f64;
        {
            let source_ml_source1_s1 =
                self::source_ml_temporary_accept_rust_ordinary_completion_profiles_loop1(
                    profiles, index,
                );
            match source_ml_source1_s1 {
                self::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Result::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Done => {
                    true
                }
                self::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Result::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Return(source_ml_temporary_result1) => {
                    source_ml_temporary_result1
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_function_return_presence(values: &[String]) -> f64 {
    if {
        let source_ml_source1_comparison_operand2 =
            self::ml_array::optional_at(values, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand3 = String::from("{");
            !(match (
                source_ml_source1_comparison_operand2,
                source_ml_source1_comparison_operand3,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            })
        }
    } {
        0f64
    } else {
        {
            let depth = 0f64;
            {
                let index = 0f64;
                {
                    let source_ml_source1_s4 =
                        self::source_ml_temporary_read_rust_function_return_presence_loop2(
                            values, depth, index,
                        );
                    match source_ml_source1_s4 {
                        self::MlTemporaryReadRustFunctionReturnPresenceLoop2Result::MlTemporaryReadRustFunctionReturnPresenceLoop2Done => {
                            -1f64
                        }
                        self::MlTemporaryReadRustFunctionReturnPresenceLoop2Result::MlTemporaryReadRustFunctionReturnPresenceLoop2Return(source_ml_temporary_result2) => {
                            source_ml_temporary_result2
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_ordinary_conditional_completion(
    first: &str,
    second: &str,
    has_else: bool,
    continuation: bool,
    terminated: bool,
) -> String {
    {
        let original =
            self::read_rust_conditional_completion(first, second, has_else, continuation);
        if original == "unsupported completion" {
            {
                let first_known = (((first == "returned") || (first == "aborted"))
                    || (first == "value"))
                    || (first == "mixed");
                {
                    let second_known = (((second == "returned") || (second == "aborted"))
                        || (second == "value"))
                        || (second == "mixed");
                    if (!continuation && !has_else) && first_known {
                        String::from("implicit Unit fallback")
                    } else if (((!continuation && has_else) && !terminated) && first_known)
                        && second_known
                    {
                        String::from("terminal mixed conditional")
                    } else {
                        original
                    }
                }
            }
        } else {
            original
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_ordinary_completion_kind(action: &str, first: &str, second: &str) -> String {
    if action == "terminal mixed conditional" {
        String::from("mixed")
    } else if action == "implicit Unit fallback" {
        if first == "value" {
            String::from("value")
        } else {
            String::from("mixed")
        }
    } else if (action == "guarded continuation")
        && ((second == "mixed") || ((first == "returned") && (second == "value")))
    {
        String::from("mixed")
    } else {
        self::read_rust_completion_kind(action, first, second)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_ordinary_else_if(first: &str) -> bool {
    (first == "returned") || (first == "aborted")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_unit_statement_disposition(
    kind: &str,
    collecting: bool,
    proven: bool,
    type_: &str,
    indirected: bool,
) -> String {
    if kind == "abort" {
        String::from("abort")
    } else if ((proven && (type_ == "unit")) && !indirected) || (collecting && (kind == "app")) {
        String::from("bind")
    } else {
        String::from("refuse")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_function_macro_disposition(name: &str) -> String {
    if name == "println" {
        String::from("print")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_rust_terminal_return_boundary(values: &[String], start: f64) -> f64 {
    if (!(self::ml_math::is_safe_integer(start)) || (start < 0f64))
        || (start >= (values.len() as f64))
    {
        -1f64
    } else {
        {
            let stack = Vec::<String>::new();
            {
                let at = start + 1f64;
                {
                    let source_ml_source1_s5 =
                        self::source_ml_temporary_find_rust_terminal_return_boundary_loop3(
                            values, stack, at,
                        );
                    match source_ml_source1_s5 {
                        self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Done => {
                            -2f64
                        }
                        self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return(source_ml_temporary_result3) => {
                            source_ml_temporary_result3
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_ordinary_completion_construction(kind: &str, terminal: bool) -> String {
    if kind == "abort" {
        String::from("value")
    } else if kind == "return" {
        if terminal {
            String::from("value")
        } else {
            String::from("completed")
        }
    } else if kind == "value" {
        if terminal {
            String::from("value")
        } else {
            String::from("continuation")
        }
    } else if kind == "binding" {
        String::from("binding")
    } else if kind == "conditional" {
        String::from("conditional")
    } else if kind == "sequence" {
        String::from("join")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_ordinary_completion_alternative_kind(first: &str, second: &str) -> String {
    if (first == "aborted") && (second == "aborted") {
        String::from("aborted")
    } else if ((first == "returned") || (first == "aborted"))
        && ((second == "returned") || (second == "aborted"))
    {
        String::from("returned")
    } else if (first == "value") && (second == "value") {
        String::from("value")
    } else {
        String::from("mixed")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_ordinary_completed_alternative(kind: &str) -> bool {
    (kind == "returned") || (kind == "aborted")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_ordinary_completion_sequence_kind(first: &str, second: &str) -> String {
    if first == "value" {
        second.to_owned()
    } else if (first == "returned") || (first == "aborted") {
        first.to_owned()
    } else if (second == "returned") || (second == "aborted") {
        String::from("returned")
    } else {
        String::from("mixed")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_function_completion_presence(values: &[String]) -> f64 {
    if {
        let source_ml_source1_comparison_operand6 =
            self::ml_array::optional_at(values, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand7 = String::from("{");
            !(match (
                source_ml_source1_comparison_operand6,
                source_ml_source1_comparison_operand7,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            })
        }
    } {
        0f64
    } else {
        {
            let depth = 0f64;
            {
                let index = 0f64;
                {
                    let source_ml_source1_s8 =
                        self::source_ml_temporary_read_rust_function_completion_presence_loop5(
                            values, depth, index,
                        );
                    match source_ml_source1_s8 {
                        self::MlTemporaryReadRustFunctionCompletionPresenceLoop5Result::MlTemporaryReadRustFunctionCompletionPresenceLoop5Done => {
                            -1f64
                        }
                        self::MlTemporaryReadRustFunctionCompletionPresenceLoop5Result::MlTemporaryReadRustFunctionCompletionPresenceLoop5Return(source_ml_temporary_result5) => {
                            source_ml_temporary_result5
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Result {
    MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Done,
    MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_rust_ordinary_completion_profiles_loop1(
    profiles: &[String],
    mut index: f64,
) -> self::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Result {
    loop {
        return if index < (profiles.len() as f64) {
            if {
                let source_ml_source1_comparison_operand9 =
                    self::ml_array::optional_at(profiles, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand10 = String::new();
                    !(match (
                        source_ml_source1_comparison_operand9,
                        source_ml_source1_comparison_operand10,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
                self::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Result::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Return(false)
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
            self::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Result::MlTemporaryAcceptRustOrdinaryCompletionProfilesLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustFunctionReturnPresenceLoop2Result {
    MlTemporaryReadRustFunctionReturnPresenceLoop2Done,
    MlTemporaryReadRustFunctionReturnPresenceLoop2Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_function_return_presence_loop2(
    values: &[String],
    mut depth: f64,
    mut index: f64,
) -> self::MlTemporaryReadRustFunctionReturnPresenceLoop2Result {
    loop {
        return if index < (values.len() as f64) {
            {
                let value =
                    (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| String::new());
                if value == "return" {
                    self::MlTemporaryReadRustFunctionReturnPresenceLoop2Result::MlTemporaryReadRustFunctionReturnPresenceLoop2Return(1f64)
                } else {
                    {
                        let depth_3 = if value == "{" {
                            { depth + 1f64 }
                        } else {
                            depth
                        };
                        if value == "}" {
                            {
                                let depth_4 = depth_3 - 1f64;
                                if depth_4 == 0f64 {
                                    self::MlTemporaryReadRustFunctionReturnPresenceLoop2Result::MlTemporaryReadRustFunctionReturnPresenceLoop2Return(0f64)
                                } else {
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            (depth, index) = (depth_4, index_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let index_3 = index + 1f64;
                                {
                                    (depth, index) = (depth_3, index_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustFunctionReturnPresenceLoop2Result::MlTemporaryReadRustFunctionReturnPresenceLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindRustTerminalReturnBoundaryLoop3Result {
    MlTemporaryFindRustTerminalReturnBoundaryLoop3Done,
    MlTemporaryFindRustTerminalReturnBoundaryLoop3Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_rust_terminal_return_boundary_loop4(
    values: &[String],
    mut end: f64,
) -> f64 {
    loop {
        return if {
            let source_ml_source1_comparison_operand11 =
                self::ml_array::optional_at(values, self::ml_array::number_index(end));
            {
                let source_ml_source1_comparison_operand12 = String::from(";");
                match (
                    source_ml_source1_comparison_operand11,
                    source_ml_source1_comparison_operand12,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } {
            {
                let end_2 = end + 1f64;
                {
                    end = end_2;
                    continue;
                }
            }
        } else {
            end
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_rust_terminal_return_boundary_loop3(
    values: &[String],
    mut stack: Vec<String>,
    mut at: f64,
) -> self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result {
    loop {
        return if at < (values.len() as f64) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| String::new());
                if ((value == "(") || (value == "[")) || (value == "{") {
                    {
                        let stack_2 = {
                            let (ml_append_right, ml_append_left) = (vec![value.clone()], stack);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let at_2 = at + 1f64;
                            {
                                (stack, at) = (stack_2, at_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    if ((stack.len() as f64) == 0f64) && ((value == ";") || (value == "}")) {
                        {
                            let end = at;
                            {
                                let end_2 = self::source_ml_temporary_find_rust_terminal_return_boundary_loop4(values, end);
                                if end_2 >= (values.len() as f64) {
                                    self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return(-2f64)
                                } else if {
                                    let source_ml_source1_comparison_operand13 =
                                        self::ml_array::optional_at(
                                            values,
                                            self::ml_array::number_index(end_2),
                                        );
                                    {
                                        let source_ml_source1_comparison_operand14 =
                                            String::from("}");
                                        match (
                                            source_ml_source1_comparison_operand13,
                                            source_ml_source1_comparison_operand14,
                                        ) {
                                            (Some(ml_left), ml_right) => ml_left == ml_right,
                                            _ => false,
                                        }
                                    }
                                } {
                                    self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return(end_2)
                                } else {
                                    self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return(-1f64)
                                }
                            }
                        }
                    } else if ((value == ")") || (value == "]")) || (value == "}") {
                        {
                            let previous = (self::ml_array::optional_at(
                                &stack,
                                self::ml_array::number_index((stack.len() as f64) - 1f64),
                            ))
                            .unwrap_or_else(|| String::new());
                            if (((value == ")") && (previous != "("))
                                || ((value == "]") && (previous != "[")))
                                || ((value == "}") && (previous != "{"))
                            {
                                self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return(-1f64)
                            } else {
                                {
                                    let stack_3 = self::ml_array::slice(&stack, 0f64, -1f64);
                                    {
                                        let at_3 = at + 1f64;
                                        {
                                            (stack, at) = (stack_3, at_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let at_4 = at + 1f64;
                            {
                                at = at_4;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustFunctionCompletionPresenceLoop5Result {
    MlTemporaryReadRustFunctionCompletionPresenceLoop5Done,
    MlTemporaryReadRustFunctionCompletionPresenceLoop5Return(f64),
}
