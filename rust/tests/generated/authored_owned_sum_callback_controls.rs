// Translated from JavaScript by meta-language: portable core, Rust target.
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]

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
            for limb in remainder.iter_mut() {
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

        pub fn zero() -> Self {
            Self::from_parts(false, Vec::new())
        }

        pub fn from_i128(value: i128) -> Self {
            let mut big = Self::from_u128(value.unsigned_abs());
            big.negative = value < 0;
            big
        }

        pub fn from_u128(mut value: u128) -> Self {
            let mut magnitude = Vec::new();
            while value > 0 {
                magnitude.push(value as u32);
                value >>= 32;
            }
            Self::from_parts(false, magnitude)
        }

        /// A decimal literal with an optional leading minus sign.
        pub fn parse(text: &str) -> Self {
            let (negative, digits) = match text.strip_prefix('-') {
                Some(rest) => (true, rest),
                None => (false, text),
            };
            let ten = Self::from_i128(10);
            let mut value = Self::zero();
            for digit in digits.chars() {
                let digit = digit.to_digit(10).expect("decimal literal");
                value = value.mul(&ten).add(&Self::from_i128(i128::from(digit)));
            }
            if negative { value.neg() } else { value }
        }

        pub fn is_zero(&self) -> bool {
            self.magnitude.is_empty()
        }

        pub fn is_negative(&self) -> bool {
            self.negative
        }

        /// The array index this names, if it is one.
        pub fn to_index(&self) -> Option<usize> {
            if self.negative || self.magnitude.len() > 2 {
                return None;
            }
            let value = self.magnitude.iter().rev().fold(0u64, |acc, &digit| (acc << 32) | u64::from(digit));
            usize::try_from(value).ok()
        }

        pub fn neg(&self) -> Self {
            Self::from_parts(!self.negative, self.magnitude.clone())
        }

        pub fn add(&self, other: &Self) -> Self {
            if self.negative == other.negative {
                return Self::from_parts(self.negative, add_magnitude(&self.magnitude, &other.magnitude));
            }
            match compare_magnitude(&self.magnitude, &other.magnitude) {
                Ordering::Less => Self::from_parts(other.negative, sub_magnitude(&other.magnitude, &self.magnitude)),
                _ => Self::from_parts(self.negative, sub_magnitude(&self.magnitude, &other.magnitude)),
            }
        }

        pub fn sub(&self, other: &Self) -> Self {
            self.add(&other.neg())
        }

        pub fn mul(&self, other: &Self) -> Self {
            Self::from_parts(self.negative != other.negative, mul_magnitude(&self.magnitude, &other.magnitude))
        }

        /// Division rounding toward zero; the divisor is not zero.
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
                text.push_str(&format!("{chunk:09}"));
            }
            formatter.write_str(&text)
        }
    }

    /// Natural subtraction: truncated at zero.
    pub fn nat_sub(a: &Big, b: &Big) -> Big {
        if a > b { a.sub(b) } else { Big::zero() }
    }

    pub fn pred(a: &Big) -> Big {
        a.sub(&Big::from_i128(1))
    }

    /// Integer division with the source's rounding ("trunc", "floor" or
    /// "euclid"); by zero it aborts with the source's message or is total
    /// (x / 0 = 0, x % 0 = x).
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

    pub fn to_nat_checked(value: Big, message: &str) -> Big {
        if value.is_negative() {
            panic!("{message}");
        }
        value
    }

    pub fn clamp_nat(value: Big) -> Big {
        if value.is_negative() { Big::zero() } else { value }
    }

    /// Bounded domains for executable theorem checks.
    pub fn range(low: i128, high: i128) -> Vec<Big> {
        (low..=high).map(Big::from_i128).collect()
    }
}

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

/// The Math functions that Rust's f64 methods do not share with JavaScript.
pub mod ml_math {
    /// Math.round: the nearest integer, the one towards +Infinity on a tie,
    /// with the sign of a zero result from the argument.
    pub fn round(x: f64) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        if x > 0.0 && x < 0.5 {
            return 0.0;
        }
        if x < 0.0 && x >= -0.5 {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 {
            floor + 1.0
        } else {
            floor
        }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where f64::signum does not.
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
    pub fn max(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else if b > a {
            b
        } else if a == b {
            if a.is_sign_negative() {
                b
            } else {
                a
            }
        } else {
            f64::NAN
        }
    }

    /// Math.min of two Numbers: NaN when either is, and -0 below 0.
    pub fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else if b < a {
            b
        } else if a == b {
            if a.is_sign_negative() {
                a
            } else {
                b
            }
        } else {
            f64::NAN
        }
    }

    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    pub fn min_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| min(a, b))
    }

    pub fn is_integer(x: f64) -> bool {
        x.is_finite() && x.trunc() == x
    }

    pub fn is_safe_integer(x: f64) -> bool {
        is_integer(x) && x.abs() <= 9007199254740991.0
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

pub fn read_rust_owned_standard_sum_callback_arity(owner: String, method: String) -> f64 {
    if (owner == "std::option::Option") {
        if self::ml_array::includes(&vec![String::from("unwrap_or_else"), String::from("ok_or_else"), String::from("map"), String::from("and_then"), String::from("or_else")], &method.clone(), 0f64, false, |a, b| a == b) {
            1f64
        } else {
            (-1f64)
        }
    } else {
        if ((owner == "std::result::Result") && self::ml_array::includes(&vec![String::from("unwrap_or_else"), String::from("map"), String::from("map_err"), String::from("and_then"), String::from("or_else")], &method.clone(), 0f64, false, |a, b| a == b)) {
            1f64
        } else {
            (-1f64)
        }
    }
}

pub fn read_rust_owned_standard_sum_callback_version_error(owner: String, method: String, minor: f64) -> String {
    if ((self::read_rust_owned_standard_sum_callback_arity(owner.clone(), method.clone())) < (0f64)) {
        String::from("the owned standard sum callback method is outside this profile")
    } else {
        if ((self::ml_math::is_integer(minor) && (minor >= (90f64))) && (minor < (100f64))) {
            String::from("")
        } else {
            String::from("the owned standard sum callback source version is uncovered")
        }
    }
}

pub fn read_rust_owned_standard_sum_callback_parameter(owner: String, method: String) -> f64 {
    if ((self::read_rust_owned_standard_sum_callback_arity(owner.clone(), method.clone())) < (0f64)) {
        (-2f64)
    } else {
        if (owner == "std::option::Option") {
            if ((method == "map") || (method == "and_then")) {
                0f64
            } else {
                (-1f64)
            }
        } else {
            if ((method == "map") || (method == "and_then")) {
                0f64
            } else {
                1f64
            }
        }
    }
}

pub fn read_rust_owned_standard_sum_callback_result(owner: String, method: String) -> String {
    if ((self::read_rust_owned_standard_sum_callback_arity(owner.clone(), method.clone())) < (0f64)) {
        String::from("")
    } else {
        if ((method == "and_then") || (method == "or_else")) {
            String::from("bound sum")
        } else {
            if (method == "unwrap_or_else") {
                String::from("payload")
            } else {
                if (method == "ok_or_else") {
                    String::from("std::result::Result")
                } else {
                    owner.clone()
                }
            }
        }
    }
}

pub fn read_rust_owned_standard_sum_callback_retained_argument(owner: String, method: String) -> f64 {
    if ((self::read_rust_owned_standard_sum_callback_arity(owner.clone(), method.clone())) < (0f64)) {
        (-2f64)
    } else {
        if (method == "or_else") {
            0f64
        } else {
            if ((owner == "std::result::Result") && (method == "and_then")) {
                1f64
            } else {
                (-1f64)
            }
        }
    }
}

pub fn read_rust_owned_standard_sum_callback_bound_result_error(source: String, result: String, layers: f64, identity: bool) -> String {
    if ((((source == result) && ((source == "std::option::Option") || (source == "std::result::Result"))) && (layers == (0f64))) && identity) {
        String::from("")
    } else {
        String::from("the callback must return its actual direct same-family sum and preserve every retained argument identity")
    }
}

pub fn plan_rust_owned_standard_sum_callback_arguments(owner: String, method: String) -> Vec<f64> {
    if (((self::read_rust_owned_standard_sum_callback_arity(owner.clone(), method.clone())) < (0f64)) || self::ml_array::includes(&vec![String::from("unwrap_or_else"), String::from("and_then"), String::from("or_else")], &method.clone(), 0f64, false, |a, b| a == b)) {
        Vec::<f64>::new()
    } else {
        if (method == "ok_or_else") {
            vec![0f64, (-1f64)]
        } else {
            if (owner == "std::option::Option") {
                vec![(-1f64)]
            } else {
                if (method == "map") {
                    vec![(-1f64), 1f64]
                } else {
                    vec![0f64, (-1f64)]
                }
            }
        }
    }
}

pub fn plan_rust_owned_standard_sum_callback_branch(owner: String, variant: String, method: String) -> Vec<String> {
    {
        let source_ml_source1_s1 = if (owner == "std::option::Option") {
            if (method == "and_then") {
                self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1Return(if (variant == "Some") {
                    vec![String::from(""), String::from("invoke")]
                } else {
                    if (variant == "None") {
                        vec![String::from("None"), String::from("empty")]
                    } else {
                        Vec::<String>::new()
                    }
                })
            } else {
                if (method == "or_else") {
                    self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1Return(if (variant == "Some") {
                        vec![String::from("Some"), String::from("payload")]
                    } else {
                        if (variant == "None") {
                            vec![String::from(""), String::from("invoke")]
                        } else {
                            Vec::<String>::new()
                        }
                    })
                } else {
                    if (method == "unwrap_or_else") {
                        self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1Return(if (variant == "Some") {
                            vec![String::from(""), String::from("payload")]
                        } else {
                            if (variant == "None") {
                                vec![String::from(""), String::from("invoke")]
                            } else {
                                Vec::<String>::new()
                            }
                        })
                    } else {
                        if (method == "ok_or_else") {
                            self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1Return(if (variant == "Some") {
                                vec![String::from("Ok"), String::from("payload")]
                            } else {
                                if (variant == "None") {
                                    vec![String::from("Err"), String::from("invoke")]
                                } else {
                                    Vec::<String>::new()
                                }
                            })
                        } else {
                            if (method == "map") {
                                self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1Return(if (variant == "Some") {
                                    vec![String::from("Some"), String::from("invoke")]
                                } else {
                                    if (variant == "None") {
                                        vec![String::from("None"), String::from("empty")]
                                    } else {
                                        Vec::<String>::new()
                                    }
                                })
                            } else {
                                self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1Next
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1Next
        };
        match source_ml_source1_s1.clone() {
            self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1Next => {
                {
                    let source_ml_source1_s2 = if (owner == "std::result::Result") {
                        if (method == "and_then") {
                            self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2Return(if (variant == "Ok") {
                                vec![String::from(""), String::from("invoke")]
                            } else {
                                if (variant == "Err") {
                                    vec![String::from("Err"), String::from("payload")]
                                } else {
                                    Vec::<String>::new()
                                }
                            })
                        } else {
                            if (method == "or_else") {
                                self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2Return(if (variant == "Ok") {
                                    vec![String::from("Ok"), String::from("payload")]
                                } else {
                                    if (variant == "Err") {
                                        vec![String::from(""), String::from("invoke")]
                                    } else {
                                        Vec::<String>::new()
                                    }
                                })
                            } else {
                                if (method == "unwrap_or_else") {
                                    self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2Return(if (variant == "Ok") {
                                        vec![String::from(""), String::from("payload")]
                                    } else {
                                        if (variant == "Err") {
                                            vec![String::from(""), String::from("invoke")]
                                        } else {
                                            Vec::<String>::new()
                                        }
                                    })
                                } else {
                                    if (method == "map") {
                                        self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2Return(if (variant == "Ok") {
                                            vec![String::from("Ok"), String::from("invoke")]
                                        } else {
                                            if (variant == "Err") {
                                                vec![String::from("Err"), String::from("payload")]
                                            } else {
                                                Vec::<String>::new()
                                            }
                                        })
                                    } else {
                                        if (method == "map_err") {
                                            self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2Return(if (variant == "Ok") {
                                                vec![String::from("Ok"), String::from("payload")]
                                            } else {
                                                if (variant == "Err") {
                                                    vec![String::from("Err"), String::from("invoke")]
                                                } else {
                                                    Vec::<String>::new()
                                                }
                                            })
                                        } else {
                                            self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2Next
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2Next
                    };
                    match source_ml_source1_s2.clone() {
                        self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2Next => {
                            Vec::<String>::new()
                        }
                        self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2Return(source_ml_temporary_result2) => {
                            source_ml_temporary_result2
                        }
                    }
                }
            }
            self::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1::MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1Return(source_ml_temporary_result2_2) => {
                source_ml_temporary_result2_2
            }
        }
    }
}

pub fn read_rust_owned_standard_sum_callback_closure_header_error(ordinary: bool, names: Vec<String>, roles: Vec<String>, result: String) -> String {
    if !ordinary {
        String::from("an owned-domain callback needs the active ordinary source profile")
    } else {
        if ((((names.len() as f64)) != ((roles.len() as f64))) || (((names.len() as f64)) > (1024f64))) {
            String::from("the complete owned-domain callback header exceeds its source budget")
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source1_s3 = self::source_ml_temporary_read_rust_owned_standard_sum_callback_closure_header_error_loop3(names, roles, at);
                    match source_ml_source1_s3.clone() {
                        self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Result::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Done => {
                            if ((result == "copy") || (result == "owned")) {
                                String::from("")
                            } else {
                                String::from("the callback result needs its actual direct Copy or supported owned source domain")
                            }
                        }
                        self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Result::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Return(source_ml_temporary_result3) => {
                            source_ml_temporary_result3.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_owned_standard_sum_callback_capture_error(captures: f64, assigned: f64) -> String {
    if ((captures == (0f64)) && (assigned == (0f64))) {
        String::from("")
    } else {
        String::from("owned-domain callbacks currently need actual capture-free source closures")
    }
}

pub fn read_rust_owned_standard_sum_callback_move_capture_error(moved: bool, names: Vec<String>, roles: Vec<String>, identities: Vec<String>, layers: Vec<f64>, borrowed: Vec<bool>, assigned: f64) -> String {
    if (assigned != (0f64)) {
        String::from("mutable callback captures need their actual environment state interpretation")
    } else {
        if (((((((names.len() as f64)) > (1024f64)) || (((roles.len() as f64)) != ((names.len() as f64)))) || (((identities.len() as f64)) != ((names.len() as f64)))) || (((layers.len() as f64)) != ((names.len() as f64)))) || (((borrowed.len() as f64)) != ((names.len() as f64)))) {
            String::from("the complete copied capture environment needs every actual source binding")
        } else {
            if ((((names.len() as f64)) > (0f64)) && !moved) {
                String::from("captured owned-domain callbacks need an explicit move source closure")
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s4 = self::source_ml_temporary_read_rust_owned_standard_sum_callback_move_capture_error_loop5(names, roles, identities, layers, borrowed, at);
                        match source_ml_source1_s4.clone() {
                            self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Done => {
                                String::from("")
                            }
                            self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Return(source_ml_temporary_result5) => {
                                source_ml_temporary_result5.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_rust_owned_standard_sum_callback_move_closure_copy(parameters: Vec<String>, result: String, moved: bool, names: Vec<String>, roles: Vec<String>, identities: Vec<String>, layers: Vec<f64>, borrowed: Vec<bool>, assigned: f64) -> bool {
    if ((((parameters.len() as f64)) > (1024f64)) || ((result != "copy") && (result != "owned"))) {
        false
    } else {
        {
            let source_ml_temporary_values7 = parameters;
            {
                let source_ml_temporary_index7 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s5 = self::source_ml_temporary_accept_rust_owned_standard_sum_callback_move_closure_copy_loop8(source_ml_temporary_values7, source_ml_temporary_index7.clone());
                    match source_ml_source1_s5.clone() {
                        self::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Result::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Done => {
                            ((self::read_rust_owned_standard_sum_callback_move_capture_error(moved, names, roles, identities, layers, borrowed, assigned)) == "")
                        }
                        self::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Result::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Return(source_ml_temporary_result8) => {
                            source_ml_temporary_result8
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_owned_standard_sum_callback_immediate_capture_error(captures: f64, kind: String, identity: bool, moved: bool, names: f64) -> String {
    if ((captures == (0f64)) || (((((captures > (0f64)) && (kind == "lambda")) && identity) && moved) && (names == captures))) {
        String::from("")
    } else {
        String::from("a copied callback environment needs its actual explicit move Lambda source and span identity")
    }
}

pub fn accept_rust_owned_standard_sum_callback_closure_copy(parameters: Vec<String>, result: String, captures: f64, assigned: f64) -> bool {
    if ((((captures != (0f64)) || (assigned != (0f64))) || (((parameters.len() as f64)) > (1024f64))) || ((result != "copy") && (result != "owned"))) {
        false
    } else {
        {
            let source_ml_temporary_values9 = parameters;
            {
                let source_ml_temporary_index9 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s6 = self::source_ml_temporary_accept_rust_owned_standard_sum_callback_closure_copy_loop10(source_ml_temporary_values9, source_ml_temporary_index9.clone());
                    match source_ml_source1_s6.clone() {
                        self::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Result::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Done => {
                            true
                        }
                        self::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Result::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Return(source_ml_temporary_result10) => {
                            source_ml_temporary_result10
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_owned_standard_sum_callback_source_error(kind: String, parameters: f64, expected: f64, wrappers: f64, callable_copy: bool, result: String) -> String {
    if (((((kind == "function") && (parameters == expected)) && (wrappers == (0f64))) && callable_copy) && ((result == "copy") || (result == "owned"))) {
        String::from("")
    } else {
        String::from("the callback needs its actual Copy callable and complete movable parameter/result contracts")
    }
}

pub fn read_rust_owned_standard_sum_callback_selection_token_limit() -> f64 {
    4096f64
}

pub fn plan_rust_owned_standard_sum_callback_selection(values: Vec<String>, kinds: Vec<String>) -> Vec<f64> {
    if ((((((values.len() as f64)) != ((kinds.len() as f64))) || (((values.len() as f64)) == (0f64))) || (((values.len() as f64)) > (4096f64))) || {
        let source_ml_source1_comparison_operand7 = self::ml_array::optional_at(&values, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand8 = String::from("(");
            !(match (source_ml_source1_comparison_operand7.clone(), source_ml_source1_comparison_operand8.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    }) {
        Vec::<f64>::new()
    } else {
        {
            let at = 1f64;
            {
                let layers = 0f64;
                {
                    let at_2 = self::source_ml_temporary_plan_rust_owned_standard_sum_callback_selection_loop11(values.clone(), at, layers);
                    {
                        let moved = {
                            let source_ml_source1_comparison_operand9 = self::ml_array::optional_at(&values, self::ml_array::number_index(at_2));
                            {
                                let source_ml_source1_comparison_operand10 = String::from("move");
                                match (source_ml_source1_comparison_operand9.clone(), source_ml_source1_comparison_operand10.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                            }
                        };
                        {
                            let at_4 = if moved {
                                {
                                    let at_3 = (at_2 + 1f64);
                                    at_3
                                }
                            } else {
                                at_2
                            };
                            {
                                let code = 1f64;
                                {
                                    let source_ml_source1_s26 = if ({
                                        let source_ml_source1_comparison_operand27 = self::ml_array::optional_at(&values, self::ml_array::number_index(at_4));
                                        {
                                            let source_ml_source1_comparison_operand28 = String::from("||");
                                            match (source_ml_source1_comparison_operand27.clone(), source_ml_source1_comparison_operand28.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                        }
                                    } || {
                                        let source_ml_source1_comparison_operand29 = self::ml_array::optional_at(&values, self::ml_array::number_index(at_4));
                                        {
                                            let source_ml_source1_comparison_operand30 = String::from("|");
                                            match (source_ml_source1_comparison_operand29.clone(), source_ml_source1_comparison_operand30.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                        }
                                    }) {
                                        {
                                            let source_ml_source1_s35 = if {
                                                let source_ml_source1_comparison_operand36 = self::ml_array::optional_at(&values, self::ml_array::number_index(at_4));
                                                {
                                                    let source_ml_source1_comparison_operand37 = String::from("||");
                                                    match (source_ml_source1_comparison_operand36.clone(), source_ml_source1_comparison_operand37.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                }
                                            } {
                                                {
                                                    let at_5 = (at_4 + 1f64);
                                                    self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15Next(at_5)
                                                }
                                            } else {
                                                {
                                                    let at_6 = (at_4 + 1f64);
                                                    {
                                                        let budget = 0f64;
                                                        {
                                                            let at_7 = self::source_ml_temporary_plan_rust_owned_standard_sum_callback_selection_loop16(values.clone(), at_6, budget);
                                                            if {
                                                                let source_ml_source1_comparison_operand38 = self::ml_array::optional_at(&values, self::ml_array::number_index(at_7));
                                                                {
                                                                    let source_ml_source1_comparison_operand39 = String::from("|");
                                                                    !(match (source_ml_source1_comparison_operand38.clone(), source_ml_source1_comparison_operand39.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                }
                                                            } {
                                                                self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15Return(Vec::<f64>::new())
                                                            } else {
                                                                {
                                                                    let at_8 = (at_7 + 1f64);
                                                                    self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15Next(at_8)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            };
                                            match source_ml_source1_s35.clone() {
                                                self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15Next(at_9) => {
                                                    if {
                                                        let source_ml_source1_comparison_operand40 = self::ml_array::optional_at(&values, self::ml_array::number_index(at_9));
                                                        {
                                                            let source_ml_source1_comparison_operand41 = String::from("->");
                                                            !(match (source_ml_source1_comparison_operand40.clone(), source_ml_source1_comparison_operand41.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                        }
                                                    } {
                                                        self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Return(Vec::<f64>::new())
                                                    } else {
                                                        {
                                                            let at_10 = (at_9 + 1f64);
                                                            {
                                                                let code_2 = 3f64;
                                                                self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Next(at_10, code_2)
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15Return(source_ml_temporary_result15) => {
                                                    self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Return(source_ml_temporary_result15)
                                                }
                                            }
                                        }
                                    } else {
                                        if moved {
                                            self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Return(Vec::<f64>::new())
                                        } else {
                                            self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Next(at_4, code)
                                        }
                                    };
                                    match source_ml_source1_s26.clone() {
                                        self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Next(at_11, code_3) => {
                                            {
                                                let start = at_11;
                                                if ({
                                                    let source_ml_source1_comparison_operand42 = self::ml_array::optional_at(&kinds, self::ml_array::number_index(at_11));
                                                    {
                                                        let source_ml_source1_comparison_operand43 = String::from("identifier");
                                                        !(match (source_ml_source1_comparison_operand42.clone(), source_ml_source1_comparison_operand43.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                    }
                                                } && !(self::ml_array::includes(&vec![String::from("crate"), String::from("self"), String::from("super")], &(self::ml_array::optional_at(&values, self::ml_array::number_index(at_11))).clone().unwrap_or_else(|| String::from("")), 0f64, false, |a, b| a == b))) {
                                                    Vec::<f64>::new()
                                                } else {
                                                    {
                                                        let at_12 = (at_11 + 1f64);
                                                        {
                                                            let parts = 1f64;
                                                            {
                                                                let at_13 = self::source_ml_temporary_plan_rust_owned_standard_sum_callback_selection_loop13(values.clone(), kinds, at_12, parts);
                                                                if (code_3 == (3f64)) {
                                                                    vec![3f64, start, at_13]
                                                                } else {
                                                                    {
                                                                        let source_ml_source1_s50 = if {
                                                                            let source_ml_source1_comparison_operand51 = self::ml_array::optional_at(&values, self::ml_array::number_index(at_13));
                                                                            {
                                                                                let source_ml_source1_comparison_operand52 = String::from("(");
                                                                                match (source_ml_source1_comparison_operand51.clone(), source_ml_source1_comparison_operand52.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                            }
                                                                        } {
                                                                            {
                                                                                let code_4 = 2f64;
                                                                                self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14Next(code_4)
                                                                            }
                                                                        } else {
                                                                            if ({
                                                                                let source_ml_source1_comparison_operand53 = self::ml_array::optional_at(&values, self::ml_array::number_index(at_13));
                                                                                {
                                                                                    let source_ml_source1_comparison_operand54 = String::from(")");
                                                                                    !(match (source_ml_source1_comparison_operand53.clone(), source_ml_source1_comparison_operand54.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                                }
                                                                            } && {
                                                                                let source_ml_source1_comparison_operand55 = self::ml_array::optional_at(&values, self::ml_array::number_index(at_13));
                                                                                {
                                                                                    let source_ml_source1_comparison_operand56 = String::from(",");
                                                                                    !(match (source_ml_source1_comparison_operand55.clone(), source_ml_source1_comparison_operand56.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                                }
                                                                            }) {
                                                                                self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14Return(Vec::<f64>::new())
                                                                            } else {
                                                                                self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14Next(code_3)
                                                                            }
                                                                        };
                                                                        match source_ml_source1_s50.clone() {
                                                                            self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14Next(code_5) => {
                                                                                vec![code_5, start, at_13]
                                                                            }
                                                                            self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14Return(source_ml_temporary_result14) => {
                                                                                source_ml_temporary_result14
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
                                        self::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12::MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Return(source_ml_temporary_result14_2) => {
                                            source_ml_temporary_result14_2
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

pub fn accept_rust_owned_standard_sum_callback_written_selection(owner: String, path: String, alias: bool, shadow: bool) -> bool {
    (((!alias && !shadow) && (owner == "library")) && ((path == "std::string::String") || (path == "std::vec::Vec")))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2 {
    MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2Next,
    MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin2Return(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1 {
    MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1Next,
    MlTemporaryPlanRustOwnedStandardSumCallbackBranchJoin1Return(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Result {
    MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Done,
    MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Result {
    MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Done,
    MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Return(String),
}

pub fn source_ml_temporary_read_rust_owned_standard_sum_callback_closure_header_error_loop4(mut names: Vec<String>, mut at: f64, mut before: f64) -> self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Result {
    loop {
        return if (before < at) {
            if {
                let source_ml_source1_comparison_operand57 = self::ml_array::optional_at(&names, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand58 = self::ml_array::optional_at(&names, self::ml_array::number_index(before));
                    match (source_ml_source1_comparison_operand57.clone(), source_ml_source1_comparison_operand58.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false }
                }
            } {
                self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Result::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Return(String::from("callback parameters need distinct source bindings"))
            } else {
                {
                    let before_2 = (before + 1f64);
                    {
                        before = before_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Result::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Done
        };
    }
}

pub fn source_ml_temporary_read_rust_owned_standard_sum_callback_closure_header_error_loop3(mut names: Vec<String>, mut roles: Vec<String>, mut at: f64) -> self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Result {
    loop {
        return if (at < ((roles.len() as f64))) {
            if ({
                let source_ml_source1_comparison_operand59 = self::ml_array::optional_at(&roles, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand60 = String::from("copy");
                    !(match (source_ml_source1_comparison_operand59.clone(), source_ml_source1_comparison_operand60.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            } && {
                let source_ml_source1_comparison_operand61 = self::ml_array::optional_at(&roles, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand62 = String::from("owned");
                    !(match (source_ml_source1_comparison_operand61.clone(), source_ml_source1_comparison_operand62.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            }) {
                self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Result::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Return(String::from("the callback parameter needs its actual direct Copy or supported owned source domain"))
            } else {
                {
                    let before = 0f64;
                    {
                        let source_ml_source1_s63 = self::source_ml_temporary_read_rust_owned_standard_sum_callback_closure_header_error_loop4(names.clone(), at, before);
                        match source_ml_source1_s63.clone() {
                            self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Result::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Done => {
                                {
                                    let at_2 = (at + 1f64);
                                    {
                                        at = at_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Result::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop4Return(source_ml_temporary_result4) => {
                                self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Result::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Return(source_ml_temporary_result4.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Result::MlTemporaryReadRustOwnedStandardSumCallbackClosureHeaderErrorLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Result {
    MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Done,
    MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Result {
    MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Done,
    MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Return(String),
}

pub fn source_ml_temporary_read_rust_owned_standard_sum_callback_move_capture_error_loop6(mut names: Vec<String>, mut at: f64, mut before: f64) -> self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Result {
    loop {
        return if (before < at) {
            if {
                let source_ml_source1_comparison_operand64 = self::ml_array::optional_at(&names, self::ml_array::number_index(before));
                {
                    let source_ml_source1_comparison_operand65 = self::ml_array::optional_at(&names, self::ml_array::number_index(at));
                    match (source_ml_source1_comparison_operand64.clone(), source_ml_source1_comparison_operand65.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false }
                }
            } {
                self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Return(String::from("copied callback captures need distinct actual lexical bindings"))
            } else {
                {
                    let before_2 = (before + 1f64);
                    {
                        before = before_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Done
        };
    }
}

pub fn source_ml_temporary_read_rust_owned_standard_sum_callback_move_capture_error_loop5(mut names: Vec<String>, mut roles: Vec<String>, mut identities: Vec<String>, mut layers: Vec<f64>, mut borrowed: Vec<bool>, mut at: f64) -> self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Result {
    loop {
        return if (at < ((names.len() as f64))) {
            if (((({
                let source_ml_source1_comparison_operand66 = self::ml_array::optional_at(&names, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand67 = String::from("");
                    match (source_ml_source1_comparison_operand66.clone(), source_ml_source1_comparison_operand67.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                }
            } || {
                let source_ml_source1_comparison_operand68 = self::ml_array::optional_at(&roles, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand69 = String::from("copy");
                    !(match (source_ml_source1_comparison_operand68.clone(), source_ml_source1_comparison_operand69.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            }) || {
                let source_ml_source1_comparison_operand70 = self::ml_array::optional_at(&identities, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand71 = String::from("");
                    match (source_ml_source1_comparison_operand70.clone(), source_ml_source1_comparison_operand71.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                }
            }) || {
                let source_ml_source1_comparison_operand72 = self::ml_array::optional_at(&layers, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand73 = 0f64;
                    !(match (source_ml_source1_comparison_operand72.clone(), source_ml_source1_comparison_operand73) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            }) || {
                let source_ml_source1_comparison_operand74 = self::ml_array::optional_at(&borrowed, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand75 = false;
                    !(match (source_ml_source1_comparison_operand74.clone(), source_ml_source1_comparison_operand75) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            }) {
                self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Return(String::from("a moved callback capture needs its actual complete direct Copy value without borrowed-place or loan provenance"))
            } else {
                {
                    let before = 0f64;
                    {
                        let source_ml_source1_s76 = self::source_ml_temporary_read_rust_owned_standard_sum_callback_move_capture_error_loop6(names.clone(), at, before);
                        match source_ml_source1_s76.clone() {
                            self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Done => {
                                {
                                    let at_2 = (at + 1f64);
                                    {
                                        at = at_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop6Return(source_ml_temporary_result6) => {
                                self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Return(source_ml_temporary_result6.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Result::MlTemporaryReadRustOwnedStandardSumCallbackMoveCaptureErrorLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Result {
    MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Done,
    MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Return(bool),
}

pub fn source_ml_temporary_accept_rust_owned_standard_sum_callback_move_closure_copy_loop8(mut source_ml_temporary_values7: Vec<String>, mut source_ml_temporary_index7: self::ml::Big) -> self::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Result {
    loop {
        return if (source_ml_temporary_index7 < (self::ml::Big::from_u128(source_ml_temporary_values7.len() as u128))) {
            {
                let role = self::ml_array::at(&source_ml_temporary_values7, source_ml_temporary_index7.to_index());
                if ((role != "copy") && (role != "owned")) {
                    self::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Result::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Return(false)
                } else {
                    {
                        let source_ml_temporary_index7_2 = source_ml_temporary_index7.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index7 = source_ml_temporary_index7_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Result::MlTemporaryAcceptRustOwnedStandardSumCallbackMoveClosureCopyLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Result {
    MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Done,
    MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Return(bool),
}

pub fn source_ml_temporary_accept_rust_owned_standard_sum_callback_closure_copy_loop10(mut source_ml_temporary_values9: Vec<String>, mut source_ml_temporary_index9: self::ml::Big) -> self::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Result {
    loop {
        return if (source_ml_temporary_index9 < (self::ml::Big::from_u128(source_ml_temporary_values9.len() as u128))) {
            {
                let role = self::ml_array::at(&source_ml_temporary_values9, source_ml_temporary_index9.to_index());
                if ((role != "copy") && (role != "owned")) {
                    self::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Result::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Return(false)
                } else {
                    {
                        let source_ml_temporary_index9_2 = source_ml_temporary_index9.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index9 = source_ml_temporary_index9_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Result::MlTemporaryAcceptRustOwnedStandardSumCallbackClosureCopyLoop10Done
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_standard_sum_callback_selection_loop11(mut values: Vec<String>, mut at: f64, mut layers: f64) -> f64 {
    loop {
        return if ({
            let source_ml_source1_comparison_operand77 = self::ml_array::optional_at(&values, self::ml_array::number_index(at));
            {
                let source_ml_source1_comparison_operand78 = String::from("(");
                match (source_ml_source1_comparison_operand77.clone(), source_ml_source1_comparison_operand78.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        } && (layers < (256f64))) {
            {
                let at_2 = (at + 1f64);
                {
                    let layers_2 = (layers + 1f64);
                    {
                        (at, layers) = (at_2, layers_2);
                        continue;
                    }
                }
            }
        } else {
            at
        };
    }
}

pub fn source_ml_temporary_plan_rust_owned_standard_sum_callback_selection_loop13(mut values: Vec<String>, mut kinds: Vec<String>, mut at: f64, mut parts: f64) -> f64 {
    loop {
        return if (({
            let source_ml_source1_comparison_operand79 = self::ml_array::optional_at(&values, self::ml_array::number_index(at));
            {
                let source_ml_source1_comparison_operand80 = String::from("::");
                match (source_ml_source1_comparison_operand79.clone(), source_ml_source1_comparison_operand80.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        } && {
            let source_ml_source1_comparison_operand81 = self::ml_array::optional_at(&kinds, self::ml_array::number_index((at + 1f64)));
            {
                let source_ml_source1_comparison_operand82 = String::from("identifier");
                match (source_ml_source1_comparison_operand81.clone(), source_ml_source1_comparison_operand82.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) && (parts < (256f64))) {
            {
                let at_2 = (at + 2f64);
                {
                    let parts_2 = (parts + 1f64);
                    {
                        (at, parts) = (at_2, parts_2);
                        continue;
                    }
                }
            }
        } else {
            at
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14 {
    MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14Next(f64),
    MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12 {
    MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Next(f64, f64),
    MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15 {
    MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15Next(f64),
    MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15Return(Vec<f64>),
}

pub fn source_ml_temporary_plan_rust_owned_standard_sum_callback_selection_loop16(mut values: Vec<String>, mut at: f64, mut budget: f64) -> f64 {
    loop {
        return if (((at < ((values.len() as f64))) && {
            let source_ml_source1_comparison_operand83 = self::ml_array::optional_at(&values, self::ml_array::number_index(at));
            {
                let source_ml_source1_comparison_operand84 = String::from("|");
                !(match (source_ml_source1_comparison_operand83.clone(), source_ml_source1_comparison_operand84.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
            }
        }) && (budget < (1024f64))) {
            {
                let at_2 = (at + 1f64);
                {
                    let budget_2 = (budget + 1f64);
                    {
                        (at, budget) = (at_2, budget_2);
                        continue;
                    }
                }
            }
        } else {
            at
        };
    }
}

fn ml_main() {
    assert!(((self::accept_rust_owned_standard_sum_callback_closure_copy(vec![String::from("owned")], String::from("owned"), 0f64, 0f64)) == (true)), "assertion 1");
    assert!(((self::accept_rust_owned_standard_sum_callback_closure_copy(vec![String::from("owned")], String::from("copy"), 0f64, 0f64)) == (true)), "assertion 2");
    assert!(((self::accept_rust_owned_standard_sum_callback_closure_copy(vec![String::from("owned")], String::from("owned"), 1f64, 0f64)) == (false)), "assertion 3");
    assert!(((self::accept_rust_owned_standard_sum_callback_closure_copy(vec![String::from("owned")], String::from("owned"), 0f64, 1f64)) == (false)), "assertion 4");
    assert!(((self::accept_rust_owned_standard_sum_callback_closure_copy(vec![String::from("borrowed")], String::from("owned"), 0f64, 0f64)) == (false)), "assertion 5");
    assert!(((self::accept_rust_owned_standard_sum_callback_closure_copy(vec![String::from("owned")], String::from(""), 0f64, 0f64)) == (false)), "assertion 6");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_arity(String::from("std::option::Option"), String::from("unwrap_or_else")), 1f64), "assertion 7");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_arity(String::from("std::option::Option"), String::from("ok_or_else")), 1f64), "assertion 8");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_arity(String::from("std::option::Option"), String::from("map")), 1f64), "assertion 9");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_arity(String::from("std::result::Result"), String::from("map_err")), 1f64), "assertion 10");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_arity(String::from("std::result::Result"), String::from("and_then")), 1f64), "assertion 11");
    assert!(((self::read_rust_owned_standard_sum_callback_version_error(String::from("std::option::Option"), String::from("map"), 90f64)) == ""), "assertion 12");
    assert!(((self::read_rust_owned_standard_sum_callback_version_error(String::from("std::result::Result"), String::from("map_err"), 99f64)) == ""), "assertion 13");
    assert!(((self::read_rust_owned_standard_sum_callback_version_error(String::from("std::option::Option"), String::from("map"), 89f64)) == "the owned standard sum callback source version is uncovered"), "assertion 14");
    assert!(((self::read_rust_owned_standard_sum_callback_version_error(String::from("std::result::Result"), String::from("map_err"), 100f64)) == "the owned standard sum callback source version is uncovered"), "assertion 15");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_parameter(String::from("std::option::Option"), String::from("unwrap_or_else")), (-1f64)), "assertion 16");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_parameter(String::from("std::option::Option"), String::from("map")), 0f64), "assertion 17");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_parameter(String::from("std::result::Result"), String::from("unwrap_or_else")), 1f64), "assertion 18");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_parameter(String::from("std::result::Result"), String::from("map")), 0f64), "assertion 19");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_parameter(String::from("std::result::Result"), String::from("map_err")), 1f64), "assertion 20");
    assert!(((self::read_rust_owned_standard_sum_callback_result(String::from("std::option::Option"), String::from("unwrap_or_else"))) == "payload"), "assertion 21");
    assert!(((self::read_rust_owned_standard_sum_callback_result(String::from("std::option::Option"), String::from("ok_or_else"))) == "std::result::Result"), "assertion 22");
    assert!(((self::plan_rust_owned_standard_sum_callback_arguments(String::from("std::option::Option"), String::from("ok_or_else"))) == (vec![0f64, (-1f64)])), "assertion 23");
    assert!(((self::plan_rust_owned_standard_sum_callback_arguments(String::from("std::option::Option"), String::from("map"))) == (vec![(-1f64)])), "assertion 24");
    assert!(((self::plan_rust_owned_standard_sum_callback_arguments(String::from("std::result::Result"), String::from("map"))) == (vec![(-1f64), 1f64])), "assertion 25");
    assert!(((self::plan_rust_owned_standard_sum_callback_arguments(String::from("std::result::Result"), String::from("map_err"))) == (vec![0f64, (-1f64)])), "assertion 26");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::option::Option"), String::from("Some"), String::from("unwrap_or_else"))) == (vec![String::from(""), String::from("payload")])), "assertion 27");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::option::Option"), String::from("None"), String::from("unwrap_or_else"))) == (vec![String::from(""), String::from("invoke")])), "assertion 28");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::option::Option"), String::from("Some"), String::from("ok_or_else"))) == (vec![String::from("Ok"), String::from("payload")])), "assertion 29");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::option::Option"), String::from("None"), String::from("ok_or_else"))) == (vec![String::from("Err"), String::from("invoke")])), "assertion 30");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::option::Option"), String::from("Some"), String::from("map"))) == (vec![String::from("Some"), String::from("invoke")])), "assertion 31");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::option::Option"), String::from("None"), String::from("map"))) == (vec![String::from("None"), String::from("empty")])), "assertion 32");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::result::Result"), String::from("Ok"), String::from("unwrap_or_else"))) == (vec![String::from(""), String::from("payload")])), "assertion 33");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::result::Result"), String::from("Err"), String::from("unwrap_or_else"))) == (vec![String::from(""), String::from("invoke")])), "assertion 34");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::result::Result"), String::from("Ok"), String::from("map"))) == (vec![String::from("Ok"), String::from("invoke")])), "assertion 35");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::result::Result"), String::from("Err"), String::from("map"))) == (vec![String::from("Err"), String::from("payload")])), "assertion 36");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::result::Result"), String::from("Ok"), String::from("map_err"))) == (vec![String::from("Ok"), String::from("payload")])), "assertion 37");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::result::Result"), String::from("Err"), String::from("map_err"))) == (vec![String::from("Err"), String::from("invoke")])), "assertion 38");
    assert!(((self::read_rust_owned_standard_sum_callback_closure_header_error(true, vec![String::from("value")], vec![String::from("owned")], String::from("owned"))) == ""), "assertion 39");
    assert!(((self::read_rust_owned_standard_sum_callback_closure_header_error(true, vec![String::from("value")], vec![String::from("owned")], String::from("copy"))) == ""), "assertion 40");
    assert!(((self::read_rust_owned_standard_sum_callback_closure_header_error(true, vec![String::from("value")], vec![String::from("borrowed")], String::from("owned"))) == "the callback parameter needs its actual direct Copy or supported owned source domain"), "assertion 41");
    assert!(((self::read_rust_owned_standard_sum_callback_closure_header_error(true, Vec::<String>::new(), Vec::<String>::new(), String::from(""))) == "the callback result needs its actual direct Copy or supported owned source domain"), "assertion 42");
    assert!(((self::read_rust_owned_standard_sum_callback_closure_header_error(true, vec![String::from("same"), String::from("same")], vec![String::from("copy"), String::from("owned")], String::from("copy"))) == "callback parameters need distinct source bindings"), "assertion 43");
    assert!(((self::read_rust_owned_standard_sum_callback_capture_error(0f64, 0f64)) == ""), "assertion 44");
    assert!(((self::read_rust_owned_standard_sum_callback_capture_error(1f64, 0f64)) == "owned-domain callbacks currently need actual capture-free source closures"), "assertion 45");
    assert!(((self::read_rust_owned_standard_sum_callback_capture_error(0f64, 1f64)) == "owned-domain callbacks currently need actual capture-free source closures"), "assertion 46");
    assert!(((self::read_rust_owned_standard_sum_callback_source_error(String::from("function"), 1f64, 1f64, 0f64, true, String::from("owned"))) == ""), "assertion 47");
    assert!(((self::read_rust_owned_standard_sum_callback_source_error(String::from("function"), 1f64, 1f64, 0f64, true, String::from("copy"))) == ""), "assertion 48");
    assert!(((self::read_rust_owned_standard_sum_callback_source_error(String::from("function"), 1f64, 1f64, 0f64, false, String::from("owned"))) == "the callback needs its actual Copy callable and complete movable parameter/result contracts"), "assertion 49");
    assert!(((self::read_rust_owned_standard_sum_callback_source_error(String::from("function"), 0f64, 1f64, 0f64, true, String::from("owned"))) == "the callback needs its actual Copy callable and complete movable parameter/result contracts"), "assertion 50");
    assert!(((self::read_rust_owned_standard_sum_callback_move_capture_error(false, Vec::<String>::new(), Vec::<String>::new(), Vec::<String>::new(), Vec::<f64>::new(), Vec::<bool>::new(), 0f64)) == ""), "assertion 51");
    assert!(((self::read_rust_owned_standard_sum_callback_move_capture_error(true, vec![String::from("flag")], vec![String::from("copy")], vec![String::from("actual bool")], vec![0f64], vec![false], 0f64)) == ""), "assertion 52");
    assert!(((self::read_rust_owned_standard_sum_callback_move_capture_error(false, vec![String::from("flag")], vec![String::from("copy")], vec![String::from("actual bool")], vec![0f64], vec![false], 0f64)) == "captured owned-domain callbacks need an explicit move source closure"), "assertion 53");
    assert!(((self::read_rust_owned_standard_sum_callback_move_capture_error(true, vec![String::from("text")], vec![String::from("owned")], vec![String::from("actual String")], vec![0f64], vec![false], 0f64)) == "a moved callback capture needs its actual complete direct Copy value without borrowed-place or loan provenance"), "assertion 54");
    assert!(((self::read_rust_owned_standard_sum_callback_move_capture_error(true, vec![String::from("flag")], vec![String::from("copy")], vec![String::from("")], vec![0f64], vec![false], 0f64)) == "a moved callback capture needs its actual complete direct Copy value without borrowed-place or loan provenance"), "assertion 55");
    assert!(((self::read_rust_owned_standard_sum_callback_move_capture_error(true, vec![String::from("flag")], vec![String::from("copy")], vec![String::from("actual bool")], vec![1f64], vec![false], 0f64)) == "a moved callback capture needs its actual complete direct Copy value without borrowed-place or loan provenance"), "assertion 56");
    assert!(((self::read_rust_owned_standard_sum_callback_move_capture_error(true, vec![String::from("flag")], vec![String::from("copy")], vec![String::from("actual bool")], vec![0f64], vec![true], 0f64)) == "a moved callback capture needs its actual complete direct Copy value without borrowed-place or loan provenance"), "assertion 57");
    assert!(((self::read_rust_owned_standard_sum_callback_move_capture_error(true, vec![String::from("flag")], vec![String::from("copy")], vec![String::from("actual bool")], vec![0f64], vec![false], 1f64)) == "mutable callback captures need their actual environment state interpretation"), "assertion 58");
    assert!(((self::read_rust_owned_standard_sum_callback_move_capture_error(true, vec![String::from("flag")], Vec::<String>::new(), vec![String::from("actual bool")], vec![0f64], vec![false], 0f64)) == "the complete copied capture environment needs every actual source binding"), "assertion 59");
    assert!(((self::read_rust_owned_standard_sum_callback_move_capture_error(true, vec![String::from("flag"), String::from("flag")], vec![String::from("copy"), String::from("copy")], vec![String::from("actual bool"), String::from("actual bool")], vec![0f64, 0f64], vec![false, false], 0f64)) == "copied callback captures need distinct actual lexical bindings"), "assertion 60");
    assert!(((self::accept_rust_owned_standard_sum_callback_move_closure_copy(vec![String::from("owned")], String::from("owned"), true, vec![String::from("flag")], vec![String::from("copy")], vec![String::from("actual bool")], vec![0f64], vec![false], 0f64)) == (true)), "assertion 61");
    assert!(((self::accept_rust_owned_standard_sum_callback_move_closure_copy(vec![String::from("owned")], String::from("copy"), true, vec![String::from("flag")], vec![String::from("copy")], vec![String::from("actual bool")], vec![0f64], vec![false], 0f64)) == (true)), "assertion 62");
    assert!(((self::accept_rust_owned_standard_sum_callback_move_closure_copy(vec![String::from("owned")], String::from("owned"), false, Vec::<String>::new(), Vec::<String>::new(), Vec::<String>::new(), Vec::<f64>::new(), Vec::<bool>::new(), 0f64)) == (true)), "assertion 63");
    assert!(((self::accept_rust_owned_standard_sum_callback_move_closure_copy(vec![String::from("owned")], String::from("owned"), false, vec![String::from("flag")], vec![String::from("copy")], vec![String::from("actual bool")], vec![0f64], vec![false], 0f64)) == (false)), "assertion 64");
    assert!(((self::accept_rust_owned_standard_sum_callback_move_closure_copy(vec![String::from("owned")], String::from("owned"), true, vec![String::from("text")], vec![String::from("owned")], vec![String::from("actual String")], vec![0f64], vec![false], 0f64)) == (false)), "assertion 65");
    assert!(((self::accept_rust_owned_standard_sum_callback_move_closure_copy(vec![String::from("owned")], String::from("owned"), true, vec![String::from("flag")], vec![String::from("copy")], vec![String::from("")], vec![0f64], vec![false], 0f64)) == (false)), "assertion 66");
    assert!(((self::read_rust_owned_standard_sum_callback_immediate_capture_error(0f64, String::from("name"), false, false, 0f64)) == ""), "assertion 67");
    assert!(((self::read_rust_owned_standard_sum_callback_immediate_capture_error(1f64, String::from("lambda"), true, true, 1f64)) == ""), "assertion 68");
    assert!(((self::read_rust_owned_standard_sum_callback_immediate_capture_error(1f64, String::from("name"), true, true, 1f64)) == "a copied callback environment needs its actual explicit move Lambda source and span identity"), "assertion 69");
    assert!(((self::read_rust_owned_standard_sum_callback_immediate_capture_error(1f64, String::from("lambda"), false, true, 1f64)) == "a copied callback environment needs its actual explicit move Lambda source and span identity"), "assertion 70");
    assert!(((self::read_rust_owned_standard_sum_callback_immediate_capture_error(1f64, String::from("lambda"), true, false, 1f64)) == "a copied callback environment needs its actual explicit move Lambda source and span identity"), "assertion 71");
    assert!(((self::read_rust_owned_standard_sum_callback_immediate_capture_error(1f64, String::from("lambda"), true, true, 2f64)) == "a copied callback environment needs its actual explicit move Lambda source and span identity"), "assertion 72");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_arity(String::from("std::option::Option"), String::from("and_then")), 1f64), "assertion 73");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_arity(String::from("std::option::Option"), String::from("or_else")), 1f64), "assertion 74");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_arity(String::from("std::result::Result"), String::from("and_then")), 1f64), "assertion 75");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_arity(String::from("std::result::Result"), String::from("or_else")), 1f64), "assertion 76");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_parameter(String::from("std::option::Option"), String::from("and_then")), 0f64), "assertion 77");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_parameter(String::from("std::option::Option"), String::from("or_else")), (-1f64)), "assertion 78");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_parameter(String::from("std::result::Result"), String::from("and_then")), 0f64), "assertion 79");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_parameter(String::from("std::result::Result"), String::from("or_else")), 1f64), "assertion 80");
    assert!(((self::read_rust_owned_standard_sum_callback_result(String::from("std::option::Option"), String::from("and_then"))) == "bound sum"), "assertion 81");
    assert!(((self::read_rust_owned_standard_sum_callback_result(String::from("std::result::Result"), String::from("or_else"))) == "bound sum"), "assertion 82");
    assert!(((self::plan_rust_owned_standard_sum_callback_arguments(String::from("std::option::Option"), String::from("and_then"))) == (Vec::<f64>::new())), "assertion 83");
    assert!(((self::plan_rust_owned_standard_sum_callback_arguments(String::from("std::result::Result"), String::from("or_else"))) == (Vec::<f64>::new())), "assertion 84");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_retained_argument(String::from("std::option::Option"), String::from("and_then")), (-1f64)), "assertion 85");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_retained_argument(String::from("std::option::Option"), String::from("or_else")), 0f64), "assertion 86");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_retained_argument(String::from("std::result::Result"), String::from("and_then")), 1f64), "assertion 87");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_retained_argument(String::from("std::result::Result"), String::from("or_else")), 0f64), "assertion 88");
    assert!(((self::read_rust_owned_standard_sum_callback_bound_result_error(String::from("std::option::Option"), String::from("std::option::Option"), 0f64, true)) == ""), "assertion 89");
    assert!(((self::read_rust_owned_standard_sum_callback_bound_result_error(String::from("std::result::Result"), String::from("std::result::Result"), 0f64, true)) == ""), "assertion 90");
    assert!(((self::read_rust_owned_standard_sum_callback_bound_result_error(String::from("std::option::Option"), String::from("std::result::Result"), 0f64, true)) == "the callback must return its actual direct same-family sum and preserve every retained argument identity"), "assertion 91");
    assert!(((self::read_rust_owned_standard_sum_callback_bound_result_error(String::from("std::result::Result"), String::from("std::result::Result"), 0f64, false)) == "the callback must return its actual direct same-family sum and preserve every retained argument identity"), "assertion 92");
    assert!(((self::read_rust_owned_standard_sum_callback_bound_result_error(String::from("std::option::Option"), String::from("std::option::Option"), 1f64, true)) == "the callback must return its actual direct same-family sum and preserve every retained argument identity"), "assertion 93");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::option::Option"), String::from("Some"), String::from("and_then"))) == (vec![String::from(""), String::from("invoke")])), "assertion 94");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::option::Option"), String::from("None"), String::from("and_then"))) == (vec![String::from("None"), String::from("empty")])), "assertion 95");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::option::Option"), String::from("Some"), String::from("or_else"))) == (vec![String::from("Some"), String::from("payload")])), "assertion 96");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::option::Option"), String::from("None"), String::from("or_else"))) == (vec![String::from(""), String::from("invoke")])), "assertion 97");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::result::Result"), String::from("Ok"), String::from("and_then"))) == (vec![String::from(""), String::from("invoke")])), "assertion 98");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::result::Result"), String::from("Err"), String::from("and_then"))) == (vec![String::from("Err"), String::from("payload")])), "assertion 99");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::result::Result"), String::from("Ok"), String::from("or_else"))) == (vec![String::from("Ok"), String::from("payload")])), "assertion 100");
    assert!(((self::plan_rust_owned_standard_sum_callback_branch(String::from("std::result::Result"), String::from("Err"), String::from("or_else"))) == (vec![String::from(""), String::from("invoke")])), "assertion 101");
    assert!(self::ml_number::same_value(self::read_rust_owned_standard_sum_callback_selection_token_limit(), 4096f64), "assertion 102");
    assert!(((self::plan_rust_owned_standard_sum_callback_selection(vec![String::from("("), String::from("mapper"), String::from(")")], vec![String::from("punctuation"), String::from("identifier"), String::from("punctuation")])) == (vec![1f64, 1f64, 2f64])), "assertion 103");
    assert!(((self::plan_rust_owned_standard_sum_callback_selection(vec![String::from("("), String::from("factory"), String::from("("), String::from(")"), String::from(")")], vec![String::from("punctuation"), String::from("identifier"), String::from("punctuation"), String::from("punctuation"), String::from("punctuation")])) == (vec![2f64, 1f64, 2f64])), "assertion 104");
    assert!(((self::plan_rust_owned_standard_sum_callback_selection(vec![String::from("("), String::from("module"), String::from("::"), String::from("mapper"), String::from(")")], vec![String::from("punctuation"), String::from("identifier"), String::from("punctuation"), String::from("identifier"), String::from("punctuation")])) == (vec![1f64, 1f64, 4f64])), "assertion 105");
    assert!(((self::plan_rust_owned_standard_sum_callback_selection(vec![String::from("("), String::from("|"), String::from("value"), String::from(":"), String::from("u8"), String::from("|"), String::from("->"), String::from("String"), String::from("{")], vec![String::from("punctuation"), String::from("punctuation"), String::from("identifier"), String::from("punctuation"), String::from("identifier"), String::from("punctuation"), String::from("punctuation"), String::from("identifier"), String::from("punctuation")])) == (vec![3f64, 7f64, 8f64])), "assertion 106");
    assert!(((self::plan_rust_owned_standard_sum_callback_selection(vec![String::from("("), String::from("move"), String::from("||"), String::from("->"), String::from("std"), String::from("::"), String::from("string"), String::from("::"), String::from("String"), String::from("{")], vec![String::from("punctuation"), String::from("keyword"), String::from("punctuation"), String::from("punctuation"), String::from("identifier"), String::from("punctuation"), String::from("identifier"), String::from("punctuation"), String::from("identifier"), String::from("punctuation")])) == (vec![3f64, 4f64, 9f64])), "assertion 107");
    assert!(((self::plan_rust_owned_standard_sum_callback_selection(vec![String::from("("), String::from("|"), String::from("value"), String::from("|"), String::from("value"), String::from(")")], vec![String::from("punctuation"), String::from("punctuation"), String::from("identifier"), String::from("punctuation"), String::from("identifier"), String::from("punctuation")])) == (Vec::<f64>::new())), "assertion 108");
    assert!(((self::plan_rust_owned_standard_sum_callback_selection(vec![String::from("("), String::from("move"), String::from("mapper"), String::from(")")], vec![String::from("punctuation"), String::from("keyword"), String::from("identifier"), String::from("punctuation")])) == (Vec::<f64>::new())), "assertion 109");
    assert!(((self::plan_rust_owned_standard_sum_callback_selection(vec![String::from("("), String::from("object"), String::from("."), String::from("mapper"), String::from(")")], vec![String::from("punctuation"), String::from("identifier"), String::from("punctuation"), String::from("identifier"), String::from("punctuation")])) == (Vec::<f64>::new())), "assertion 110");
    assert!(((self::plan_rust_owned_standard_sum_callback_selection(vec![String::from("("), String::from("||"), String::from("->"), String::from("&"), String::from("String"), String::from("{")], vec![String::from("punctuation"), String::from("punctuation"), String::from("punctuation"), String::from("punctuation"), String::from("identifier"), String::from("punctuation")])) == (Vec::<f64>::new())), "assertion 111");
    assert!(((self::accept_rust_owned_standard_sum_callback_written_selection(String::from("library"), String::from("std::string::String"), false, false)) == (true)), "assertion 112");
    assert!(((self::accept_rust_owned_standard_sum_callback_written_selection(String::from("library"), String::from("std::vec::Vec"), false, false)) == (true)), "assertion 113");
    assert!(((self::accept_rust_owned_standard_sum_callback_written_selection(String::from("declaration"), String::from("String"), false, false)) == (false)), "assertion 114");
    assert!(((self::accept_rust_owned_standard_sum_callback_written_selection(String::from("library"), String::from("std::string::String"), true, false)) == (false)), "assertion 115");
    assert!(((self::accept_rust_owned_standard_sum_callback_written_selection(String::from("library"), String::from("std::string::String"), false, true)) == (false)), "assertion 116");
    assert!(((self::accept_rust_owned_standard_sum_callback_written_selection(String::from("primitive"), String::from("u8"), false, false)) == (false)), "assertion 117");
}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(move || { ml_main() })
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}

#[test]
fn all_literal_owned_sum_callback_controls() { main(); }
