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
