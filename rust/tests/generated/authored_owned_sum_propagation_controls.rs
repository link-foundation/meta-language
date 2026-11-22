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

pub fn read_rust_primitive_owner_path(segments: Vec<String>, ordinary: bool, shadowed: bool, absolute: bool) -> String {
    if (!ordinary || shadowed) {
        String::from("")
    } else {
        {
            let owner = String::from("");
            {
                let owner_4 = if ((((segments.len() as f64)) == (2f64)) && !absolute) {
                    {
                        let owner_2 = (self::ml_array::optional_at(&segments, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| String::from(""));
                        owner_2.clone()
                    }
                } else {
                    if (((((segments.len() as f64)) == (4f64)) && ({
                        let source_ml_source2_comparison_operand1 = self::ml_array::optional_at(&segments, self::ml_array::number_index(0f64));
                        {
                            let source_ml_source2_comparison_operand2 = String::from("std");
                            match (source_ml_source2_comparison_operand1.clone(), source_ml_source2_comparison_operand2.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        }
                    } || {
                        let source_ml_source2_comparison_operand3 = self::ml_array::optional_at(&segments, self::ml_array::number_index(0f64));
                        {
                            let source_ml_source2_comparison_operand4 = String::from("core");
                            match (source_ml_source2_comparison_operand3.clone(), source_ml_source2_comparison_operand4.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        }
                    })) && {
                        let source_ml_source2_comparison_operand5 = self::ml_array::optional_at(&segments, self::ml_array::number_index(1f64));
                        {
                            let source_ml_source2_comparison_operand6 = String::from("primitive");
                            match (source_ml_source2_comparison_operand5.clone(), source_ml_source2_comparison_operand6.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        }
                    }) {
                        {
                            let owner_3 = (self::ml_array::optional_at(&segments, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| String::from(""));
                            owner_3.clone()
                        }
                    } else {
                        owner.clone()
                    }
                };
                if self::ml_array::includes(&vec![String::from("bool"), String::from("i8"), String::from("u8"), String::from("i16"), String::from("u16"), String::from("i32"), String::from("u32"), String::from("i64"), String::from("u64"), String::from("i128"), String::from("u128"), String::from("isize"), String::from("usize")], &owner_4.clone(), 0f64, false, |a, b| a == b) {
                    owner_4.clone()
                } else {
                    String::from("")
                }
            }
        }
    }
}

pub fn accept_rust_source_signature_model(model: String) -> bool {
    ((model == "") || (model == "Rust source function signature 1"))
}

pub fn accept_rust_source_signature_wrapper(wrapper: String) -> bool {
    ((wrapper == "reference") || (wrapper == "box"))
}

pub fn accept_rust_source_signature_owner_kind(kind: String) -> bool {
    ((((((kind == "primitive") || (kind == "declaration")) || (kind == "library")) || (kind == "tuple")) || (kind == "unit")) || (kind == "slice"))
}

pub fn accept_rust_source_signature_catalog_owner(kind: String) -> bool {
    ((((kind == "data") || (kind == "module")) || (kind == "externalCrate")) || (kind == "ctor"))
}

pub fn accept_rust_source_signature_primitive(kind: String, path: String, bits: String, signed: bool) -> bool {
    if (path == "f64") {
        (kind == "float")
    } else {
        if (path == "bool") {
            (kind == "bool")
        } else {
            if (path == "str") {
                (kind == "string")
            } else {
                if (path == "i8") {
                    (((kind == "fixed") && (bits == "8")) && (signed == (true)))
                } else {
                    if (path == "u8") {
                        (((kind == "fixed") && (bits == "8")) && (signed == (false)))
                    } else {
                        if (path == "i16") {
                            (((kind == "fixed") && (bits == "16")) && (signed == (true)))
                        } else {
                            if (path == "u16") {
                                (((kind == "fixed") && (bits == "16")) && (signed == (false)))
                            } else {
                                if (path == "i32") {
                                    (((kind == "fixed") && (bits == "32")) && (signed == (true)))
                                } else {
                                    if (path == "u32") {
                                        (((kind == "fixed") && (bits == "32")) && (signed == (false)))
                                    } else {
                                        if (path == "i64") {
                                            (((kind == "fixed") && (bits == "64")) && (signed == (true)))
                                        } else {
                                            if (path == "u64") {
                                                (((kind == "fixed") && (bits == "64")) && (signed == (false)))
                                            } else {
                                                if (path == "i128") {
                                                    (((kind == "fixed") && (bits == "128")) && (signed == (true)))
                                                } else {
                                                    if (path == "u128") {
                                                        (((kind == "fixed") && (bits == "128")) && (signed == (false)))
                                                    } else {
                                                        if (path == "usize") {
                                                            (((kind == "fixed") && (bits == "64")) && (signed == (false)))
                                                        } else {
                                                            if (path == "isize") {
                                                                (((kind == "fixed") && (bits == "64")) && (signed == (true)))
                                                            } else {
                                                                false
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

pub fn read_rust_primitive_selection_call_role(path: String, ordinary: bool, shadowed: bool) -> String {
    if (ordinary && !shadowed) {
        if (path == "std::cmp::min") {
            String::from("min")
        } else {
            if (path == "std::cmp::max") {
                String::from("max")
            } else {
                String::from("")
            }
        }
    } else {
        String::from("")
    }
}

pub fn read_rust_primitive_selection_comparison(method: String, ordinary: bool) -> String {
    if ordinary {
        if (method == "min") {
            String::from("le")
        } else {
            if (method == "max") {
                String::from("gt")
            } else {
                String::from("")
            }
        }
    } else {
        String::from("")
    }
}

pub fn read_rust_primitive_selection_condition(method: String, kind: String) -> String {
    if (kind == "bool") {
        if (method == "min") {
            String::from("not")
        } else {
            if (method == "max") {
                String::from("identity")
            } else {
                String::from("")
            }
        }
    } else {
        if (kind == "fixed") {
            self::read_rust_primitive_selection_comparison(method.clone(), true)
        } else {
            String::from("")
        }
    }
}

pub fn accept_rust_primitive_selection_receiver(kind: String, bits: String, signed: bool, owner: String, path: String, children: f64, proven: bool, layers: f64) -> bool {
    (((((((kind == "fixed") || (kind == "bool")) && (owner == "primitive")) && (children == (0f64))) && proven) && (layers == (0f64))) && self::accept_rust_source_signature_primitive(kind.clone(), path.clone(), bits.clone(), signed))
}

pub fn accept_rust_primitive_selection_argument(expected: String, bits: String, signed: bool, owner: String, actual: String, actual_bits: String, actual_signed: bool, actual_owner: String, layers: f64, proven: bool, known: bool, literal: bool) -> bool {
    if (layers != (0f64)) {
        false
    } else {
        if (((literal && (expected == "fixed")) && (actual == "")) && (actual_owner == "")) {
            true
        } else {
            (((((known && proven) && (expected == actual)) && (bits == actual_bits)) && (signed == actual_signed)) && (owner == actual_owner))
        }
    }
}

pub fn accept_rust_primitive_from_method(owner: String, method: String) -> bool {
    ((method == "from") && self::ml_array::includes(&vec![String::from("bool"), String::from("i8"), String::from("u8"), String::from("i16"), String::from("u16"), String::from("i32"), String::from("u32"), String::from("i64"), String::from("u64"), String::from("i128"), String::from("u128"), String::from("isize"), String::from("usize"), String::from("f64")], &owner.clone(), 0f64, false, |a, b| a == b))
}

pub fn read_rust_primitive_from_owner(segments: Vec<String>, ordinary: bool, shadowed: bool, absolute: bool) -> String {
    if (!ordinary || shadowed) {
        String::from("")
    } else {
        if ((((((segments.len() as f64)) == (2f64)) && !absolute) && {
            let source_ml_source2_comparison_operand7 = self::ml_array::optional_at(&segments, self::ml_array::number_index(0f64));
            {
                let source_ml_source2_comparison_operand8 = String::from("f64");
                match (source_ml_source2_comparison_operand7.clone(), source_ml_source2_comparison_operand8.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) || ((((((segments.len() as f64)) == (4f64)) && ({
            let source_ml_source2_comparison_operand9 = self::ml_array::optional_at(&segments, self::ml_array::number_index(0f64));
            {
                let source_ml_source2_comparison_operand10 = String::from("std");
                match (source_ml_source2_comparison_operand9.clone(), source_ml_source2_comparison_operand10.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        } || {
            let source_ml_source2_comparison_operand11 = self::ml_array::optional_at(&segments, self::ml_array::number_index(0f64));
            {
                let source_ml_source2_comparison_operand12 = String::from("core");
                match (source_ml_source2_comparison_operand11.clone(), source_ml_source2_comparison_operand12.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        })) && {
            let source_ml_source2_comparison_operand13 = self::ml_array::optional_at(&segments, self::ml_array::number_index(1f64));
            {
                let source_ml_source2_comparison_operand14 = String::from("primitive");
                match (source_ml_source2_comparison_operand13.clone(), source_ml_source2_comparison_operand14.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) && {
            let source_ml_source2_comparison_operand15 = self::ml_array::optional_at(&segments, self::ml_array::number_index(2f64));
            {
                let source_ml_source2_comparison_operand16 = String::from("f64");
                match (source_ml_source2_comparison_operand15.clone(), source_ml_source2_comparison_operand16.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        })) {
            String::from("f64")
        } else {
            self::read_rust_primitive_owner_path(segments, ordinary, shadowed, absolute)
        }
    }
}

pub fn read_rust_primitive_from_literal_owner(form: String, typed: bool, kind: String) -> String {
    if ((form != "numeral") && (form != "negative numeral")) {
        String::from("")
    } else {
        if (kind == "float") {
            String::from("f64")
        } else {
            if !typed {
                String::from("i32")
            } else {
                String::from("")
            }
        }
    }
}

pub fn accept_rust_primitive_from_source(kind: String, bits: String, signed: bool, owner: String, path: String, children: f64, proven: bool, layers: f64) -> bool {
    (((((((((kind == "float") && (bits == "")) && !signed) && (owner == "primitive")) && (path == "f64")) && (children == (0f64))) && proven) && (layers == (0f64))) || self::accept_rust_primitive_selection_receiver(kind.clone(), bits.clone(), signed, owner.clone(), path.clone(), children, proven, layers))
}

pub fn plan_rust_primitive_from(target: String, source: String) -> String {
    if (!(self::accept_rust_primitive_from_method(target.clone(), String::from("from"))) || !(self::accept_rust_primitive_from_method(source.clone(), String::from("from")))) {
        String::from("")
    } else {
        if (target == source) {
            String::from("identity")
        } else {
            if (((source == "bool") && (target != "bool")) && (target != "f64")) {
                String::from("boolean integer")
            } else {
                if ((target == "f64") && self::ml_array::includes(&vec![String::from("i8"), String::from("i16"), String::from("i32"), String::from("u8"), String::from("u16"), String::from("u32")], &source.clone(), 0f64, false, |a, b| a == b)) {
                    String::from("floating exact")
                } else {
                    {
                        let admitted = (((((((((source == "u8") && self::ml_array::includes(&vec![String::from("u16"), String::from("u32"), String::from("u64"), String::from("u128"), String::from("usize"), String::from("i16"), String::from("i32"), String::from("i64"), String::from("i128"), String::from("isize")], &target.clone(), 0f64, false, |a, b| a == b)) || ((source == "u16") && self::ml_array::includes(&vec![String::from("u32"), String::from("u64"), String::from("u128"), String::from("usize"), String::from("i32"), String::from("i64"), String::from("i128")], &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "u32") && self::ml_array::includes(&vec![String::from("u64"), String::from("u128"), String::from("i64"), String::from("i128")], &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "u64") && self::ml_array::includes(&vec![String::from("u128"), String::from("i128")], &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i8") && self::ml_array::includes(&vec![String::from("i16"), String::from("i32"), String::from("i64"), String::from("i128"), String::from("isize")], &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i16") && self::ml_array::includes(&vec![String::from("i32"), String::from("i64"), String::from("i128"), String::from("isize")], &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i32") && self::ml_array::includes(&vec![String::from("i64"), String::from("i128")], &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i64") && (target == "i128")));
                        if admitted {
                            String::from("integer exact")
                        } else {
                            String::from("")
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_standard_sum_source_owner(path: String, standard_definition: bool, layers: f64) -> String {
    if (!standard_definition || (layers != (0f64))) {
        String::from("")
    } else {
        if ((path == "std::option::Option") || (path == "core::option::Option")) {
            String::from("std::option::Option")
        } else {
            if ((path == "std::result::Result") || (path == "core::result::Result")) {
                String::from("std::result::Result")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_rust_standard_sum_source_path_owner(path: String) -> String {
    if self::ml_array::includes(&vec![String::from("Option"), String::from("std::option::Option"), String::from("core::option::Option"), String::from("None"), String::from("Some"), String::from("Option::None"), String::from("Option::Some"), String::from("std::option::Option::None"), String::from("std::option::Option::Some"), String::from("core::option::Option::None"), String::from("core::option::Option::Some")], &path.clone(), 0f64, false, |a, b| a == b) {
        String::from("std::option::Option")
    } else {
        if self::ml_array::includes(&vec![String::from("Result"), String::from("std::result::Result"), String::from("core::result::Result"), String::from("Ok"), String::from("Err"), String::from("Result::Ok"), String::from("Result::Err"), String::from("std::result::Result::Ok"), String::from("std::result::Result::Err"), String::from("core::result::Result::Ok"), String::from("core::result::Result::Err")], &path.clone(), 0f64, false, |a, b| a == b) {
            String::from("std::result::Result")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_standard_sum_source_path_constructor(path: String) -> String {
    if self::ml_array::includes(&vec![String::from("None"), String::from("Option::None"), String::from("std::option::Option::None"), String::from("core::option::Option::None")], &path.clone(), 0f64, false, |a, b| a == b) {
        String::from("None")
    } else {
        if self::ml_array::includes(&vec![String::from("Some"), String::from("Option::Some"), String::from("std::option::Option::Some"), String::from("core::option::Option::Some")], &path.clone(), 0f64, false, |a, b| a == b) {
            String::from("Some")
        } else {
            if self::ml_array::includes(&vec![String::from("Ok"), String::from("Result::Ok"), String::from("std::result::Result::Ok"), String::from("core::result::Result::Ok")], &path.clone(), 0f64, false, |a, b| a == b) {
                String::from("Ok")
            } else {
                if self::ml_array::includes(&vec![String::from("Err"), String::from("Result::Err"), String::from("std::result::Result::Err"), String::from("core::result::Result::Err")], &path.clone(), 0f64, false, |a, b| a == b) {
                    String::from("Err")
                } else {
                    String::from("")
                }
            }
        }
    }
}

pub fn read_rust_standard_sum_source_instantiation_limit() -> f64 {
    256f64
}

pub fn read_rust_standard_sum_source_depth_limit() -> f64 {
    16f64
}

pub fn read_rust_standard_sum_source_specialization_name(ordinal: f64, collision: f64) -> String {
    if (((((!(self::ml_math::is_integer(ordinal)) || (ordinal < (0f64))) || (ordinal >= (256f64))) || !(self::ml_math::is_integer(collision))) || (collision < (0f64))) || (collision >= (4096f64))) {
        String::from("")
    } else {
        format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("ml_standard_sum_value")), ({
            let source_ml_source2_string_argument17 = ordinal;
            self::ml_number::js_number(source_ml_source2_string_argument17)
        }))), (String::from("_")))), ({
            let source_ml_source2_string_argument18 = collision;
            self::ml_number::js_number(source_ml_source2_string_argument18)
        }))
    }
}

pub fn read_rust_standard_sum_source_phase(header: bool, complete: bool, copies: bool) -> String {
    if header {
        String::from("provisional")
    } else {
        if (complete && copies) {
            String::from("admitted")
        } else {
            String::from("refuse")
        }
    }
}

pub fn read_rust_standard_sum_source_depth(children: Vec<f64>, sum: bool) -> f64 {
    {
        let depth = 0f64;
        {
            let source_ml_source1_temporary_values1 = children;
            {
                let source_ml_source1_temporary_index1 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s19 = self::source_ml_source1_temporary_read_rust_standard_sum_source_depth_loop2(depth, source_ml_source1_temporary_values1, source_ml_source1_temporary_index1.clone());
                    match source_ml_source2_s19.clone() {
                        self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Done(depth_2) => {
                            {
                                let result = (depth_2 + if sum {
                                    1f64
                                } else {
                                    0f64
                                });
                                if (result <= (16f64)) {
                                    result
                                } else {
                                    (-1f64)
                                }
                            }
                        }
                        self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Return(source_ml_source1_temporary_result2) => {
                            source_ml_source1_temporary_result2
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_standard_sum_source_materialization_error(nodes: f64, units: f64, additional_nodes: f64, additional_units: f64) -> String {
    {
        let source_ml_source1_temporary_values3 = vec![nodes, units, additional_nodes, additional_units];
        {
            let source_ml_source1_temporary_index3 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s20 = self::source_ml_source1_temporary_read_rust_standard_sum_source_materialization_error_loop4(source_ml_source1_temporary_values3, source_ml_source1_temporary_index3.clone());
                match source_ml_source2_s20.clone() {
                    self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Done => {
                        if ((((nodes > (1048576f64)) || (units > (8388608f64))) || (additional_nodes > ((1048576f64 - nodes)))) || (additional_units > ((8388608f64 - units)))) {
                            String::from("the complete standard sum source metadata exceeds its materialization budget")
                        } else {
                            String::from("")
                        }
                    }
                    self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Return(source_ml_source1_temporary_result4) => {
                        source_ml_source1_temporary_result4.clone()
                    }
                }
            }
        }
    }
}

pub fn read_rust_standard_sum_source_argument_count(owner: String) -> f64 {
    if (owner == "std::option::Option") {
        1f64
    } else {
        if (owner == "std::result::Result") {
            2f64
        } else {
            (-1f64)
        }
    }
}

pub fn accept_rust_standard_sum_source_arguments(owner: String, identities: Vec<String>, direct_copies: Vec<bool>) -> bool {
    {
        let count = self::read_rust_standard_sum_source_argument_count(owner.clone());
        if (((count < (0f64)) || (((identities.len() as f64)) != count)) || (((direct_copies.len() as f64)) != count)) {
            false
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source2_s21 = self::source_ml_source1_temporary_accept_rust_standard_sum_source_arguments_loop5(identities, direct_copies, count, at);
                    match source_ml_source2_s21.clone() {
                        self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Return(source_ml_source1_temporary_result5) => {
                            source_ml_source1_temporary_result5
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_standard_sum_source_constructors(owner: String) -> Vec<String> {
    if (owner == "std::option::Option") {
        vec![String::from("None"), String::from("Some")]
    } else {
        if (owner == "std::result::Result") {
            vec![String::from("Ok"), String::from("Err")]
        } else {
            Vec::<String>::new()
        }
    }
}

pub fn read_rust_standard_sum_source_field_argument(owner: String, constructor: String) -> f64 {
    if (owner == "std::option::Option") {
        if (constructor == "None") {
            (-1f64)
        } else {
            if (constructor == "Some") {
                0f64
            } else {
                (-2f64)
            }
        }
    } else {
        if (owner == "std::result::Result") {
            if (constructor == "Ok") {
                0f64
            } else {
                if (constructor == "Err") {
                    1f64
                } else {
                    (-2f64)
                }
            }
        } else {
            (-2f64)
        }
    }
}

pub fn read_rust_standard_sum_source_constructor_arity(owner: String, constructor: String) -> f64 {
    {
        let argument = self::read_rust_standard_sum_source_field_argument(owner.clone(), constructor.clone());
        if (argument == ((-2f64))) {
            (-1f64)
        } else {
            if (argument == ((-1f64))) {
                0f64
            } else {
                1f64
            }
        }
    }
}

pub fn accept_rust_standard_sum_source_inferred_constructor(owner: String, constructor: String) -> bool {
    ((owner == "std::option::Option") && (constructor == "Some"))
}

pub fn accept_rust_standard_sum_question_context(operand_owner: String, return_owner: String, ordinary_callable: bool, direct_copy_operand: bool, operand_error: String, return_error: String) -> bool {
    ((ordinary_callable && direct_copy_operand) && self::accept_rust_standard_sum_question_family(operand_owner.clone(), return_owner.clone(), operand_error.clone(), return_error.clone()))
}

pub fn accept_rust_standard_sum_question_family(operand_owner: String, return_owner: String, operand_error: String, return_error: String) -> bool {
    if (operand_owner != return_owner) {
        false
    } else {
        if (operand_owner == "std::option::Option") {
            ((operand_error == "") && (return_error == ""))
        } else {
            (((operand_owner == "std::result::Result") && (operand_error != "")) && (operand_error == return_error))
        }
    }
}

pub fn accept_rust_standard_sum_owned_question_context(operand_owner: String, return_owner: String, ordinary: bool, role: String, wrappers: f64, operand_error: String, return_error: String) -> bool {
    (((ordinary && (role == "owned")) && (wrappers == (0f64))) && self::accept_rust_standard_sum_question_family(operand_owner.clone(), return_owner.clone(), operand_error.clone(), return_error.clone()))
}

pub fn read_rust_standard_sum_primitive_question_conversion_plan(operand_owner: String, return_owner: String, ordinary: bool, role: String, wrappers: f64, direct_copy: bool, source_known: bool, target_known: bool, source: String, target: String, minor: f64) -> Vec<String> {
    if ((((((((!ordinary || (operand_owner != "std::result::Result")) || (return_owner != "std::result::Result")) || (wrappers != (0f64))) || !source_known) || !target_known) || !(self::ml_math::is_integer(minor))) || (minor < (90f64))) || (minor >= (100f64))) {
        Vec::<String>::new()
    } else {
        if ((source == target) || ((role != "owned") && ((role != "copy") || !direct_copy))) {
            Vec::<String>::new()
        } else {
            {
                let plan = self::plan_rust_primitive_from(target.clone(), source.clone());
                if (plan == "") {
                    Vec::<String>::new()
                } else {
                    vec![role.clone(), target.clone(), plan.clone()]
                }
            }
        }
    }
}

pub fn accept_rust_standard_sum_primitive_question_completion(profile: String, consumed: bool) -> bool {
    ((profile == "copy") || ((profile == "owned") && consumed))
}

pub fn read_rust_standard_sum_question_branch(owner: String, constructor: String) -> String {
    if ((self::read_rust_standard_sum_source_constructor_arity(owner.clone(), constructor.clone())) < (0f64)) {
        String::from("")
    } else {
        if (owner == "std::option::Option") {
            if (constructor == "Some") {
                String::from("continue field")
            } else {
                String::from("return absent")
            }
        } else {
            if (constructor == "Ok") {
                String::from("continue field")
            } else {
                String::from("return error field")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result {
    MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Done(f64),
    MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Return(f64),
}

pub fn source_ml_source1_temporary_read_rust_standard_sum_source_depth_loop2(mut depth: f64, mut source_ml_source1_temporary_values1: Vec<f64>, mut source_ml_source1_temporary_index1: self::ml::Big) -> self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result {
    loop {
        return if (source_ml_source1_temporary_index1 < (self::ml::Big::from_u128(source_ml_source1_temporary_values1.len() as u128))) {
            {
                let child = self::ml_array::at(&source_ml_source1_temporary_values1, source_ml_source1_temporary_index1.to_index());
                if ((!(self::ml_math::is_integer(child)) || (child < (0f64))) || (child > (16f64))) {
                    self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Return((-1f64))
                } else {
                    if (child > depth) {
                        {
                            let depth_2 = child;
                            {
                                let source_ml_source1_temporary_index1_2 = source_ml_source1_temporary_index1.add(&self::ml::Big::from_i128(1));
                                {
                                    (depth, source_ml_source1_temporary_index1) = (depth_2, source_ml_source1_temporary_index1_2.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_source1_temporary_index1_3 = source_ml_source1_temporary_index1.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1 = source_ml_source1_temporary_index1_3.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Done(depth)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result {
    MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Done,
    MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Return(String),
}

pub fn source_ml_source1_temporary_read_rust_standard_sum_source_materialization_error_loop4(mut source_ml_source1_temporary_values3: Vec<f64>, mut source_ml_source1_temporary_index3: self::ml::Big) -> self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result {
    loop {
        return if (source_ml_source1_temporary_index3 < (self::ml::Big::from_u128(source_ml_source1_temporary_values3.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values3, source_ml_source1_temporary_index3.to_index());
                if (!(self::ml_math::is_integer(value)) || (value < (0f64))) {
                    self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Return(String::from("invalid standard sum source materialization count"))
                } else {
                    {
                        let source_ml_source1_temporary_index3_2 = source_ml_source1_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index3 = source_ml_source1_temporary_index3_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result {
    MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Done,
    MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Return(bool),
}

pub fn source_ml_source1_temporary_accept_rust_standard_sum_source_arguments_loop5(mut identities: Vec<String>, mut direct_copies: Vec<bool>, mut count: f64, mut at: f64) -> self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result {
    loop {
        return if (at < count) {
            if ((((self::ml_array::optional_at(&identities, self::ml_array::number_index(at))).clone().unwrap_or_else(|| String::from(""))) == "") || {
                let source_ml_source2_comparison_operand22 = self::ml_array::optional_at(&direct_copies, self::ml_array::number_index(at));
                {
                    let source_ml_source2_comparison_operand23 = true;
                    !(match (source_ml_source2_comparison_operand22.clone(), source_ml_source2_comparison_operand23) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            }) {
                self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Return(false)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Done
        };
    }
}

fn ml_main() {
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::option::Option"), String::from("std::option::Option"), true, String::from("owned"), 0f64, String::from(""), String::from(""))) == (true)), "assertion 1");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("owned"), 0f64, String::from("actual error String"), String::from("actual error String"))) == (true)), "assertion 2");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("owned"), 0f64, String::from("actual error u8"), String::from("actual error u16"))) == (false)), "assertion 3");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::option::Option"), String::from("std::result::Result"), true, String::from("owned"), 0f64, String::from(""), String::from("actual error String"))) == (false)), "assertion 4");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::option::Option"), String::from("std::option::Option"), false, String::from("owned"), 0f64, String::from(""), String::from(""))) == (false)), "assertion 5");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::option::Option"), String::from("std::option::Option"), true, String::from("copy"), 0f64, String::from(""), String::from(""))) == (false)), "assertion 6");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::option::Option"), String::from("std::option::Option"), true, String::from("owned"), 1f64, String::from(""), String::from(""))) == (false)), "assertion 7");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::option::Option"), String::from("std::option::Option"), true, String::from(""), 0f64, String::from(""), String::from(""))) == (false)), "assertion 8");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("owned"), 0f64, String::from(""), String::from(""))) == (false)), "assertion 9");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("local::Option"), String::from("local::Option"), true, String::from("owned"), 0f64, String::from(""), String::from(""))) == (false)), "assertion 10");
    assert!(((self::accept_rust_standard_sum_question_family(String::from("std::option::Option"), String::from("std::option::Option"), String::from(""), String::from(""))) == (true)), "assertion 11");
    assert!(((self::accept_rust_standard_sum_question_family(String::from("std::result::Result"), String::from("std::result::Result"), String::from("a"), String::from("a"))) == (true)), "assertion 12");
    assert!(((self::accept_rust_standard_sum_question_family(String::from("std::result::Result"), String::from("std::result::Result"), String::from("a"), String::from("b"))) == (false)), "assertion 13");
    assert!(((self::accept_rust_standard_sum_question_context(String::from("std::option::Option"), String::from("std::option::Option"), true, false, String::from(""), String::from(""))) == (false)), "assertion 14");
    assert!(((self::accept_rust_standard_sum_question_context(String::from("std::option::Option"), String::from("std::option::Option"), true, true, String::from(""), String::from(""))) == (true)), "assertion 15");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u8"), String::from("u16"), 90f64)) == (vec![String::from("copy"), String::from("u16"), String::from("integer exact")])), "assertion 16");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("owned"), 0f64, false, true, true, String::from("u8"), String::from("u16"), 99f64)) == (vec![String::from("owned"), String::from("u16"), String::from("integer exact")])), "assertion 17");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("bool"), String::from("u16"), 90f64)) == (vec![String::from("copy"), String::from("u16"), String::from("boolean integer")])), "assertion 18");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u16"), String::from("f64"), 90f64)) == (vec![String::from("copy"), String::from("f64"), String::from("floating exact")])), "assertion 19");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u16"), String::from("u8"), 90f64)) == (Vec::<String>::new())), "assertion 20");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("usize"), String::from("u64"), 90f64)) == (Vec::<String>::new())), "assertion 21");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u64"), String::from("f64"), 90f64)) == (Vec::<String>::new())), "assertion 22");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, false, true, String::from("u8"), String::from("u16"), 90f64)) == (Vec::<String>::new())), "assertion 23");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, false, String::from("u8"), String::from("u16"), 90f64)) == (Vec::<String>::new())), "assertion 24");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, false, true, true, String::from("u8"), String::from("u16"), 90f64)) == (Vec::<String>::new())), "assertion 25");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("owned"), 1f64, false, true, true, String::from("u8"), String::from("u16"), 90f64)) == (Vec::<String>::new())), "assertion 26");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("borrowed"), 0f64, false, true, true, String::from("u8"), String::from("u16"), 90f64)) == (Vec::<String>::new())), "assertion 27");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::option::Option"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u8"), String::from("u16"), 90f64)) == (Vec::<String>::new())), "assertion 28");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("local::Result"), String::from("local::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u8"), String::from("u16"), 90f64)) == (Vec::<String>::new())), "assertion 29");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), false, String::from("copy"), 0f64, true, true, true, String::from("u8"), String::from("u16"), 90f64)) == (Vec::<String>::new())), "assertion 30");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u8"), String::from("u16"), 89f64)) == (Vec::<String>::new())), "assertion 31");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u8"), String::from("u16"), 100f64)) == (Vec::<String>::new())), "assertion 32");
    assert!(((self::read_rust_standard_sum_primitive_question_conversion_plan(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("copy"), 0f64, true, true, true, String::from("u8"), String::from("u8"), 90f64)) == (Vec::<String>::new())), "assertion 33");
    assert!(((self::accept_rust_standard_sum_primitive_question_completion(String::from("copy"), false)) == (true)), "assertion 34");
    assert!(((self::accept_rust_standard_sum_primitive_question_completion(String::from("owned"), true)) == (true)), "assertion 35");
    assert!(((self::accept_rust_standard_sum_primitive_question_completion(String::from("owned"), false)) == (false)), "assertion 36");
    assert!(((self::accept_rust_standard_sum_primitive_question_completion(String::from(""), true)) == (false)), "assertion 37");
    assert!(((self::accept_rust_standard_sum_question_context(String::from("std::result::Result"), String::from("std::result::Result"), true, true, String::from("u8 actual identity"), String::from("u16 actual identity"))) == (false)), "assertion 38");
    assert!(((self::accept_rust_standard_sum_owned_question_context(String::from("std::result::Result"), String::from("std::result::Result"), true, String::from("owned"), 0f64, String::from("u8 actual identity"), String::from("u16 actual identity"))) == (false)), "assertion 39");
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
fn all_literal_owned_sum_propagation_controls() { main(); }
