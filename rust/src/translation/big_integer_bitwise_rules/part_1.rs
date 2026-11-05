// Generated from complete js/src/translation/big-integer-bitwise-rules.js. Do not edit by hand.
use super::MlApplyBigIntegerBitwiseOperationJoin3;
use super::MlApplyBigIntegerBitwiseOperationLoop6Result;
use super::ml_apply_big_integer_bitwise_operation_loop4;
use super::ml_apply_big_integer_bitwise_operation_loop5;
use super::ml_apply_big_integer_bitwise_operation_loop6;
use super::ml_render_big_integer_bitwise_expression_loop7;

/// Unbounded integers for the portable core's naturals and integers.
pub mod ml {
    use std::cmp::Ordering;
    use std::fmt;

    /// Sign and magnitude; the magnitude is little-endian base 2^32 without
    /// leading zero limbs, and zero is never negative.
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Big {
        negative: bool,
        magnitude: Vec<u32>,
    }

    fn trim(mut magnitude: Vec<u32>) -> Vec<u32> {
        while magnitude.last() == Some(&0) {
            magnitude.pop();
        }
        magnitude
    }

    fn compare_magnitude(a: &[u32], b: &[u32]) -> Ordering {
        a.len().cmp(&b.len()).then_with(|| a.iter().rev().cmp(b.iter().rev()))
    }

    fn add_magnitude(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut out = Vec::with_capacity(a.len().max(b.len()) + 1);
        let mut carry = 0u64;
        for index in 0..a.len().max(b.len()) {
            let sum = u64::from(*a.get(index).unwrap_or(&0)) + u64::from(*b.get(index).unwrap_or(&0)) + carry;
            out.push(sum as u32);
            carry = sum >> 32;
        }
        if carry > 0 {
            out.push(carry as u32);
        }
        out
    }

    /// |a| - |b| where |a| >= |b|.
    fn sub_magnitude(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut out = Vec::with_capacity(a.len());
        let mut borrow = 0i64;
        for (index, limb) in a.iter().enumerate() {
            let mut difference = i64::from(*limb) - i64::from(*b.get(index).unwrap_or(&0)) - borrow;
            borrow = 0;
            if difference < 0 {
                difference += 1 << 32;
                borrow = 1;
            }
            out.push(difference as u32);
        }
        trim(out)
    }

    fn mul_magnitude(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut out = vec![0u32; a.len() + b.len()];
        for (i, x) in a.iter().enumerate() {
            let mut carry = 0u64;
            for (j, y) in b.iter().enumerate() {
                let product = u64::from(out[i + j]) + u64::from(*x) * u64::from(*y) + carry;
                out[i + j] = product as u32;
                carry = product >> 32;
            }
            let mut index = i + b.len();
            while carry > 0 {
                let sum = u64::from(out[index]) + carry;
                out[index] = sum as u32;
                carry = sum >> 32;
                index += 1;
            }
        }
        trim(out)
    }

    /// Quotient and remainder of magnitudes; the divisor is not zero.
    fn divmod_magnitude(a: &[u32], b: &[u32]) -> (Vec<u32>, Vec<u32>) {
        let mut quotient = vec![0u32; a.len()];
        if b.len() == 1 {
            let divisor = u64::from(b[0]);
            let mut remainder = 0u64;
            for index in (0..a.len()).rev() {
                let current = (remainder << 32) | u64::from(a[index]);
                quotient[index] = (current / divisor) as u32;
                remainder = current % divisor;
            }
            return (trim(quotient), trim(vec![remainder as u32]));
        }
        let mut remainder: Vec<u32> = Vec::new();
        for bit in (0..a.len() * 32).rev() {
            let mut carry = (a[bit / 32] >> (bit % 32)) & 1;
            for limb in &mut remainder {
                let next = *limb >> 31;
                *limb = (*limb << 1) | carry;
                carry = next;
            }
            if carry != 0 {
                remainder.push(carry);
            }
            if compare_magnitude(&remainder, b) != Ordering::Less {
                remainder = sub_magnitude(&remainder, b);
                quotient[bit / 32] |= 1 << (bit % 32);
            }
        }
        (trim(quotient), remainder)
    }

    impl Big {
        fn from_parts(negative: bool, magnitude: Vec<u32>) -> Self {
            let magnitude = trim(magnitude);
            Self { negative: negative && !magnitude.is_empty(), magnitude }
        }

        #[must_use]
        pub fn zero() -> Self {
            Self::from_parts(false, Vec::new())
        }

        #[must_use]
        pub fn from_i128(value: i128) -> Self {
            let mut big = Self::from_u128(value.unsigned_abs());
            big.negative = value < 0;
            big
        }

        #[must_use]
        pub fn from_u128(mut value: u128) -> Self {
            let mut magnitude = Vec::new();
            while value > 0 {
                magnitude.push(value as u32);
                value >>= 32;
            }
            Self::from_parts(false, magnitude)
        }

        /// A decimal literal with an optional leading minus sign.
        #[must_use]
        pub fn parse(text: &str) -> Self {
            let (negative, digits) = text.strip_prefix('-').map_or((false, text), |rest| (true, rest));
            let ten = Self::from_i128(10);
            let mut value = Self::zero();
            for digit in digits.chars() {
                let digit = digit.to_digit(10).expect("decimal literal");
                value = value.mul(&ten).add(&Self::from_i128(i128::from(digit)));
            }
            if negative { value.neg() } else { value }
        }

        #[must_use]
        pub const fn is_zero(&self) -> bool {
            self.magnitude.is_empty()
        }

        #[must_use]
        pub const fn is_negative(&self) -> bool {
            self.negative
        }

        /// The array index this names, if it is one.
        #[must_use]
        pub fn to_index(&self) -> Option<usize> {
            if self.negative || self.magnitude.len() > 2 {
                return None;
            }
            let value = self.magnitude.iter().rev().fold(0u64, |acc, &digit| (acc << 32) | u64::from(digit));
            usize::try_from(value).ok()
        }

        #[must_use]
        pub fn neg(&self) -> Self {
            Self::from_parts(!self.negative, self.magnitude.clone())
        }

        #[must_use]
        pub fn add(&self, other: &Self) -> Self {
            if self.negative == other.negative {
                return Self::from_parts(self.negative, add_magnitude(&self.magnitude, &other.magnitude));
            }
            match compare_magnitude(&self.magnitude, &other.magnitude) {
                Ordering::Less => Self::from_parts(other.negative, sub_magnitude(&other.magnitude, &self.magnitude)),
                _ => Self::from_parts(self.negative, sub_magnitude(&self.magnitude, &other.magnitude)),
            }
        }

        #[must_use]
        pub fn sub(&self, other: &Self) -> Self {
            self.add(&other.neg())
        }

        #[must_use]
        pub fn mul(&self, other: &Self) -> Self {
            Self::from_parts(self.negative != other.negative, mul_magnitude(&self.magnitude, &other.magnitude))
        }

        /// Division rounding toward zero; the divisor is not zero.
        #[must_use]
        pub fn divmod_trunc(&self, other: &Self) -> (Self, Self) {
            let (quotient, remainder) = divmod_magnitude(&self.magnitude, &other.magnitude);
            (
                Self::from_parts(self.negative != other.negative, quotient),
                Self::from_parts(self.negative, remainder),
            )
        }
    }

    impl Ord for Big {
        fn cmp(&self, other: &Self) -> Ordering {
            match (self.negative, other.negative) {
                (false, true) => Ordering::Greater,
                (true, false) => Ordering::Less,
                (false, false) => compare_magnitude(&self.magnitude, &other.magnitude),
                (true, true) => compare_magnitude(&other.magnitude, &self.magnitude),
            }
        }
    }

    impl PartialOrd for Big {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl fmt::Display for Big {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_zero() {
                return formatter.write_str("0");
            }
            let billion = [1_000_000_000u32];
            let mut chunks = Vec::new();
            let mut rest = self.magnitude.clone();
            while !rest.is_empty() {
                let (quotient, remainder) = divmod_magnitude(&rest, &billion);
                chunks.push(remainder.first().copied().unwrap_or(0));
                rest = quotient;
            }
            let mut text = String::new();
            if self.negative {
                text.push('-');
            }
            text.push_str(&chunks.pop().unwrap_or(0).to_string());
            for chunk in chunks.iter().rev() {
                std::fmt::Write::write_fmt(&mut text, format_args!("{chunk:09}")).expect("writing to a String cannot fail");
            }
            formatter.write_str(&text)
        }
    }

    /// Natural subtraction: truncated at zero.
    #[must_use]
    pub fn nat_sub(a: &Big, b: &Big) -> Big {
        if a > b { a.sub(b) } else { Big::zero() }
    }

    #[must_use]
    pub fn pred(a: &Big) -> Big {
        a.sub(&Big::from_i128(1))
    }

    /// Integer division with the source's rounding ("trunc", "floor" or
    /// "euclid"); by zero it aborts with the source's message or is total
    /// (x / 0 = 0, x % 0 = x).
    #[must_use]
    pub fn divide(a: &Big, b: &Big, rounding: &str, by_zero: Option<&str>, remainder: bool) -> Big {
        if b.is_zero() {
            if let Some(message) = by_zero {
                panic!("{message}");
            }
            return if remainder { a.clone() } else { Big::zero() };
        }
        let one = Big::from_i128(1);
        let (mut quotient, rest) = a.divmod_trunc(b);
        if !rest.is_zero() {
            if rounding == "floor" && rest.is_negative() != b.is_negative() {
                quotient = quotient.sub(&one);
            }
            if rounding == "euclid" && rest.is_negative() {
                quotient = if b.is_negative() { quotient.add(&one) } else { quotient.sub(&one) };
            }
        }
        if remainder { a.sub(&quotient.mul(b)) } else { quotient }
    }

    #[must_use]
    pub fn to_nat_checked(value: Big, message: &str) -> Big {
        assert!(!value.is_negative(), "{message}");
        value
    }

    #[must_use]
    pub fn clamp_nat(value: Big) -> Big {
        if value.is_negative() { Big::zero() } else { value }
    }

    /// Bounded domains for executable theorem checks.
    #[must_use]
    pub fn range(low: i128, high: i128) -> Vec<Big> {
        (low..=high).map(Big::from_i128).collect()
    }
}

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
            let other = if digits == down { increment(&down) } else { Some(down) };
            if let Some(other) = other {
                let back = format!("{}.{}e{}", &other[..1], &other[1..], n - 1);
                if last % 2 == 1 && back.parse::<f64>() == Ok(x.abs()) {
                    digits = other.trim_end_matches('0').to_string();
                }
            }
        }
        let k = i32::try_from(digits.len()).expect("binary64 significand length fits i32");
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
        let digits = if digits.is_empty() { String::from("0") } else { digits };
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
        index.and_then(|index| values.get(index)).map_or_else(|| panic!("array index out of range"), Clone::clone)
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
    const fn integer(value: f64) -> f64 { if value.is_nan() { 0.0 } else { value.trunc() } }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 { (length as f64 + value).max(0.0) as usize } else { value.min(length as f64) as usize }
    }
    #[must_use]
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len()); let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    #[must_use]
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
    #[must_use]
    pub fn includes<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    #[must_use]
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values { if !out.iter().any(|previous| equal(previous, &value)) { out.push(value); } }
        out
    }
    #[must_use]
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) { out[index].1 = value; }
            else { out.push((key, value)); }
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
                            let ml_s1 = self::ml_read_primitive_bitwise_domain_loop2(number, integer, &(ml_values1), ml_index1);
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
pub fn apply_big_integer_bitwise_operation(operation: f64, left: self::ml::Big, right: self::ml::Big) -> self::ml::Big {
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
assert!(!(((operation != 0f64) && (operation != 1f64)) && (operation != 2f64)), "{}", "an unknown BigInt bitwise operation");

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
                                            let ml_s3 = self::ml_apply_big_integer_bitwise_operation_loop6(operation, negative_left, negative_right, first, second, weight, result);
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
pub fn render_big_integer_bitwise_expression(target: &str, operation: &str, arguments_: &[String]) -> String {
    {
        let code = self::read_big_integer_bitwise_operation_code(operation);
        if (code < 0f64) || ((arguments_.len() as f64) != (self::read_big_integer_bitwise_arity(operation))) {
            String::new()
        } else {
            {
                let left = (self::ml_array::optional_at(arguments_, self::ml_array::number_index(0f64))).unwrap_or_else(|| String::new());
                {
                    let right = (self::ml_array::optional_at(arguments_, self::ml_array::number_index(1f64))).unwrap_or_else(|| String::new());
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
                            format!("({operation_name} {left}{})", if code == 6f64 {
                                String::new()
                            } else {
                                format!("{}{}", (String::from(" ")), right)
                            })
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
                                            let suffix_2 = self::ml_render_big_integer_bitwise_expression_loop7(&(text), suffix);
                                            {
                                                let a = format!("ml_integer_left{}", self::ml_number::js_number(suffix_2));
                                                {
                                                    let b = format!("ml_integer_right{}", self::ml_number::js_number(suffix_2));
                                                    {
                                                        let am = format!("(Int.toNat (-1 - {a}))");
                                                        {
                                                            let bm = format!("(Int.toNat (-1 - {b}))");
                                                            {
                                                                let ap = format!("(Int.toNat {a})");
                                                                {
                                                                    let bp = format!("(Int.toNat {b})");
                                                                    {
                                                                        let prefix = format!("(let {a} : Int := {left}; let {b} : Int := {right}; ");
                                                                        if (code == 3f64) || (code == 4f64) {
                                                                            {
                                                                                let count = format!("(Int.toNat (if {b} < 0 then -{b} else {b}))");
                                                                                {
                                                                                    let shift_left = format!("({a} * Int.ofNat (Nat.shiftLeft 1 {count}))");
                                                                                    {
                                                                                        let shift_right = format!("(if {a} < 0 then -1 - Int.ofNat (Nat.shiftRight {am} {count}) else Int.ofNat (Nat.shiftRight {ap} {count}))");
                                                                                        format!("{prefix}if {a} = 0 then 0 else if {b} < 0 then {} else {})", if code == 3f64 {
                                                                                            shift_right.clone()
                                                                                        } else {
                                                                                            shift_left.clone()
                                                                                        }, if code == 3f64 {
                                                                                            shift_left.clone()
                                                                                        } else {
                                                                                            shift_right.clone()
                                                                                        })
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            {
                                                                                let difference = String::from("(Nat.bitwise (fun ml_bit_left ml_bit_right => ml_bit_left && !ml_bit_right) ");
                                                                                {
                                                                                    let both = if code == 0f64 {
                                                                                        format!("(-1 - Int.ofNat (Nat.lor {am} {bm}))")
                                                                                    } else if code == 1f64 {
                                                                                            format!("(-1 - Int.ofNat (Nat.land {am} {bm}))")
                                                                                        } else {
                                                                                            format!("(Int.ofNat (Nat.xor {am} {bm}))")
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
                                                                                                format!("{prefix}if {a} < 0 then (if {b} < 0 then {both} else {left_negative}) else (if {b} < 0 then {right_negative} else {positive}))")
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
                                format!("crate::ml_big_integer_bitwise::apply_big_integer_bitwise_operation({}f64, {left}, {})", self::ml_number::js_number(code), if code == 6f64 {
                                    String::from("ml::Big::zero()")
                                } else {
                                    right.clone()
                                })
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
    String::from("BigInt bitwise operations use exact unbounded integers with infinite sign extension, negative shift reversal and arithmetic right shifts; operands are evaluated once in source order without Number coercion or modulo-32 shift counts. Native source and the emitted integer runtime are generated from the same checked definition.")
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadPrimitiveBitwiseDomainLoop2Result {
    MlReadPrimitiveBitwiseDomainLoop2Done(bool, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_primitive_bitwise_domain_loop2(mut number: bool, mut integer: bool, ml_values1: &[String], mut ml_index1: self::ml::Big) -> self::MlReadPrimitiveBitwiseDomainLoop2Result {
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
            self::MlReadPrimitiveBitwiseDomainLoop2Result::MlReadPrimitiveBitwiseDomainLoop2Done(number, integer)
        };
    }
}
