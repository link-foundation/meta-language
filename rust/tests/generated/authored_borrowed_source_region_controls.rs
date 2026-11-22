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

pub fn accept_rust_owned_binding_tracking(collecting: bool, excluded: bool) -> bool {
    (!collecting && !excluded)
}

pub fn accept_rust_owned_binding_value(kind: String, storage: String, layers: f64, known: bool) -> bool {
    (((((kind == "array") && (storage == "owned vector")) || ((kind == "string") && (storage == "owned string"))) && (layers == (0f64))) && known)
}

pub fn accept_rust_owned_binding_registration(owned: bool, borrowed_place: bool) -> bool {
    (owned && !borrowed_place)
}

pub fn read_rust_owned_binding_read_error(state: String) -> String {
    if (state == "available") {
        String::from("")
    } else {
        if (state == "moved") {
            String::from("a moved source binding cannot be read or consumed again")
        } else {
            if (state == "dropped") {
                String::from("an owned source binding does not outlive this use")
            } else {
                if (state == "possibly moved") {
                    String::from("a source binding moved on a returning branch cannot be reused")
                } else {
                    String::from("a source binding needs its actual ownership state")
                }
            }
        }
    }
}

pub fn consume_rust_owned_binding_state(state: String) -> String {
    if (state == "available") {
        String::from("moved")
    } else {
        state.clone()
    }
}

pub fn merge_rust_owned_binding_states(left: String, right: String, left_returns: bool, right_returns: bool) -> String {
    if (!left_returns && !right_returns) {
        String::from("unreachable")
    } else {
        if (!left_returns || (left == "absent")) {
            right.clone()
        } else {
            if (!right_returns || (right == "absent")) {
                left.clone()
            } else {
                if (left == right) {
                    left.clone()
                } else {
                    String::from("possibly moved")
                }
            }
        }
    }
}

pub fn accept_rust_owned_binding_guard(before: Vec<String>, after: Vec<String>) -> bool {
    if (((before.len() as f64)) != ((after.len() as f64))) {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s1 = self::source_ml_temporary_accept_rust_owned_binding_guard_loop1(before, after, index);
                match source_ml_source1_s1.clone() {
                    self::MlTemporaryAcceptRustOwnedBindingGuardLoop1Result::MlTemporaryAcceptRustOwnedBindingGuardLoop1Done => {
                        true
                    }
                    self::MlTemporaryAcceptRustOwnedBindingGuardLoop1Result::MlTemporaryAcceptRustOwnedBindingGuardLoop1Return(source_ml_temporary_result1) => {
                        source_ml_temporary_result1
                    }
                }
            }
        }
    }
}

pub fn expire_rust_owned_binding_state(state: String) -> String {
    if ((state == "available") || (state == "possibly moved")) {
        String::from("dropped")
    } else {
        state.clone()
    }
}

pub fn accept_rust_owned_result_transfer(binding: bool, available: bool, transferred: bool, fresh: bool) -> bool {
    if binding {
        available
    } else {
        (transferred || fresh)
    }
}

pub fn read_rust_owned_borrow_join_error(tracked: bool, borrowed: bool, named: bool, same: bool) -> String {
    if (((tracked && borrowed) && named) && !same) {
        String::from("alternative borrowed owners need their complete loan union proof")
    } else {
        String::from("")
    }
}

pub fn merge_rust_owned_loan_bindings(left: Vec<String>, right: Vec<String>) -> Vec<String> {
    {
        let result = self::ml_array::slice(&left, 0f64, 0f64);
        {
            let source_ml_temporary_values2 = left;
            {
                let source_ml_temporary_index2 = self::ml::Big::from_i128(0);
                {
                    let result_2 = self::source_ml_temporary_merge_rust_owned_loan_bindings_loop4(result, source_ml_temporary_values2, source_ml_temporary_index2.clone());
                    {
                        let source_ml_temporary_values3 = right;
                        {
                            let source_ml_temporary_index3 = self::ml::Big::from_i128(0);
                            {
                                let result_3 = self::source_ml_temporary_merge_rust_owned_loan_bindings_loop5(result_2, source_ml_temporary_values3, source_ml_temporary_index3.clone());
                                result_3
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn project_rust_owned_loan_bindings(identity: String, loans: Vec<String>) -> Vec<String> {
    self::merge_rust_owned_loan_bindings(if (identity == "") {
        Vec::<String>::new()
    } else {
        vec![identity.clone()]
    }, loans)
}

pub fn read_rust_owned_loan_read_error(states: Vec<String>) -> String {
    if self::ml_array::includes(&states, &String::from("dropped"), 0f64, false, |a, b| a == b) {
        self::read_rust_owned_binding_read_error(String::from("dropped"))
    } else {
        if self::ml_array::includes(&states, &String::from("moved"), 0f64, false, |a, b| a == b) {
            self::read_rust_owned_binding_read_error(String::from("moved"))
        } else {
            if self::ml_array::includes(&states, &String::from("possibly moved"), 0f64, false, |a, b| a == b) {
                self::read_rust_owned_binding_read_error(String::from("possibly moved"))
            } else {
                {
                    let source_ml_temporary_values6 = states;
                    {
                        let source_ml_temporary_index6 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source1_s2 = self::source_ml_temporary_read_rust_owned_loan_read_error_loop7(source_ml_temporary_values6, source_ml_temporary_index6.clone());
                            match source_ml_source1_s2.clone() {
                                self::MlTemporaryReadRustOwnedLoanReadErrorLoop7Result::MlTemporaryReadRustOwnedLoanReadErrorLoop7Done => {
                                    String::from("")
                                }
                                self::MlTemporaryReadRustOwnedLoanReadErrorLoop7Result::MlTemporaryReadRustOwnedLoanReadErrorLoop7Return(source_ml_temporary_result7) => {
                                    source_ml_temporary_result7.clone()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_owned_string_construction_error(kind: String, storage: String, layers: f64, known: bool, references: bool) -> String {
    if ((known && (kind == "string")) && (((storage == "owned string") && (layers == (0f64))) || ((((storage == "owned string") || (storage == "string slice")) && (layers == (1f64))) && references))) {
        String::from("")
    } else {
        String::from("standard String construction needs an actual owned String or one immutable String/str reference")
    }
}

pub fn accept_rust_owned_macro_argument_transfer(macro_: String) -> bool {
    (macro_ == "vec")
}

pub fn accept_rust_string_slice_clone_reference(method: String, kind: String, storage: String, layers: f64) -> bool {
    ((((method == "clone") && (kind == "string")) && (storage == "string slice")) && (layers == (1f64)))
}

pub fn read_rust_owned_temporary_disposition(framed: bool, extended: bool, escaping: bool) -> String {
    if (framed || escaping) {
        String::from("leave")
    } else {
        if extended {
            String::from("extend")
        } else {
            String::from("expire")
        }
    }
}

pub fn read_rust_owned_place_move_error(tracked: bool, borrowed: bool, owned: bool) -> String {
    if ((tracked && borrowed) && owned) {
        String::from("an owned source value cannot move out of a borrowed place")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_source_value_role(kind: String, bits: String, signed: bool, owner: String, path: String, arguments_count: f64, wrappers: Vec<String>, borrowed: bool) -> String {
    if {
        let source_ml_source1_comparison_operand3 = self::ml_array::optional_at(&wrappers, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand4 = String::from("reference");
            match (source_ml_source1_comparison_operand3.clone(), source_ml_source1_comparison_operand4.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    } {
        String::from("reference")
    } else {
        if {
            let source_ml_source1_comparison_operand5 = self::ml_array::optional_at(&wrappers, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand6 = String::from("box");
                match (source_ml_source1_comparison_operand5.clone(), source_ml_source1_comparison_operand6.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        } {
            String::from("box")
        } else {
            if ((((kind == "array") && (owner == "library")) && (path == "std::vec::Vec")) && (arguments_count == (1f64))) {
                if borrowed {
                    String::from("borrowed vector")
                } else {
                    String::from("owned")
                }
            } else {
                if ((kind == "data") && ((owner == "declaration") || (owner == "library"))) {
                    String::from("owned")
                } else {
                    if ((((kind == "string") && (owner == "library")) && self::ml_array::includes(&vec![String::from("std::string::String"), String::from("alloc::string::String")], &path.clone(), 0f64, false, |a, b| a == b)) && (arguments_count == (0f64))) {
                        String::from("string")
                    } else {
                        if (((kind == "string") && (owner == "primitive")) && (path == "str")) {
                            String::from("text")
                        } else {
                            if ((((kind == "unit") && (owner == "unit")) && (path == "")) && (arguments_count == (0f64))) {
                                String::from("copy")
                            } else {
                                if (((owner == "primitive") && (arguments_count == (0f64))) && self::accept_rust_source_signature_primitive(kind.clone(), path.clone(), bits.clone(), signed)) {
                                    String::from("copy")
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

pub fn read_rust_borrowed_source_use_plan(role: String, mode: String) -> String {
    if !(self::ml_array::includes(&vec![String::from("copy"), String::from("inferred numeric"), String::from("reference"), String::from("owned"), String::from("string"), String::from("box"), String::from("borrowed vector")], &role.clone(), 0f64, false, |a, b| a == b)) {
        String::from("unsupported")
    } else {
        if ((mode == "move") && self::ml_array::includes(&vec![String::from("owned"), String::from("string"), String::from("box"), String::from("borrowed vector")], &role.clone(), 0f64, false, |a, b| a == b)) {
            String::from("consume")
        } else {
            String::from("read")
        }
    }
}

pub fn read_rust_borrowed_source_read_error(state: String, loans: Vec<String>) -> String {
    {
        let source_ml_source1_s7 = if (state != "") {
            {
                let error = self::read_rust_owned_binding_read_error(state.clone());
                if !error.is_empty() {
                    self::MlTemporaryReadRustBorrowedSourceReadErrorJoin9::MlTemporaryReadRustBorrowedSourceReadErrorJoin9Return(error.clone())
                } else {
                    self::MlTemporaryReadRustBorrowedSourceReadErrorJoin9::MlTemporaryReadRustBorrowedSourceReadErrorJoin9Next
                }
            }
        } else {
            self::MlTemporaryReadRustBorrowedSourceReadErrorJoin9::MlTemporaryReadRustBorrowedSourceReadErrorJoin9Next
        };
        match source_ml_source1_s7.clone() {
            self::MlTemporaryReadRustBorrowedSourceReadErrorJoin9::MlTemporaryReadRustBorrowedSourceReadErrorJoin9Next => {
                {
                    let source_ml_temporary_values8 = loans;
                    {
                        let source_ml_temporary_index8 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source1_s8 = self::source_ml_temporary_read_rust_borrowed_source_read_error_loop10(source_ml_temporary_values8, source_ml_temporary_index8.clone());
                            match source_ml_source1_s8.clone() {
                                self::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Done => {
                                    String::from("")
                                }
                                self::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Return(source_ml_temporary_result10) => {
                                    source_ml_temporary_result10.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::MlTemporaryReadRustBorrowedSourceReadErrorJoin9::MlTemporaryReadRustBorrowedSourceReadErrorJoin9Return(source_ml_temporary_result10_2) => {
                source_ml_temporary_result10_2.clone()
            }
        }
    }
}

pub fn consume_rust_borrowed_source_region(state: String) -> String {
    self::consume_rust_owned_binding_state(state.clone())
}

pub fn expire_rust_borrowed_source_region(state: String) -> String {
    self::expire_rust_owned_binding_state(state.clone())
}

pub fn read_rust_borrowed_source_adjustment(source: String, target: String, same: bool, inner: bool, source_inner: bool, place: bool, literal: bool, text: bool, source_references: f64, target_references: f64) -> String {
    if ((same && (source == target)) && (source_references == target_references)) {
        String::from("preserve")
    } else {
        if ((source == "inferred numeric") && (target == "copy")) {
            String::from("constrain primitive")
        } else {
            if ((target == "reference") && (same || inner)) {
                if ((source == "reference") && (target_references <= source_references)) {
                    String::from("project reference")
                } else {
                    if place {
                        String::from("borrow")
                    } else {
                        if literal {
                            String::from("static literal")
                        } else {
                            String::from("temporary borrow")
                        }
                    }
                }
            } else {
                if (((source == "reference") && (target == "copy")) && same) {
                    String::from("copy referent")
                } else {
                    if ((target == "box") && inner) {
                        String::from("construct box")
                    } else {
                        if ((source == "box") && source_inner) {
                            if (target == "copy") {
                                String::from("copy box payload")
                            } else {
                                String::from("move box payload")
                            }
                        } else {
                            if (((source == "reference") && (target == "string")) && text) {
                                String::from("materialize text")
                            } else {
                                String::from("unsupported")
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_borrowed_source_position_flow_error(role: String, same: bool, static_demand: bool, static_proof: bool) -> String {
    if !same {
        String::from("source expression qualifiers or owners disagree with their actual value flow")
    } else {
        if (static_demand && !static_proof) {
            String::from("a static source lifetime cannot be supplied by a local binder region")
        } else {
            if !(self::ml_array::includes(&vec![String::from("copy"), String::from("reference"), String::from("owned"), String::from("string"), String::from("box"), String::from("borrowed vector")], &role.clone(), 0f64, false, |a, b| a == b)) {
                String::from("the source expression needs a separate value-domain proof")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_rust_borrowed_source_return_error(regions: Vec<f64>, allowed: Vec<f64>, static_regions: Vec<f64>, interface_known: bool) -> String {
    if !interface_known {
        String::from("a borrowed source result needs its actual source parameter lifetime interface")
    } else {
        {
            let source_ml_temporary_values11 = regions;
            {
                let source_ml_temporary_index11 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s9 = self::source_ml_temporary_read_rust_borrowed_source_return_error_loop12(allowed, static_regions, source_ml_temporary_values11, source_ml_temporary_index11.clone());
                    match source_ml_source1_s9.clone() {
                        self::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Done => {
                            String::from("")
                        }
                        self::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Return(source_ml_temporary_result12) => {
                            source_ml_temporary_result12.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_borrowed_source_lifetime_error(demand: String, sources: Vec<String>) -> String {
    if ((demand == "") || (demand == "_")) {
        String::from("")
    } else {
        {
            let source_ml_temporary_values13 = sources;
            {
                let source_ml_temporary_index13 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s10 = self::source_ml_temporary_read_rust_borrowed_source_lifetime_error_loop14(demand.clone(), source_ml_temporary_values13, source_ml_temporary_index13.clone());
                    match source_ml_source1_s10.clone() {
                        self::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Done => {
                            String::from("")
                        }
                        self::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Return(source_ml_temporary_result14) => {
                            source_ml_temporary_result14.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_borrowed_source_target_field_error(kind: String, wrappers: Vec<String>, child: bool) -> String {
    if ((self::ml_array::includes(&wrappers, &String::from("reference"), 0f64, false, |a, b| a == b) || child) || self::ml_array::includes(&vec![String::from("function"), String::from("tuple"), String::from("record"), String::from("map"), String::from("set"), String::from("cell")], &kind.clone(), 0f64, false, |a, b| a == b)) {
        String::from("a borrowed referent needs a closed target-copy field graph with represented source qualifiers")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_source_operation(kind: String) -> String {
    if (kind == "toString") {
        String::from("show")
    } else {
        if self::ml_array::includes(&vec![String::from("lit"), String::from("unit"), String::from("var"), String::from("let"), String::from("if"), String::from("array"), String::from("call"), String::from("ctor"), String::from("index"), String::from("length"), String::from("print"), String::from("cast"), String::from("unary"), String::from("binary"), String::from("math"), String::from("show"), String::from("concat")], &kind.clone(), 0f64, false, |a, b| a == b) {
            kind.clone()
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_borrowed_source_region_budget_error(visits: f64, depth: f64, roots: f64, loans: f64) -> String {
    {
        let source_ml_temporary_values15 = vec![visits, depth, roots, loans];
        {
            let source_ml_temporary_index15 = self::ml::Big::from_i128(0);
            {
                let source_ml_source1_s11 = self::source_ml_temporary_read_rust_borrowed_source_region_budget_error_loop16(source_ml_temporary_values15, source_ml_temporary_index15.clone());
                match source_ml_source1_s11.clone() {
                    self::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Done => {
                        if ((((visits > (100000f64)) || (depth > (256f64))) || (roots > (100000f64))) || (loans > (1048576f64))) {
                            String::from("the immutable source binder and region proof exceeds its finite resource profile")
                        } else {
                            String::from("")
                        }
                    }
                    self::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Return(source_ml_temporary_result16) => {
                        source_ml_temporary_result16.clone()
                    }
                }
            }
        }
    }
}

pub fn reserve_rust_borrowed_source_position(nodes: f64, units: f64, add_nodes: f64, add_units: f64) -> Vec<f64> {
    {
        let source_ml_temporary_values17 = vec![nodes, units, add_nodes, add_units];
        {
            let source_ml_temporary_index17 = self::ml::Big::from_i128(0);
            {
                let source_ml_source1_s12 = self::source_ml_temporary_reserve_rust_borrowed_source_position_loop18(source_ml_temporary_values17, source_ml_temporary_index17.clone());
                match source_ml_source1_s12.clone() {
                    self::MlTemporaryReserveRustBorrowedSourcePositionLoop18Result::MlTemporaryReserveRustBorrowedSourcePositionLoop18Done => {
                        {
                            let next_nodes = (nodes + (4f64 * add_nodes));
                            {
                                let next_units = (units + (4f64 * add_units));
                                if ((next_nodes > (1048576f64)) || (next_units > (8388608f64))) {
                                    Vec::<f64>::new()
                                } else {
                                    vec![next_nodes, next_units]
                                }
                            }
                        }
                    }
                    self::MlTemporaryReserveRustBorrowedSourcePositionLoop18Result::MlTemporaryReserveRustBorrowedSourcePositionLoop18Return(source_ml_temporary_result18) => {
                        source_ml_temporary_result18
                    }
                }
            }
        }
    }
}

pub fn read_rust_borrowed_source_runtime_error(selected: bool, runtime_shell: bool) -> String {
    if (selected && runtime_shell) {
        String::from("borrowed source body positions require their independent runtime-shell binder adapter")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_source_observation_error(role: String, operation: String) -> String {
    if self::ml_array::includes(&vec![String::from("source state packet"), String::from("source loop packet"), String::from("source loop control")], &role.clone(), 0f64, false, |a, b| a == b) {
        String::from("source state packets are interpreted through their actual result/state slots")
    } else {
        if (self::ml_array::includes(&vec![String::from("reference"), String::from("box"), String::from("owned"), String::from("borrowed vector")], &role.clone(), 0f64, false, |a, b| a == b) && (operation != "len")) {
            String::from("this source observation needs an independently represented value or source trait proof")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_borrowed_source_observation_role(role: String, kind: String, owner: String, path: String, argument_count: f64, wrappers: Vec<String>) -> String {
    if ((((((role == "reference") && (kind == "string")) && (((owner == "primitive") && (path == "str")) || ((owner == "library") && self::ml_array::includes(&vec![String::from("std::string::String"), String::from("alloc::string::String")], &path.clone(), 0f64, false, |a, b| a == b)))) && (argument_count == (0f64))) && (((wrappers.len() as f64)) == (1f64))) && {
        let source_ml_source1_comparison_operand13 = self::ml_array::optional_at(&wrappers, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand14 = String::from("reference");
            match (source_ml_source1_comparison_operand13.clone(), source_ml_source1_comparison_operand14.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) {
        String::from("text")
    } else {
        role.clone()
    }
}

pub fn read_rust_borrowed_source_match_error(role: String, kind: String, pattern: String, fields: f64, bindings: f64) -> String {
    if ((role != "reference") || (kind != "data")) {
        String::from("source matching requires its actual borrowed nominal place adapter")
    } else {
        if !(self::ml_array::includes(&vec![String::from("wild"), String::from("bind"), String::from("ctor")], &pattern.clone(), 0f64, false, |a, b| a == b)) {
            String::from("this source pattern needs its independent binding projection")
        } else {
            if ((pattern == "ctor") && ((!(self::ml_math::is_safe_integer(fields)) || (fields < (0f64))) || (fields != bindings))) {
                String::from("source constructor patterns need every actual declared field position")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_rust_borrowed_source_owned_match_error(composed: bool, kind: String, pattern: String, fields: f64, bindings: f64) -> String {
    if !composed {
        String::from("owned matches require the explicit closed-call region composition")
    } else {
        self::read_rust_borrowed_source_match_error(String::from("reference"), kind.clone(), pattern.clone(), fields, bindings)
    }
}

pub fn reserve_rust_borrowed_source_branches(cells: f64, addition: f64) -> f64 {
    if ((((((!(self::ml_math::is_safe_integer(cells)) || !(self::ml_math::is_safe_integer(addition))) || (cells < (0f64))) || (addition < (0f64))) || {
        let source_ml_source1_same_value_operand15 = cells;
        {
            let source_ml_source1_same_value_operand16 = (-0f64);
            (((source_ml_source1_same_value_operand15 == source_ml_source1_same_value_operand16) && ((source_ml_source1_same_value_operand15 != (0f64)) || (((1f64 / source_ml_source1_same_value_operand15)) == ((1f64 / source_ml_source1_same_value_operand16))))) || ((source_ml_source1_same_value_operand15 != source_ml_source1_same_value_operand15) && (source_ml_source1_same_value_operand16 != source_ml_source1_same_value_operand16)))
        }
    }) || {
        let source_ml_source1_same_value_operand17 = addition;
        {
            let source_ml_source1_same_value_operand18 = (-0f64);
            (((source_ml_source1_same_value_operand17 == source_ml_source1_same_value_operand18) && ((source_ml_source1_same_value_operand17 != (0f64)) || (((1f64 / source_ml_source1_same_value_operand17)) == ((1f64 / source_ml_source1_same_value_operand18))))) || ((source_ml_source1_same_value_operand17 != source_ml_source1_same_value_operand17) && (source_ml_source1_same_value_operand18 != source_ml_source1_same_value_operand18)))
        }
    }) || (addition > ((1048576f64 - cells)))) {
        (-1f64)
    } else {
        (cells + addition)
    }
}

pub fn merge_rust_borrowed_source_region(left: String, right: String) -> String {
    self::merge_rust_owned_binding_states(left.clone(), right.clone(), true, true)
}

pub fn read_rust_borrowed_source_match_operation(kind: String) -> String {
    if (kind == "match") {
        String::from("match")
    } else {
        String::from("")
    }
}

pub fn accept_rust_borrowed_source_box_reference(source: Vec<String>, target: Vec<String>, canonical: bool, same_leaf: bool) -> bool {
    if ((((!canonical || !same_leaf) || {
        let source_ml_source1_comparison_operand19 = self::ml_array::optional_at(&source, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand20 = String::from("reference");
            !(match (source_ml_source1_comparison_operand19.clone(), source_ml_source1_comparison_operand20.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    }) || {
        let source_ml_source1_comparison_operand21 = self::ml_array::optional_at(&target, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand22 = String::from("reference");
            !(match (source_ml_source1_comparison_operand21.clone(), source_ml_source1_comparison_operand22.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    }) || (((source.len() as f64)) <= ((target.len() as f64)))) {
        false
    } else {
        {
            let index = 1f64;
            {
                let source_ml_source1_s23 = self::source_ml_temporary_accept_rust_borrowed_source_box_reference_loop19(source, index);
                match source_ml_source1_s23.clone() {
                    self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Done => {
                        {
                            let index_2 = 1f64;
                            {
                                let source_ml_source1_s24 = self::source_ml_temporary_accept_rust_borrowed_source_box_reference_loop20(target, index_2);
                                match source_ml_source1_s24.clone() {
                                    self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Done => {
                                        true
                                    }
                                    self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Return(source_ml_temporary_result20) => {
                                        source_ml_temporary_result20
                                    }
                                }
                            }
                        }
                    }
                    self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Return(source_ml_temporary_result20_2) => {
                        source_ml_temporary_result20_2
                    }
                }
            }
        }
    }
}

pub fn read_rust_borrowed_source_iterator_operation(kind: String, operation: String) -> String {
    if (kind == "append") {
        String::from("iterator concat")
    } else {
        if ((kind == "method") && self::ml_array::includes(&vec![String::from("map"), String::from("slice")], &operation.clone(), 0f64, false, |a, b| a == b)) {
            format!("{}{}", (String::from("iterator ")), operation)
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_borrowed_source_storage_write_error(state: String) -> String {
    if self::ml_array::includes(&vec![String::from("available"), String::from("moved"), String::from("possibly moved")], &state.clone(), 0f64, false, |a, b| a == b) {
        String::from("")
    } else {
        String::from("a source whole-root write requires its initialized lexical storage scope")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustOwnedBindingGuardLoop1Result {
    MlTemporaryAcceptRustOwnedBindingGuardLoop1Done,
    MlTemporaryAcceptRustOwnedBindingGuardLoop1Return(bool),
}

pub fn source_ml_temporary_accept_rust_owned_binding_guard_loop1(mut before: Vec<String>, mut after: Vec<String>, mut index: f64) -> self::MlTemporaryAcceptRustOwnedBindingGuardLoop1Result {
    loop {
        return if (index < ((before.len() as f64))) {
            if {
                let source_ml_source1_comparison_operand25 = self::ml_array::optional_at(&before, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand26 = self::ml_array::optional_at(&after, self::ml_array::number_index(index));
                    !(match (source_ml_source1_comparison_operand25.clone(), source_ml_source1_comparison_operand26.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                }
            } {
                self::MlTemporaryAcceptRustOwnedBindingGuardLoop1Result::MlTemporaryAcceptRustOwnedBindingGuardLoop1Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRustOwnedBindingGuardLoop1Result::MlTemporaryAcceptRustOwnedBindingGuardLoop1Done
        };
    }
}

pub fn source_ml_temporary_merge_rust_owned_loan_bindings_loop4(mut result: Vec<String>, mut source_ml_temporary_values2: Vec<String>, mut source_ml_temporary_index2: self::ml::Big) -> Vec<String> {
    loop {
        return if (source_ml_temporary_index2 < (self::ml::Big::from_u128(source_ml_temporary_values2.len() as u128))) {
            {
                let identity = self::ml_array::at(&source_ml_temporary_values2, source_ml_temporary_index2.to_index());
                if !(self::ml_array::includes(&result, &identity.clone(), 0f64, false, |a, b| a == b)) {
                    {
                        let result_2 = { let (ml_append_right, ml_append_left) = (vec![identity.clone()], result); self::ml_array::append(ml_append_left, ml_append_right) };
                        {
                            let source_ml_temporary_index2_2 = source_ml_temporary_index2.add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_temporary_index2) = (result_2, source_ml_temporary_index2_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_temporary_index2_3 = source_ml_temporary_index2.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index2 = source_ml_temporary_index2_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

pub fn source_ml_temporary_merge_rust_owned_loan_bindings_loop5(mut result: Vec<String>, mut source_ml_temporary_values3: Vec<String>, mut source_ml_temporary_index3: self::ml::Big) -> Vec<String> {
    loop {
        return if (source_ml_temporary_index3 < (self::ml::Big::from_u128(source_ml_temporary_values3.len() as u128))) {
            {
                let identity = self::ml_array::at(&source_ml_temporary_values3, source_ml_temporary_index3.to_index());
                if !(self::ml_array::includes(&result, &identity.clone(), 0f64, false, |a, b| a == b)) {
                    {
                        let result_2 = { let (ml_append_right, ml_append_left) = (vec![identity.clone()], result); self::ml_array::append(ml_append_left, ml_append_right) };
                        {
                            let source_ml_temporary_index3_2 = source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_temporary_index3) = (result_2, source_ml_temporary_index3_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_temporary_index3_3 = source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index3 = source_ml_temporary_index3_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedLoanReadErrorLoop7Result {
    MlTemporaryReadRustOwnedLoanReadErrorLoop7Done,
    MlTemporaryReadRustOwnedLoanReadErrorLoop7Return(String),
}

pub fn source_ml_temporary_read_rust_owned_loan_read_error_loop7(mut source_ml_temporary_values6: Vec<String>, mut source_ml_temporary_index6: self::ml::Big) -> self::MlTemporaryReadRustOwnedLoanReadErrorLoop7Result {
    loop {
        return if (source_ml_temporary_index6 < (self::ml::Big::from_u128(source_ml_temporary_values6.len() as u128))) {
            {
                let state = self::ml_array::at(&source_ml_temporary_values6, source_ml_temporary_index6.to_index());
                if (state != "available") {
                    self::MlTemporaryReadRustOwnedLoanReadErrorLoop7Result::MlTemporaryReadRustOwnedLoanReadErrorLoop7Return(self::read_rust_owned_binding_read_error(String::from("")))
                } else {
                    {
                        let source_ml_temporary_index6_2 = source_ml_temporary_index6.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index6 = source_ml_temporary_index6_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedLoanReadErrorLoop7Result::MlTemporaryReadRustOwnedLoanReadErrorLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result {
    MlTemporaryReadRustBorrowedSourceReadErrorLoop10Done,
    MlTemporaryReadRustBorrowedSourceReadErrorLoop10Return(String),
}

pub fn source_ml_temporary_read_rust_borrowed_source_read_error_loop10(mut source_ml_temporary_values8: Vec<String>, mut source_ml_temporary_index8: self::ml::Big) -> self::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result {
    loop {
        return if (source_ml_temporary_index8 < (self::ml::Big::from_u128(source_ml_temporary_values8.len() as u128))) {
            {
                let loan = self::ml_array::at(&source_ml_temporary_values8, source_ml_temporary_index8.to_index());
                {
                    let error = self::read_rust_owned_binding_read_error(loan.clone());
                    if !error.is_empty() {
                        self::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Return(String::from("a source expression uses a region whose actual binder has moved or expired"))
                    } else {
                        {
                            let source_ml_temporary_index8_2 = source_ml_temporary_index8.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index8 = source_ml_temporary_index8_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustBorrowedSourceReadErrorJoin9 {
    MlTemporaryReadRustBorrowedSourceReadErrorJoin9Next,
    MlTemporaryReadRustBorrowedSourceReadErrorJoin9Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result {
    MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Done,
    MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Return(String),
}

pub fn source_ml_temporary_read_rust_borrowed_source_return_error_loop12(mut allowed: Vec<f64>, mut static_regions: Vec<f64>, mut source_ml_temporary_values11: Vec<f64>, mut source_ml_temporary_index11: self::ml::Big) -> self::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result {
    loop {
        return if (source_ml_temporary_index11 < (self::ml::Big::from_u128(source_ml_temporary_values11.len() as u128))) {
            {
                let region = self::ml_array::at(&source_ml_temporary_values11, source_ml_temporary_index11.to_index());
                if (!(self::ml_array::includes(&allowed, &region, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) && !(self::ml_array::includes(&static_regions, &region, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) {
                    self::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Return(String::from("a returned source region is not derived from its actual enclosing parameter interface"))
                } else {
                    {
                        let source_ml_temporary_index11_2 = source_ml_temporary_index11.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index11 = source_ml_temporary_index11_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result {
    MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Done,
    MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Return(String),
}

pub fn source_ml_temporary_read_rust_borrowed_source_lifetime_error_loop14(mut demand: String, mut source_ml_temporary_values13: Vec<String>, mut source_ml_temporary_index13: self::ml::Big) -> self::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result {
    loop {
        return if (source_ml_temporary_index13 < (self::ml::Big::from_u128(source_ml_temporary_values13.len() as u128))) {
            {
                let source = self::ml_array::at(&source_ml_temporary_values13, source_ml_temporary_index13.to_index());
                if ((source != "static") && (source != demand)) {
                    self::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Return(String::from("an expression lifetime is not supplied by its actual source binder regions"))
                } else {
                    {
                        let source_ml_temporary_index13_2 = source_ml_temporary_index13.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index13 = source_ml_temporary_index13_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result {
    MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Done,
    MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Return(String),
}

pub fn source_ml_temporary_read_rust_borrowed_source_region_budget_error_loop16(mut source_ml_temporary_values15: Vec<f64>, mut source_ml_temporary_index15: self::ml::Big) -> self::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result {
    loop {
        return if (source_ml_temporary_index15 < (self::ml::Big::from_u128(source_ml_temporary_values15.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_temporary_values15, source_ml_temporary_index15.to_index());
                if ((!(self::ml_math::is_safe_integer(value)) || (value < (0f64))) || {
                    let source_ml_source1_same_value_operand27 = value;
                    {
                        let source_ml_source1_same_value_operand28 = (-0f64);
                        (((source_ml_source1_same_value_operand27 == source_ml_source1_same_value_operand28) && ((source_ml_source1_same_value_operand27 != (0f64)) || (((1f64 / source_ml_source1_same_value_operand27)) == ((1f64 / source_ml_source1_same_value_operand28))))) || ((source_ml_source1_same_value_operand27 != source_ml_source1_same_value_operand27) && (source_ml_source1_same_value_operand28 != source_ml_source1_same_value_operand28)))
                    }
                }) {
                    self::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Return(String::from("invalid source region proof resource count"))
                } else {
                    {
                        let source_ml_temporary_index15_2 = source_ml_temporary_index15.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index15 = source_ml_temporary_index15_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReserveRustBorrowedSourcePositionLoop18Result {
    MlTemporaryReserveRustBorrowedSourcePositionLoop18Done,
    MlTemporaryReserveRustBorrowedSourcePositionLoop18Return(Vec<f64>),
}

pub fn source_ml_temporary_reserve_rust_borrowed_source_position_loop18(mut source_ml_temporary_values17: Vec<f64>, mut source_ml_temporary_index17: self::ml::Big) -> self::MlTemporaryReserveRustBorrowedSourcePositionLoop18Result {
    loop {
        return if (source_ml_temporary_index17 < (self::ml::Big::from_u128(source_ml_temporary_values17.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_temporary_values17, source_ml_temporary_index17.to_index());
                if ((!(self::ml_math::is_safe_integer(value)) || (value < (0f64))) || {
                    let source_ml_source1_same_value_operand29 = value;
                    {
                        let source_ml_source1_same_value_operand30 = (-0f64);
                        (((source_ml_source1_same_value_operand29 == source_ml_source1_same_value_operand30) && ((source_ml_source1_same_value_operand29 != (0f64)) || (((1f64 / source_ml_source1_same_value_operand29)) == ((1f64 / source_ml_source1_same_value_operand30))))) || ((source_ml_source1_same_value_operand29 != source_ml_source1_same_value_operand29) && (source_ml_source1_same_value_operand30 != source_ml_source1_same_value_operand30)))
                    }
                }) {
                    self::MlTemporaryReserveRustBorrowedSourcePositionLoop18Result::MlTemporaryReserveRustBorrowedSourcePositionLoop18Return(Vec::<f64>::new())
                } else {
                    {
                        let source_ml_temporary_index17_2 = source_ml_temporary_index17.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index17 = source_ml_temporary_index17_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReserveRustBorrowedSourcePositionLoop18Result::MlTemporaryReserveRustBorrowedSourcePositionLoop18Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result {
    MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Done,
    MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Return(bool),
}

pub fn source_ml_temporary_accept_rust_borrowed_source_box_reference_loop20(mut target: Vec<String>, mut index: f64) -> self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result {
    loop {
        return if (index < ((target.len() as f64))) {
            if {
                let source_ml_source1_comparison_operand31 = self::ml_array::optional_at(&target, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand32 = String::from("box");
                    !(match (source_ml_source1_comparison_operand31.clone(), source_ml_source1_comparison_operand32.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            } {
                self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result {
    MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Done,
    MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Return(bool),
}

pub fn source_ml_temporary_accept_rust_borrowed_source_box_reference_loop19(mut source: Vec<String>, mut index: f64) -> self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result {
    loop {
        return if (index < ((source.len() as f64))) {
            if {
                let source_ml_source1_comparison_operand33 = self::ml_array::optional_at(&source, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand34 = String::from("box");
                    !(match (source_ml_source1_comparison_operand33.clone(), source_ml_source1_comparison_operand34.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            } {
                self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Done
        };
    }
}

fn ml_main() {
    assert!(((self::read_rust_borrowed_source_value_role(String::from("data"), String::from(""), false, String::from("declaration"), String::from("Node"), 0f64, vec![String::from("reference")], false)) == "reference"), "assertion 1");
    assert!(((self::read_rust_borrowed_source_value_role(String::from("array"), String::from(""), false, String::from("library"), String::from("std::vec::Vec"), 1f64, Vec::<String>::new(), true)) == "borrowed vector"), "assertion 2");
    assert!(((self::read_rust_borrowed_source_value_role(String::from("array"), String::from(""), false, String::from("declaration"), String::from("std::vec::Vec"), 1f64, Vec::<String>::new(), true)) == ""), "assertion 3");
    assert!(((self::read_rust_borrowed_source_use_plan(String::from("reference"), String::from("move"))) == "read"), "assertion 4");
    assert!(((self::read_rust_borrowed_source_use_plan(String::from("borrowed vector"), String::from("move"))) == "consume"), "assertion 5");
    assert!(((self::read_rust_borrowed_source_read_error(String::from("available"), vec![String::from("available")])) == ""), "assertion 6");
    assert!(((self::read_rust_borrowed_source_read_error(String::from("available"), vec![String::from("moved")])) == "a source expression uses a region whose actual binder has moved or expired"), "assertion 7");
    assert!(((self::read_rust_borrowed_source_adjustment(String::from("owned"), String::from("reference"), true, true, false, true, false, false, 0f64, 1f64)) == "borrow"), "assertion 8");
    assert!(((self::read_rust_borrowed_source_adjustment(String::from("reference"), String::from("reference"), true, false, false, true, false, false, 1f64, 1f64)) == "preserve"), "assertion 9");
    assert!(((self::read_rust_borrowed_source_return_error(vec![2f64], vec![2f64], vec![0f64], true)) == ""), "assertion 10");
    assert!(((self::read_rust_borrowed_source_return_error(vec![3f64], vec![2f64], vec![0f64], true)) == "a returned source region is not derived from its actual enclosing parameter interface"), "assertion 11");
    assert!(((self::read_rust_borrowed_source_return_error(vec![0f64], Vec::<f64>::new(), vec![0f64], true)) == ""), "assertion 12");
    assert!(((self::read_rust_borrowed_source_lifetime_error(String::from("a"), vec![String::from("a"), String::from("static")])) == ""), "assertion 13");
    assert!(((self::read_rust_borrowed_source_lifetime_error(String::from("static"), vec![String::from("a")])) == "an expression lifetime is not supplied by its actual source binder regions"), "assertion 14");
    assert!(((self::read_rust_borrowed_source_target_field_error(String::from("data"), vec![String::from("reference")], false)) == "a borrowed referent needs a closed target-copy field graph with represented source qualifiers"), "assertion 15");
    assert!(((self::read_rust_borrowed_source_operation(String::from("match"))) == ""), "assertion 16");
    assert!(((self::read_rust_borrowed_source_operation(String::from("if"))) == "if"), "assertion 17");
    assert!(((self::read_rust_borrowed_source_region_budget_error(100000f64, 256f64, 100000f64, 1048576f64)) == ""), "assertion 18");
    assert!(((self::read_rust_borrowed_source_region_budget_error(100001f64, 0f64, 1f64, 0f64)) == "the immutable source binder and region proof exceeds its finite resource profile"), "assertion 19");
    assert!(((self::read_rust_borrowed_source_region_budget_error(f64::NAN, 0f64, 1f64, 0f64)) == "invalid source region proof resource count"), "assertion 20");
    assert!(((self::read_rust_borrowed_source_region_budget_error(f64::INFINITY, 0f64, 1f64, 0f64)) == "invalid source region proof resource count"), "assertion 21");
    assert!(((self::read_rust_borrowed_source_region_budget_error((-0f64), 0f64, 1f64, 0f64)) == "invalid source region proof resource count"), "assertion 22");
    assert!(((self::read_rust_borrowed_source_region_budget_error((-1f64), 0f64, 1f64, 0f64)) == "invalid source region proof resource count"), "assertion 23");
    assert!(((self::read_rust_borrowed_source_region_budget_error(0.5f64, 0f64, 1f64, 0f64)) == "invalid source region proof resource count"), "assertion 24");
    assert!(((self::read_rust_borrowed_source_region_budget_error(9007199254740992f64, 0f64, 1f64, 0f64)) == "invalid source region proof resource count"), "assertion 25");
    assert!(((self::reserve_rust_borrowed_source_position(0f64, 0f64, 1f64, 2f64)) == (vec![4f64, 8f64])), "assertion 26");
    assert!(((self::reserve_rust_borrowed_source_position(1048572f64, 8388600f64, 1f64, 2f64)) == (vec![1048576f64, 8388608f64])), "assertion 27");
    assert!(((self::reserve_rust_borrowed_source_position(1048573f64, 0f64, 1f64, 0f64)) == (Vec::<f64>::new())), "assertion 28");
    assert!(((self::reserve_rust_borrowed_source_position(0f64, 8388601f64, 0f64, 2f64)) == (Vec::<f64>::new())), "assertion 29");
    assert!(((self::reserve_rust_borrowed_source_position(0f64, 0f64, (-0f64), 0f64)) == (Vec::<f64>::new())), "assertion 30");
    assert!(((self::read_rust_borrowed_source_runtime_error(true, true)) == "borrowed source body positions require their independent runtime-shell binder adapter"), "assertion 31");
    assert!(((self::read_rust_borrowed_source_runtime_error(false, true)) == ""), "assertion 32");
    assert!(((self::read_rust_borrowed_source_runtime_error(true, false)) == ""), "assertion 33");
    assert!(((self::read_rust_borrowed_source_operation(String::from("toString"))) == "show"), "assertion 34");
    assert!(((self::read_rust_borrowed_source_observation_error(String::from("reference"), String::from("show"))) == "this source observation needs an independently represented value or source trait proof"), "assertion 35");
    assert!(((self::read_rust_borrowed_source_observation_error(String::from("owned"), String::from("display"))) == "this source observation needs an independently represented value or source trait proof"), "assertion 36");
    assert!(((self::read_rust_borrowed_source_observation_error(String::from("copy"), String::from("binary"))) == ""), "assertion 37");
    assert!(((self::read_rust_borrowed_source_observation_error(String::from("borrowed vector"), String::from("len"))) == ""), "assertion 38");
    assert!(((self::read_rust_borrowed_source_observation_role(String::from("reference"), String::from("string"), String::from("primitive"), String::from("str"), 0f64, vec![String::from("reference")])) == "text"), "assertion 39");
    assert!(((self::read_rust_borrowed_source_observation_role(String::from("reference"), String::from("string"), String::from("library"), String::from("std::string::String"), 0f64, vec![String::from("reference")])) == "text"), "assertion 40");
    assert!(((self::read_rust_borrowed_source_observation_role(String::from("reference"), String::from("string"), String::from("library"), String::from("alloc::string::String"), 0f64, vec![String::from("reference")])) == "text"), "assertion 41");
    assert!(((self::read_rust_borrowed_source_observation_role(String::from("reference"), String::from("string"), String::from("declaration"), String::from("str"), 0f64, vec![String::from("reference")])) == "reference"), "assertion 42");
    assert!(((self::read_rust_borrowed_source_observation_role(String::from("reference"), String::from("string"), String::from("primitive"), String::from("str"), 0f64, vec![String::from("reference"), String::from("reference")])) == "reference"), "assertion 43");
    assert!(((self::read_rust_borrowed_source_match_operation(String::from("match"))) == "match"), "assertion 44");
    assert!(((self::read_rust_borrowed_source_match_operation(String::from("if"))) == ""), "assertion 45");
    assert!(((self::read_rust_borrowed_source_match_error(String::from("reference"), String::from("data"), String::from("ctor"), 2f64, 2f64)) == ""), "assertion 46");
    assert!(((self::read_rust_borrowed_source_match_error(String::from("owned"), String::from("data"), String::from("ctor"), 2f64, 2f64)) == "source matching requires its actual borrowed nominal place adapter"), "assertion 47");
    assert!(((self::read_rust_borrowed_source_match_error(String::from("reference"), String::from("data"), String::from("ctor"), 2f64, 1f64)) == "source constructor patterns need every actual declared field position"), "assertion 48");
    assert!(((self::read_rust_borrowed_source_match_error(String::from("reference"), String::from("data"), String::from("wild"), 0f64, 0f64)) == ""), "assertion 49");
    assert!(self::ml_number::same_value(self::reserve_rust_borrowed_source_branches(1048570f64, 6f64), 1048576f64), "assertion 50");
    assert!(self::ml_number::same_value(self::reserve_rust_borrowed_source_branches(1048570f64, 7f64), (-1f64)), "assertion 51");
    assert!(self::ml_number::same_value(self::reserve_rust_borrowed_source_branches(0f64, (-0f64)), (-1f64)), "assertion 52");
    assert!(((self::merge_rust_borrowed_source_region(String::from("available"), String::from("moved"))) == "possibly moved"), "assertion 53");
    assert!(((self::merge_rust_borrowed_source_region(String::from("available"), String::from("available"))) == "available"), "assertion 54");
    assert!(((self::accept_rust_borrowed_source_box_reference(vec![String::from("reference"), String::from("box")], vec![String::from("reference")], true, true)) == (true)), "assertion 55");
    assert!(((self::accept_rust_borrowed_source_box_reference(vec![String::from("reference"), String::from("box")], vec![String::from("reference")], false, true)) == (false)), "assertion 56");
    assert!(((self::accept_rust_borrowed_source_box_reference(vec![String::from("reference"), String::from("box")], vec![String::from("reference")], true, false)) == (false)), "assertion 57");
    assert!(((self::accept_rust_borrowed_source_box_reference(vec![String::from("reference"), String::from("reference")], vec![String::from("reference")], true, true)) == (false)), "assertion 58");
    assert!(((self::accept_rust_borrowed_source_box_reference(vec![String::from("reference")], vec![String::from("reference"), String::from("box")], true, true)) == (false)), "assertion 59");
    assert!(((self::read_rust_borrowed_source_adjustment(String::from("reference"), String::from("reference"), false, false, false, true, false, false, 1f64, 1f64)) == "unsupported"), "assertion 60");
    assert!(((self::read_rust_borrowed_source_iterator_operation(String::from("method"), String::from("map"))) == "iterator map"), "assertion 61");
    assert!(((self::read_rust_borrowed_source_iterator_operation(String::from("method"), String::from("slice"))) == "iterator slice"), "assertion 62");
    assert!(((self::read_rust_borrowed_source_iterator_operation(String::from("append"), String::from(""))) == "iterator concat"), "assertion 63");
    assert!(((self::read_rust_borrowed_source_iterator_operation(String::from("method"), String::from("concat"))) == ""), "assertion 64");
    assert!(((self::read_rust_borrowed_source_iterator_operation(String::from("method"), String::from("reduce"))) == ""), "assertion 65");
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
fn all_literal_borrowed_source_region_controls() { main(); }
