pub mod ml_big_integer_bitwise {
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]

use crate::ml;

/// JavaScript's Number operations that Rust's f64 does not share.
pub mod ml_number {
    /// ECMAScript Number::toString: the shortest decimal that reads back as
    /// the value, the even one on a tie, laid out as JavaScript prints it.
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
            let other = if digits == down { increment(&down) } else { Some(down) };
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
        if x < 0.0 {
            format!("-{body}")
        } else {
            body
        }
    }

    /// What console.log prints: -0 as "-0", where String(-0) is "0".
    pub fn js_console(x: f64) -> String {
        if x == 0.0 && x.is_sign_negative() {
            String::from("-0")
        } else {
            js_number(x)
        }
    }

    /// SameValue, as Object.is and assert.strictEqual compare: NaN equals
    /// NaN, and 0 differs from -0.
    pub fn same_value(a: f64, b: f64) -> bool {
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
        let digits = if digits.is_empty() { String::from("0") } else { digits };
        (digits, exponent.parse::<i32>().expect("exponent") + 1)
    }
}

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        match index.and_then(|index| values.get(index)) {
            Some(value) => value.clone(),
            None => panic!("array index out of range"),
        }
    }

    /// The index a Number names: a non-negative integer, -0 included.
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9007199254740992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    fn integer(value: f64) -> f64 { if value.is_nan() { 0.0 } else { value.trunc() } }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 { (length as f64 + value).max(0.0) as usize } else { value.min(length as f64) as usize }
    }
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len()); let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> f64 {
        if values.is_empty() { return -1.0; }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) { return -1.0; }
            let start = if from < 0.0 { (values.len() as f64 + from) as usize } else { from.min((values.len() - 1) as f64) as usize };
            (0..=start).rev().find(|&index| equal(&values[index], search)).map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 { (values.len() as f64 + from).max(0.0) as usize } else { from.min(values.len() as f64) as usize };
            (start..values.len()).find(|&index| equal(&values[index], search)).map_or(-1.0, |index| index as f64)
        }
    }
    pub fn includes<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values { if !out.iter().any(|previous| equal(previous, &value)) { out.push(value); } }
        out
    }
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) { out[index].1 = value; }
            else { out.push((key, value)); }
        }
        out
    }
}

pub fn read_big_integer_bitwise_operation_code(operation: String) -> f64 {
    if (operation == "conjoinBigIntegerBits") {
        0f64
    } else {
        if (operation == "disjoinBigIntegerBits") {
            1f64
        } else {
            if (operation == "excludeEqualBigIntegerBits") {
                2f64
            } else {
                if (operation == "shiftBigIntegerBitsLeft") {
                    3f64
                } else {
                    if (operation == "shiftBigIntegerBitsRight") {
                        4f64
                    } else {
                        if (operation == "complementBigIntegerBits") {
                            6f64
                        } else {
                            (-1f64)
                        }
                    }
                }
            }
        }
    }
}

pub fn read_big_integer_bitwise_operation(operation: String) -> String {
    if (operation == "conjoinJavaScriptNumberBits") {
        String::from("conjoinBigIntegerBits")
    } else {
        if (operation == "disjoinJavaScriptNumberBits") {
            String::from("disjoinBigIntegerBits")
        } else {
            if (operation == "excludeEqualJavaScriptNumberBits") {
                String::from("excludeEqualBigIntegerBits")
            } else {
                if (operation == "shiftJavaScriptNumberBitsLeft") {
                    String::from("shiftBigIntegerBitsLeft")
                } else {
                    if (operation == "shiftJavaScriptNumberBitsRight") {
                        String::from("shiftBigIntegerBitsRight")
                    } else {
                        if (operation == "complementJavaScriptNumberBits") {
                            String::from("complementBigIntegerBits")
                        } else {
                            if ((self::read_big_integer_bitwise_operation_code(operation.clone())) < (0f64)) {
                                String::from("")
                            } else {
                                operation.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_big_integer_bitwise_arity(operation: String) -> f64 {
    {
        let code = self::read_big_integer_bitwise_operation_code(operation.clone());
        if (code == (6f64)) {
            1f64
        } else {
            if (code < (0f64)) {
                0f64
            } else {
                2f64
            }
        }
    }
}

pub fn accept_linked_big_integer_bitwise_operation(expression: String, operation: String) -> bool {
    ((expression == "math") && ((self::read_big_integer_bitwise_arity(operation.clone())) != (0f64)))
}

pub fn read_primitive_bitwise_domain(kinds: Vec<String>) -> String {
    if (((kinds.len() as f64)) == (0f64)) {
        String::from("")
    } else {
        {
            let number = true;
            {
                let integer = true;
                {
                    let ml_values1 = kinds;
                    {
                        let ml_index1 = crate::ml::Big::from_i128(0);
                        {
                            let ml_s1 = self::ml_read_primitive_bitwise_domain_loop2(number, integer, ml_values1, ml_index1.clone());
                            match ml_s1.clone() {
                                self::MlReadPrimitiveBitwiseDomainLoop2Result::MlReadPrimitiveBitwiseDomainLoop2Done(number_2, integer_2) => {
                                    if number_2 {
                                        String::from("Number")
                                    } else {
                                        if integer_2 {
                                            String::from("BigInt")
                                        } else {
                                            String::from("")
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

pub fn apply_big_integer_bitwise_operation(operation: f64, left: crate::ml::Big, right: crate::ml::Big) -> crate::ml::Big {
    if (operation == (6f64)) {
        (crate::ml::Big::from_i128(-1)).sub(&left)
    } else {
        if ((operation == (3f64)) || (operation == (4f64))) {
            if (left == (crate::ml::Big::from_i128(0))) {
                crate::ml::Big::from_i128(0)
            } else {
                {
                    let count = right.clone();
                    {
                        let forward = (operation == (3f64));
                        {
                            let ml_s2 = if (count < (crate::ml::Big::from_i128(0))) {
                                {
                                    let count_2 = count.neg();
                                    {
                                        let forward_2 = !forward;
                                        self::MlApplyBigIntegerBitwiseOperationJoin3::MlApplyBigIntegerBitwiseOperationJoin3Next(count_2.clone(), forward_2)
                                    }
                                }
                            } else {
                                self::MlApplyBigIntegerBitwiseOperationJoin3::MlApplyBigIntegerBitwiseOperationJoin3Next(count.clone(), forward)
                            };
                            match ml_s2.clone() {
                                self::MlApplyBigIntegerBitwiseOperationJoin3::MlApplyBigIntegerBitwiseOperationJoin3Next(count_3, forward_3) => {
                                    {
                                        let value = left.clone();
                                        {
                                            let value_4 = if forward_3 {
                                                {
                                                    let factor = crate::ml::Big::from_i128(2);
                                                    {
                                                        let value_2 = self::ml_apply_big_integer_bitwise_operation_loop4(count_3.clone(), value.clone(), factor.clone());
                                                        value_2.clone()
                                                    }
                                                }
                                            } else {
                                                {
                                                    let value_3 = self::ml_apply_big_integer_bitwise_operation_loop5(count_3.clone(), value.clone());
                                                    value_3.clone()
                                                }
                                            };
                                            value_4.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if (((operation != (0f64)) && (operation != (1f64))) && (operation != (2f64))) {
                panic!("{}", "an unknown BigInt bitwise operation")
            } else {
                {
                    let negative_left = (left < (crate::ml::Big::from_i128(0)));
                    {
                        let negative_right = (right < (crate::ml::Big::from_i128(0)));
                        {
                            let first = if negative_left {
                                (crate::ml::Big::from_i128(-1)).sub(&left)
                            } else {
                                left.clone()
                            };
                            {
                                let second = if negative_right {
                                    (crate::ml::Big::from_i128(-1)).sub(&right)
                                } else {
                                    right.clone()
                                };
                                {
                                    let weight = crate::ml::Big::from_i128(1);
                                    {
                                        let result = crate::ml::Big::from_i128(0);
                                        {
                                            let ml_s3 = self::ml_apply_big_integer_bitwise_operation_loop6(operation, negative_left, negative_right, first.clone(), second.clone(), weight.clone(), result.clone());
                                            match ml_s3.clone() {
                                                self::MlApplyBigIntegerBitwiseOperationLoop6Result::MlApplyBigIntegerBitwiseOperationLoop6Done(weight_2, result_2) => {
                                                    {
                                                        let negative = if (operation == (0f64)) {
                                                            (negative_left && negative_right)
                                                        } else {
                                                            if (operation == (1f64)) {
                                                                (negative_left || negative_right)
                                                            } else {
                                                                (negative_left != negative_right)
                                                            }
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
    }
}

pub fn render_big_integer_bitwise_expression(target: String, operation: String, arguments_: Vec<String>) -> String {
    {
        let code = self::read_big_integer_bitwise_operation_code(operation.clone());
        if ((code < (0f64)) || (((arguments_.len() as f64)) != (self::read_big_integer_bitwise_arity(operation.clone())))) {
            String::from("")
        } else {
            {
                let left = (self::ml_array::optional_at(&arguments_, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| String::from(""));
                {
                    let right = (self::ml_array::optional_at(&arguments_, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| String::from(""));
                    if (target == "Rocq") {
                        {
                            let operation_name = if (code == (0f64)) {
                                String::from("Z.land")
                            } else {
                                if (code == (1f64)) {
                                    String::from("Z.lor")
                                } else {
                                    if (code == (2f64)) {
                                        String::from("Z.lxor")
                                    } else {
                                        if (code == (3f64)) {
                                            String::from("Z.shiftl")
                                        } else {
                                            if (code == (4f64)) {
                                                String::from("Z.shiftr")
                                            } else {
                                                String::from("Z.lnot")
                                            }
                                        }
                                    }
                                }
                            };
                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), operation_name)), (String::from(" ")))), left)), (if (code == (6f64)) {
                                String::from("")
                            } else {
                                format!("{}{}", (String::from(" ")), right)
                            }))), (String::from(")")))
                        }
                    } else {
                        if (target == "Lean") {
                            if (code == (6f64)) {
                                format!("{}{}", (format!("{}{}", (String::from("(-1 - (")), left)), (String::from("))")))
                            } else {
                                {
                                    let text = format!("{}{}", (format!("{}{}", left, (String::from(" ")))), right);
                                    {
                                        let suffix = 0f64;
                                        {
                                            let suffix_2 = self::ml_render_big_integer_bitwise_expression_loop7(text.clone(), suffix);
                                            {
                                                let a = format!("{}{}", (String::from("ml_integer_left")), (self::ml_number::js_number(suffix_2)));
                                                {
                                                    let b = format!("{}{}", (String::from("ml_integer_right")), (self::ml_number::js_number(suffix_2)));
                                                    {
                                                        let am = format!("{}{}", (format!("{}{}", (String::from("(Int.toNat (-1 - ")), a)), (String::from("))")));
                                                        {
                                                            let bm = format!("{}{}", (format!("{}{}", (String::from("(Int.toNat (-1 - ")), b)), (String::from("))")));
                                                            {
                                                                let ap = format!("{}{}", (format!("{}{}", (String::from("(Int.toNat ")), a)), (String::from(")")));
                                                                {
                                                                    let bp = format!("{}{}", (format!("{}{}", (String::from("(Int.toNat ")), b)), (String::from(")")));
                                                                    {
                                                                        let prefix = format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(let ")), a)), (String::from(" : Int := ")))), left)), (String::from("; let ")))), b)), (String::from(" : Int := ")))), right)), (String::from("; ")));
                                                                        if ((code == (3f64)) || (code == (4f64))) {
                                                                            {
                                                                                let count = format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(Int.toNat (if ")), b)), (String::from(" < 0 then -")))), b)), (String::from(" else ")))), b)), (String::from("))")));
                                                                                {
                                                                                    let shift_left = format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), a)), (String::from(" * Int.ofNat (Nat.shiftLeft 1 ")))), count)), (String::from("))")));
                                                                                    {
                                                                                        let shift_right = format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(if ")), a)), (String::from(" < 0 then -1 - Int.ofNat (Nat.shiftRight ")))), am)), (String::from(" ")))), count)), (String::from(") else Int.ofNat (Nat.shiftRight ")))), ap)), (String::from(" ")))), count)), (String::from("))")));
                                                                                        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", prefix, (String::from("if ")))), a)), (String::from(" = 0 then 0 else if ")))), b)), (String::from(" < 0 then ")))), (if (code == (3f64)) {
                                                                                            shift_right.clone()
                                                                                        } else {
                                                                                            shift_left.clone()
                                                                                        }))), (String::from(" else ")))), (if (code == (3f64)) {
                                                                                            shift_left.clone()
                                                                                        } else {
                                                                                            shift_right.clone()
                                                                                        }))), (String::from(")")))
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            {
                                                                                let difference = String::from("(Nat.bitwise (fun ml_bit_left ml_bit_right => ml_bit_left && !ml_bit_right) ");
                                                                                {
                                                                                    let both = if (code == (0f64)) {
                                                                                        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(-1 - Int.ofNat (Nat.lor ")), am)), (String::from(" ")))), bm)), (String::from("))")))
                                                                                    } else {
                                                                                        if (code == (1f64)) {
                                                                                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(-1 - Int.ofNat (Nat.land ")), am)), (String::from(" ")))), bm)), (String::from("))")))
                                                                                        } else {
                                                                                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(Int.ofNat (Nat.xor ")), am)), (String::from(" ")))), bm)), (String::from("))")))
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let left_negative = if (code == (0f64)) {
                                                                                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(Int.ofNat ")), difference)), bp)), (String::from(" ")))), am)), (String::from("))")))
                                                                                        } else {
                                                                                            if (code == (1f64)) {
                                                                                                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(-1 - Int.ofNat ")), difference)), am)), (String::from(" ")))), bp)), (String::from("))")))
                                                                                            } else {
                                                                                                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(-1 - Int.ofNat (Nat.xor ")), am)), (String::from(" ")))), bp)), (String::from("))")))
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let right_negative = if (code == (0f64)) {
                                                                                                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(Int.ofNat ")), difference)), ap)), (String::from(" ")))), bm)), (String::from("))")))
                                                                                            } else {
                                                                                                if (code == (1f64)) {
                                                                                                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(-1 - Int.ofNat ")), difference)), bm)), (String::from(" ")))), ap)), (String::from("))")))
                                                                                                } else {
                                                                                                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(-1 - Int.ofNat (Nat.xor ")), ap)), (String::from(" ")))), bm)), (String::from("))")))
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let positive = format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(Int.ofNat (")), (if (code == (0f64)) {
                                                                                                    String::from("Nat.land")
                                                                                                } else {
                                                                                                    if (code == (1f64)) {
                                                                                                        String::from("Nat.lor")
                                                                                                    } else {
                                                                                                        String::from("Nat.xor")
                                                                                                    }
                                                                                                }))), (String::from(" ")))), ap)), (String::from(" ")))), bp)), (String::from("))")));
                                                                                                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", prefix, (String::from("if ")))), a)), (String::from(" < 0 then (if ")))), b)), (String::from(" < 0 then ")))), both)), (String::from(" else ")))), left_negative)), (String::from(") else (if ")))), b)), (String::from(" < 0 then ")))), right_negative)), (String::from(" else ")))), positive)), (String::from("))")))
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
                            if (target == "Rust") {
                                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_big_integer_bitwise::apply_big_integer_bitwise_operation(")), (self::ml_number::js_number(code)))), (String::from("f64, ")))), left)), (String::from(", ")))), (if (code == (6f64)) {
                                    String::from("ml::Big::zero()")
                                } else {
                                    right.clone()
                                }))), (String::from(")")))
                            } else {
                                if (target != "JavaScript") {
                                    String::from("")
                                } else {
                                    if (code == (6f64)) {
                                        format!("{}{}", (format!("{}{}", (String::from("(~(")), left)), (String::from("))")))
                                    } else {
                                        {
                                            let symbol = if (code == (0f64)) {
                                                String::from("&")
                                            } else {
                                                if (code == (1f64)) {
                                                    String::from("|")
                                                } else {
                                                    if (code == (2f64)) {
                                                        String::from("^")
                                                    } else {
                                                        if (code == (3f64)) {
                                                            String::from("<<")
                                                        } else {
                                                            String::from(">>")
                                                        }
                                                    }
                                                }
                                            };
                                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("((")), left)), (String::from(") ")))), symbol)), (String::from(" (")))), right)), (String::from("))")))
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

pub fn read_big_integer_bitwise_statement() -> String {
    String::from("BigInt bitwise operations use exact unbounded integers with infinite sign extension, negative shift reversal and arithmetic right shifts; operands are evaluated once in source order without Number coercion or modulo-32 shift counts. Native source and the emitted integer runtime are generated from the same checked definition.")
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadPrimitiveBitwiseDomainLoop2Result {
    MlReadPrimitiveBitwiseDomainLoop2Done(bool, bool),
}

pub fn ml_read_primitive_bitwise_domain_loop2(mut number: bool, mut integer: bool, mut ml_values1: Vec<String>, mut ml_index1: crate::ml::Big) -> self::MlReadPrimitiveBitwiseDomainLoop2Result {
    loop {
        return if (ml_index1 < (crate::ml::Big::from_u128(ml_values1.len() as u128))) {
            {
                let kind = self::ml_array::at(&ml_values1, ml_index1.to_index());
                {
                    let number_2 = (number && (kind == "float"));
                    {
                        let integer_2 = (integer && ((kind == "int") || (kind == "nat")));
                        {
                            let ml_index1_2 = ml_index1.add(&crate::ml::Big::from_i128(1));
                            {
                                (number, integer, ml_index1) = (number_2, integer_2, ml_index1_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadPrimitiveBitwiseDomainLoop2Result::MlReadPrimitiveBitwiseDomainLoop2Done(number, integer)
        };
    }
}

pub fn ml_apply_big_integer_bitwise_operation_loop4(mut count: crate::ml::Big, mut value: crate::ml::Big, mut factor: crate::ml::Big) -> crate::ml::Big {
    loop {
        return if (count > (crate::ml::Big::from_i128(0))) {
            {
                let value_3 = if ((crate::ml::divide(&count, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (crate::ml::Big::from_i128(1))) {
                    {
                        let value_2 = value.mul(&factor);
                        value_2.clone()
                    }
                } else {
                    value.clone()
                };
                {
                    let count_2 = crate::ml::divide(&count, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (count_2 > (crate::ml::Big::from_i128(0))) {
                        {
                            let factor_2 = factor.mul(&factor);
                            {
                                (count, value, factor) = (count_2.clone(), value_3.clone(), factor_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (count, value) = (count_2.clone(), value_3.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

pub fn ml_apply_big_integer_bitwise_operation_loop5(mut count: crate::ml::Big, mut value: crate::ml::Big) -> crate::ml::Big {
    loop {
        return if (((count > (crate::ml::Big::from_i128(0))) && (value != (crate::ml::Big::from_i128(0)))) && (value != (crate::ml::Big::from_i128(-1)))) {
            {
                let value_2 = if (value < (crate::ml::Big::from_i128(0))) {
                    crate::ml::divide(&value.sub(&crate::ml::Big::from_i128(1)), &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
                } else {
                    crate::ml::divide(&value, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
                };
                {
                    let count_2 = count.sub(&crate::ml::Big::from_i128(1));
                    {
                        (count, value) = (count_2.clone(), value_2.clone());
                        continue;
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlApplyBigIntegerBitwiseOperationJoin3 {
    MlApplyBigIntegerBitwiseOperationJoin3Next(crate::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlApplyBigIntegerBitwiseOperationLoop6Result {
    MlApplyBigIntegerBitwiseOperationLoop6Done(crate::ml::Big, crate::ml::Big),
}

pub fn ml_apply_big_integer_bitwise_operation_loop6(mut operation: f64, mut negative_left: bool, mut negative_right: bool, mut first: crate::ml::Big, mut second: crate::ml::Big, mut weight: crate::ml::Big, mut result: crate::ml::Big) -> self::MlApplyBigIntegerBitwiseOperationLoop6Result {
    loop {
        return if ((first > (crate::ml::Big::from_i128(0))) || (second > (crate::ml::Big::from_i128(0)))) {
            {
                let a = if negative_left {
                    (crate::ml::Big::from_i128(1)).sub(&crate::ml::divide(&first, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true))
                } else {
                    crate::ml::divide(&first, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)
                };
                {
                    let b = if negative_right {
                        (crate::ml::Big::from_i128(1)).sub(&crate::ml::divide(&second, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true))
                    } else {
                        crate::ml::divide(&second, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)
                    };
                    {
                        let selected = if (operation == (0f64)) {
                            ((a == (crate::ml::Big::from_i128(1))) && (b == (crate::ml::Big::from_i128(1))))
                        } else {
                            if (operation == (1f64)) {
                                ((a == (crate::ml::Big::from_i128(1))) || (b == (crate::ml::Big::from_i128(1))))
                            } else {
                                (a != b)
                            }
                        };
                        {
                            let result_3 = if selected {
                                {
                                    let result_2 = result.add(&weight);
                                    result_2.clone()
                                }
                            } else {
                                result.clone()
                            };
                            {
                                let first_2 = crate::ml::divide(&first, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                                {
                                    let second_2 = crate::ml::divide(&second, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                                    {
                                        let weight_2 = weight.mul(&crate::ml::Big::from_i128(2));
                                        {
                                            (first, second, weight, result) = (first_2.clone(), second_2.clone(), weight_2.clone(), result_3.clone());
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
            self::MlApplyBigIntegerBitwiseOperationLoop6Result::MlApplyBigIntegerBitwiseOperationLoop6Done(weight.clone(), result.clone())
        };
    }
}

pub fn ml_render_big_integer_bitwise_expression_loop7(mut text: String, mut suffix: f64) -> f64 {
    loop {
        return if (text.contains((format!("{}{}", (String::from("ml_integer_left")), (self::ml_number::js_number(suffix)))).as_str()) || text.contains((format!("{}{}", (String::from("ml_integer_right")), (self::ml_number::js_number(suffix)))).as_str())) {
            {
                let suffix_2 = (suffix + 1f64);
                {
                    suffix = suffix_2;
                    continue;
                }
            }
        } else {
            suffix
        };
    }
}
}
