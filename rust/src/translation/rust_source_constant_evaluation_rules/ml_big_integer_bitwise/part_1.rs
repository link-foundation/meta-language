// Generated from complete js/src/translation/rust-source-constant-evaluation-rules.js. Do not edit by hand.
use super::ml_apply_big_integer_bitwise_operation_loop6;
use super::ml_render_big_integer_bitwise_expression_loop7;

/// JavaScript's Number operations that Rust's f64 does not share.
pub mod ml_number {
    /// ECMAScript `Number::toString`: the shortest decimal that reads back as
    /// the value, the even one on a tie, laid out as JavaScript prints it.
    #[must_use]
    pub fn js_number(x: f64) -> String {
        if x.is_nan() {
            return String::from("NaN");
        }
        if x.is_infinite() {
            return String::from(if x > 0.0 { "Infinity" } else { "-Infinity" });
        }
        if x == 0.0 {
            return String::from("0");
        }
        let (mut digits, n) = decimal(&format!("{:e}", x.abs()));
        // Rust's shortest digits round a tie away from zero; ECMAScript takes the even neighbour.
        let (exact, _) = decimal(&format!("{:.800e}", x.abs()));
        let rest = &exact[digits.len().min(exact.len())..];
        if digits.len() > 1 && rest.starts_with('5') && rest[1..].bytes().all(|b| b == b'0') {
            let last = digits.as_bytes()[digits.len() - 1] - b'0';
            let down = exact[..digits.len()].to_string();
            let other = if digits == down {
                increment(&down)
            } else {
                Some(down)
            };
            if let Some(other) = other {
                let back = format!("{}.{}e{}", &other[..1], &other[1..], n - 1);
                if last % 2 == 1 && back.parse::<f64>() == Ok(x.abs()) {
                    digits = other.trim_end_matches('0').to_string();
                }
            }
        }
        let k = digits.len() as i32;
        let body = if k <= n && n <= 21 {
            format!("{digits}{}", "0".repeat((n - k) as usize))
        } else if 0 < n && n <= 21 {
            format!("{}.{}", &digits[..n as usize], &digits[n as usize..])
        } else if -6 < n && n <= 0 {
            format!("0.{}{digits}", "0".repeat((-n) as usize))
        } else {
            let sign = if n - 1 < 0 { '-' } else { '+' };
            let power = (n - 1).abs();
            if k == 1 {
                format!("{digits}e{sign}{power}")
            } else {
                format!("{}.{}e{sign}{power}", &digits[..1], &digits[1..])
            }
        };
        if x < 0.0 { format!("-{body}") } else { body }
    }

    /// What console.log prints: -0 as "-0", where String(-0) is "0".
    #[must_use]
    pub fn js_console(x: f64) -> String {
        if x == 0.0 && x.is_sign_negative() {
            String::from("-0")
        } else {
            js_number(x)
        }
    }

    /// `SameValue`, as `Object.is` and `assert.strictEqual` compare: NaN equals
    /// NaN, and 0 differs from -0.
    #[must_use]
    pub const fn same_value(a: f64, b: f64) -> bool {
        if a.is_nan() || b.is_nan() {
            a.is_nan() && b.is_nan()
        } else {
            a.to_bits() == b.to_bits()
        }
    }

    /// A decimal digit string plus one, or None when it gains a digit.
    fn increment(digits: &str) -> Option<String> {
        let mut bytes = digits.as_bytes().to_vec();
        for index in (0..bytes.len()).rev() {
            if bytes[index] == b'9' {
                bytes[index] = b'0';
            } else {
                bytes[index] += 1;
                return String::from_utf8(bytes).ok();
            }
        }
        None
    }

    /// The significant digits without trailing zeros, and the exponent n with
    /// value 0.digits × 10^n, of Rust's {:e} text.
    fn decimal(text: &str) -> (String, i32) {
        let (mantissa, exponent) = text.split_once('e').expect("exponent");
        let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
        let digits = digits.trim_end_matches('0').to_string();
        let digits = if digits.is_empty() {
            String::from("0")
        } else {
            digits
        };
        (digits, exponent.parse::<i32>().expect("exponent") + 1)
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
pub fn read_big_integer_bitwise_operation_code(operation: &str) -> f64 {
    if operation == "conjoinBigIntegerBits" {
        0f64
    } else if operation == "disjoinBigIntegerBits" {
        1f64
    } else if operation == "excludeEqualBigIntegerBits" {
        2f64
    } else if operation == "shiftBigIntegerBitsLeft" {
        3f64
    } else if operation == "shiftBigIntegerBitsRight" {
        4f64
    } else if operation == "complementBigIntegerBits" {
        6f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_big_integer_bitwise_operation(operation: &str) -> String {
    if operation == "conjoinJavaScriptNumberBits" {
        String::from("conjoinBigIntegerBits")
    } else if operation == "disjoinJavaScriptNumberBits" {
        String::from("disjoinBigIntegerBits")
    } else if operation == "excludeEqualJavaScriptNumberBits" {
        String::from("excludeEqualBigIntegerBits")
    } else if operation == "shiftJavaScriptNumberBitsLeft" {
        String::from("shiftBigIntegerBitsLeft")
    } else if operation == "shiftJavaScriptNumberBitsRight" {
        String::from("shiftBigIntegerBitsRight")
    } else if operation == "complementJavaScriptNumberBits" {
        String::from("complementBigIntegerBits")
    } else if (self::read_big_integer_bitwise_operation_code(operation)) < 0f64 {
        String::new()
    } else {
        operation.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_big_integer_bitwise_arity(operation: &str) -> f64 {
    {
        let code = self::read_big_integer_bitwise_operation_code(operation);
        if code == 6f64 {
            1f64
        } else if code < 0f64 {
            0f64
        } else {
            2f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_big_integer_bitwise_operation(expression: &str, operation: &str) -> bool {
    (expression == "math") && ((self::read_big_integer_bitwise_arity(operation)) != 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_primitive_bitwise_domain(kinds: &[String]) -> String {
    if (kinds.len() as f64) == 0f64 {
        String::new()
    } else {
        {
            let number = true;
            {
                let integer = true;
                {
                    let ml_values1 = kinds.to_vec();
                    {
                        let ml_index1 = self::ml::Big::from_i128(0);
                        {
                            let ml_s1 = self::ml_read_primitive_bitwise_domain_loop2(
                                number,
                                integer,
                                &(ml_values1),
                                ml_index1,
                            );
                            match ml_s1 {
                                self::MlReadPrimitiveBitwiseDomainLoop2Result::MlReadPrimitiveBitwiseDomainLoop2Done(number_2, integer_2) => {
                                    if number_2 {
                                        String::from("Number")
                                    } else if integer_2 {
                                            String::from("BigInt")
                                        } else {
                                            String::new()
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
pub fn apply_big_integer_bitwise_operation(
    operation: f64,
    left: self::ml::Big,
    right: self::ml::Big,
) -> self::ml::Big {
    if operation == 6f64 {
        (self::ml::Big::from_i128(-1)).sub(&left)
    } else if (operation == 3f64) || (operation == 4f64) {
        if left == (self::ml::Big::from_i128(0)) {
            self::ml::Big::from_i128(0)
        } else {
            {
                let count = right.clone();
                {
                    let forward = operation == 3f64;
                    {
                        let ml_s2 = if count < (self::ml::Big::from_i128(0)) {
                            {
                                let count_2 = count.neg();
                                {
                                    let forward_2 = !forward;
                                    self::MlApplyBigIntegerBitwiseOperationJoin3::MlApplyBigIntegerBitwiseOperationJoin3Next(count_2, forward_2)
                                }
                            }
                        } else {
                            self::MlApplyBigIntegerBitwiseOperationJoin3::MlApplyBigIntegerBitwiseOperationJoin3Next(count.clone(), forward)
                        };
                        match ml_s2 {
                                self::MlApplyBigIntegerBitwiseOperationJoin3::MlApplyBigIntegerBitwiseOperationJoin3Next(count_3, forward_3) => {
                                    {
                                        let value = left.clone();
                                        {
                                            if forward_3 {
                                                {
                                                    let factor = self::ml::Big::from_i128(2);
                                                    {
                                                        self::ml_apply_big_integer_bitwise_operation_loop4(count_3.clone(), value.clone(), factor)
                                                    }
                                                }
                                            } else {
                                                {
                                                    self::ml_apply_big_integer_bitwise_operation_loop5(count_3.clone(), value.clone())
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
        assert!(
            !(((operation != 0f64) && (operation != 1f64)) && (operation != 2f64)),
            "{}",
            "an unknown BigInt bitwise operation"
        );

        {
            let negative_left = left < (self::ml::Big::from_i128(0));
            {
                let negative_right = right < (self::ml::Big::from_i128(0));
                {
                    let first = if negative_left {
                        (self::ml::Big::from_i128(-1)).sub(&left)
                    } else {
                        left.clone()
                    };
                    {
                        let second = if negative_right {
                            (self::ml::Big::from_i128(-1)).sub(&right)
                        } else {
                            right.clone()
                        };
                        {
                            let weight = self::ml::Big::from_i128(1);
                            {
                                let result = self::ml::Big::from_i128(0);
                                {
                                    let ml_s3 = self::ml_apply_big_integer_bitwise_operation_loop6(
                                        operation,
                                        negative_left,
                                        negative_right,
                                        first,
                                        second,
                                        weight,
                                        result,
                                    );
                                    match ml_s3 {
                                                self::MlApplyBigIntegerBitwiseOperationLoop6Result::MlApplyBigIntegerBitwiseOperationLoop6Done(weight_2, result_2) => {
                                                    {
                                                        let negative = if operation == 0f64 {
                                                            negative_left && negative_right
                                                        } else if operation == 1f64 {
                                                                negative_left || negative_right
                                                            } else {
                                                                negative_left != negative_right
                                                            };
                                                        if negative {
                                                            result_2.sub(&weight_2)
                                                        } else {
                                                            result_2.clone()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_big_integer_bitwise_expression(
    target: &str,
    operation: &str,
    arguments_: &[String],
) -> String {
    {
        let code = self::read_big_integer_bitwise_operation_code(operation);
        if (code < 0f64)
            || ((arguments_.len() as f64) != (self::read_big_integer_bitwise_arity(operation)))
        {
            String::new()
        } else {
            {
                let left =
                    (self::ml_array::optional_at(arguments_, self::ml_array::number_index(0f64)))
                        .unwrap_or_else(|| String::new());
                {
                    let right = (self::ml_array::optional_at(
                        arguments_,
                        self::ml_array::number_index(1f64),
                    ))
                    .unwrap_or_else(|| String::new());
                    if target == "Rocq" {
                        {
                            let operation_name = if code == 0f64 {
                                String::from("Z.land")
                            } else if code == 1f64 {
                                String::from("Z.lor")
                            } else if code == 2f64 {
                                String::from("Z.lxor")
                            } else if code == 3f64 {
                                String::from("Z.shiftl")
                            } else if code == 4f64 {
                                String::from("Z.shiftr")
                            } else {
                                String::from("Z.lnot")
                            };
                            format!(
                                "({operation_name} {left}{})",
                                if code == 6f64 {
                                    String::new()
                                } else {
                                    format!("{}{}", (String::from(" ")), right)
                                }
                            )
                        }
                    } else if target == "Lean" {
                        if code == 6f64 {
                            format!("(-1 - ({left}))")
                        } else {
                            {
                                let text = format!("{left} {right}");
                                {
                                    let suffix = 0f64;
                                    {
                                        let suffix_2 =
                                            self::ml_render_big_integer_bitwise_expression_loop7(
                                                &(text),
                                                suffix,
                                            );
                                        {
                                            let a = format!(
                                                "ml_integer_left{}",
                                                self::ml_number::js_number(suffix_2)
                                            );
                                            {
                                                let b = format!(
                                                    "ml_integer_right{}",
                                                    self::ml_number::js_number(suffix_2)
                                                );
                                                {
                                                    let am = format!("(Int.toNat (-1 - {a}))");
                                                    {
                                                        let bm = format!("(Int.toNat (-1 - {b}))");
                                                        {
                                                            let ap = format!("(Int.toNat {a})");
                                                            {
                                                                let bp = format!("(Int.toNat {b})");
                                                                {
                                                                    let prefix = format!(
                                                                        "(let {a} : Int := {left}; let {b} : Int := {right}; "
                                                                    );
                                                                    if (code == 3f64)
                                                                        || (code == 4f64)
                                                                    {
                                                                        {
                                                                            let count = format!(
                                                                                "(Int.toNat (if {b} < 0 then -{b} else {b}))"
                                                                            );
                                                                            {
                                                                                let shift_left = format!(
                                                                                    "({a} * Int.ofNat (Nat.shiftLeft 1 {count}))"
                                                                                );
                                                                                {
                                                                                    let shift_right = format!(
                                                                                        "(if {a} < 0 then -1 - Int.ofNat (Nat.shiftRight {am} {count}) else Int.ofNat (Nat.shiftRight {ap} {count}))"
                                                                                    );
                                                                                    format!(
                                                                                        "{prefix}if {a} = 0 then 0 else if {b} < 0 then {} else {})",
                                                                                        if code
                                                                                            == 3f64
                                                                                        {
                                                                                            shift_right.clone()
                                                                                        } else {
                                                                                            shift_left.clone()
                                                                                        },
                                                                                        if code
                                                                                            == 3f64
                                                                                        {
                                                                                            shift_left.clone()
                                                                                        } else {
                                                                                            shift_right.clone()
                                                                                        }
                                                                                    )
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let difference =
                                                                                String::from(
                                                                                    "(Nat.bitwise (fun ml_bit_left ml_bit_right => ml_bit_left && !ml_bit_right) ",
                                                                                );
                                                                            {
                                                                                let both = if code
                                                                                    == 0f64
                                                                                {
                                                                                    format!(
                                                                                        "(-1 - Int.ofNat (Nat.lor {am} {bm}))"
                                                                                    )
                                                                                } else if code
                                                                                    == 1f64
                                                                                {
                                                                                    format!(
                                                                                        "(-1 - Int.ofNat (Nat.land {am} {bm}))"
                                                                                    )
                                                                                } else {
                                                                                    format!(
                                                                                        "(Int.ofNat (Nat.xor {am} {bm}))"
                                                                                    )
                                                                                };
                                                                                {
                                                                                    let left_negative = if code == 0f64 {
                                                                                            format!("(Int.ofNat {difference}{bp} {am}))")
                                                                                        } else if code == 1f64 {
                                                                                                format!("(-1 - Int.ofNat {difference}{am} {bp}))")
                                                                                            } else {
                                                                                                format!("(-1 - Int.ofNat (Nat.xor {am} {bp}))")
                                                                                            };
                                                                                    {
                                                                                        let right_negative = if code == 0f64 {
                                                                                                format!("(Int.ofNat {difference}{ap} {bm}))")
                                                                                            } else if code == 1f64 {
                                                                                                    format!("(-1 - Int.ofNat {difference}{bm} {ap}))")
                                                                                                } else {
                                                                                                    format!("(-1 - Int.ofNat (Nat.xor {ap} {bm}))")
                                                                                                };
                                                                                        {
                                                                                            let positive = format!("(Int.ofNat ({} {ap} {bp}))", if code == 0f64 {
                                                                                                    String::from("Nat.land")
                                                                                                } else if code == 1f64 {
                                                                                                        String::from("Nat.lor")
                                                                                                    } else {
                                                                                                        String::from("Nat.xor")
                                                                                                    });
                                                                                            format!(
                                                                                                "{prefix}if {a} < 0 then (if {b} < 0 then {both} else {left_negative}) else (if {b} < 0 then {right_negative} else {positive}))"
                                                                                            )
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
                    } else if target == "Rust" {
                        format!(
                            "crate::ml_big_integer_bitwise::apply_big_integer_bitwise_operation({}f64, {left}, {})",
                            self::ml_number::js_number(code),
                            if code == 6f64 {
                                String::from("ml::Big::zero()")
                            } else {
                                right.clone()
                            }
                        )
                    } else if target == "JavaScript" {
                        if code == 6f64 {
                            format!("(~({left}))")
                        } else {
                            {
                                let symbol = if code == 0f64 {
                                    String::from("&")
                                } else if code == 1f64 {
                                    String::from("|")
                                } else if code == 2f64 {
                                    String::from("^")
                                } else if code == 3f64 {
                                    String::from("<<")
                                } else {
                                    String::from(">>")
                                };
                                format!("(({left}) {symbol} ({right}))")
                            }
                        }
                    } else {
                        String::new()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_big_integer_bitwise_statement() -> String {
    String::from(
        "BigInt bitwise operations use exact unbounded integers with infinite sign extension, negative shift reversal and arithmetic right shifts; operands are evaluated once in source order without Number coercion or modulo-32 shift counts. Native source and the emitted integer runtime are generated from the same checked definition.",
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadPrimitiveBitwiseDomainLoop2Result {
    MlReadPrimitiveBitwiseDomainLoop2Done(bool, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_primitive_bitwise_domain_loop2(
    mut number: bool,
    mut integer: bool,
    ml_values1: &[String],
    mut ml_index1: self::ml::Big,
) -> self::MlReadPrimitiveBitwiseDomainLoop2Result {
    loop {
        if ml_index1 < (self::ml::Big::from_u128(ml_values1.len() as u128)) {
            {
                let kind = self::ml_array::at(ml_values1, ml_index1.to_index());
                {
                    let number_2 = number && (kind == "float");
                    {
                        let integer_2 = integer && ((kind == "int") || (kind == "nat"));
                        {
                            let ml_index1_2 = ml_index1.add(&self::ml::Big::from_i128(1));
                            {
                                (number, integer, ml_index1) = (number_2, integer_2, ml_index1_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlReadPrimitiveBitwiseDomainLoop2Result::MlReadPrimitiveBitwiseDomainLoop2Done(
                number, integer,
            )
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_apply_big_integer_bitwise_operation_loop4(
    mut count: self::ml::Big,
    mut value: self::ml::Big,
    mut factor: self::ml::Big,
) -> self::ml::Big {
    loop {
        if count > (self::ml::Big::from_i128(0)) {
            {
                let value_3 = if (self::ml::divide(
                    &count,
                    &self::ml::Big::from_i128(2),
                    "trunc",
                    Some("Division by zero"),
                    true,
                )) == (self::ml::Big::from_i128(1))
                {
                    { value.mul(&factor) }
                } else {
                    value.clone()
                };
                {
                    let count_2 = self::ml::divide(
                        &count,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        false,
                    );
                    if count_2 > (self::ml::Big::from_i128(0)) {
                        {
                            let factor_2 = factor.mul(&factor);
                            {
                                (count, value, factor) =
                                    (count_2.clone(), value_3.clone(), factor_2);
                                continue;
                            }
                        }
                    }
                    {
                        {
                            (count, value) = (count_2.clone(), value_3.clone());
                            continue;
                        }
                    }
                }
            }
        }
        return { value.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_apply_big_integer_bitwise_operation_loop5(
    mut count: self::ml::Big,
    mut value: self::ml::Big,
) -> self::ml::Big {
    loop {
        if ((count > (self::ml::Big::from_i128(0))) && (value != (self::ml::Big::from_i128(0))))
            && (value != (self::ml::Big::from_i128(-1)))
        {
            {
                let value_2 = if value < (self::ml::Big::from_i128(0)) {
                    self::ml::divide(
                        &value.sub(&self::ml::Big::from_i128(1)),
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        false,
                    )
                } else {
                    self::ml::divide(
                        &value,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        false,
                    )
                };
                {
                    let count_2 = count.sub(&self::ml::Big::from_i128(1));
                    {
                        (count, value) = (count_2, value_2);
                        continue;
                    }
                }
            }
        }
        return { value.clone() };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlApplyBigIntegerBitwiseOperationJoin3 {
    MlApplyBigIntegerBitwiseOperationJoin3Next(self::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlApplyBigIntegerBitwiseOperationLoop6Result {
    MlApplyBigIntegerBitwiseOperationLoop6Done(self::ml::Big, self::ml::Big),
}
