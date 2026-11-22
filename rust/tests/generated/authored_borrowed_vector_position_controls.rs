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

pub fn accept_rust_retained_source_signature_model(model: String) -> bool {
    (model == "Rust source function signature 2")
}

pub fn select_rust_retained_source_signature_model(model: String, retain: bool) -> String {
    if (model != "") {
        model.clone()
    } else {
        if retain {
            String::from("Rust source function signature 2")
        } else {
            String::from("")
        }
    }
}

pub fn accept_rust_retained_function_owner(model: String, owner: String, type_: String, paths: f64, children: f64, parameters: f64, reference: bool) -> bool {
    ((((((((self::accept_rust_retained_source_signature_model(model.clone()) && (owner == "function")) && (type_ == "function")) && (paths == (0f64))) && self::ml_math::is_integer(parameters)) && (parameters >= (0f64))) && (parameters <= (100000f64))) && (children == ((parameters + 1f64)))) && !reference)
}

pub fn read_rust_primitive_owner_path(segments: std::rc::Rc<std::cell::RefCell<Vec<String>>>, ordinary: bool, shadowed: bool, absolute: bool) -> String {
    if (!ordinary || shadowed) {
        String::from("")
    } else {
        {
            let owner = String::from("");
            {
                let owner_4 = if ((({ let ml_reference_array1 = segments.clone(); let ml_reference_projection1 = ml_reference_array1.borrow().len() as f64; ml_reference_projection1 }) == (2f64)) && !absolute) {
                    {
                        let owner_2 = ({
                            let source_ml_source2_array_read_value1 = segments.clone();
                            {
                                let source_ml_source2_array_read_index2 = 0f64;
                                { let ml_reference_array2 = source_ml_source2_array_read_value1.clone(); let ml_reference_position2 = source_ml_source2_array_read_index2; let ml_reference_projection2 = self::ml_array::optional_at(&*ml_reference_array2.borrow(), self::ml_array::number_index(ml_reference_position2)); ml_reference_projection2 }
                            }
                        }).clone().unwrap_or_else(|| String::from(""));
                        owner_2.clone()
                    }
                } else {
                    if (((({ let ml_reference_array3 = segments.clone(); let ml_reference_projection3 = ml_reference_array3.borrow().len() as f64; ml_reference_projection3 }) == (4f64)) && ({
                        let source_ml_source2_comparison_operand5 = {
                            let source_ml_source2_array_read_value3 = segments.clone();
                            {
                                let source_ml_source2_array_read_index4 = 0f64;
                                { let ml_reference_array4 = source_ml_source2_array_read_value3.clone(); let ml_reference_position4 = source_ml_source2_array_read_index4; let ml_reference_projection4 = self::ml_array::optional_at(&*ml_reference_array4.borrow(), self::ml_array::number_index(ml_reference_position4)); ml_reference_projection4 }
                            }
                        };
                        {
                            let source_ml_source2_comparison_operand6 = String::from("std");
                            match (source_ml_source2_comparison_operand5.clone(), source_ml_source2_comparison_operand6.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        }
                    } || {
                        let source_ml_source2_comparison_operand9 = {
                            let source_ml_source2_array_read_value7 = segments.clone();
                            {
                                let source_ml_source2_array_read_index8 = 0f64;
                                { let ml_reference_array5 = source_ml_source2_array_read_value7.clone(); let ml_reference_position5 = source_ml_source2_array_read_index8; let ml_reference_projection5 = self::ml_array::optional_at(&*ml_reference_array5.borrow(), self::ml_array::number_index(ml_reference_position5)); ml_reference_projection5 }
                            }
                        };
                        {
                            let source_ml_source2_comparison_operand10 = String::from("core");
                            match (source_ml_source2_comparison_operand9.clone(), source_ml_source2_comparison_operand10.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        }
                    })) && {
                        let source_ml_source2_comparison_operand13 = {
                            let source_ml_source2_array_read_value11 = segments.clone();
                            {
                                let source_ml_source2_array_read_index12 = 1f64;
                                { let ml_reference_array6 = source_ml_source2_array_read_value11.clone(); let ml_reference_position6 = source_ml_source2_array_read_index12; let ml_reference_projection6 = self::ml_array::optional_at(&*ml_reference_array6.borrow(), self::ml_array::number_index(ml_reference_position6)); ml_reference_projection6 }
                            }
                        };
                        {
                            let source_ml_source2_comparison_operand14 = String::from("primitive");
                            match (source_ml_source2_comparison_operand13.clone(), source_ml_source2_comparison_operand14.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        }
                    }) {
                        {
                            let owner_3 = ({
                                let source_ml_source2_array_read_value15 = segments.clone();
                                {
                                    let source_ml_source2_array_read_index16 = 2f64;
                                    { let ml_reference_array7 = source_ml_source2_array_read_value15.clone(); let ml_reference_position7 = source_ml_source2_array_read_index16; let ml_reference_projection7 = self::ml_array::optional_at(&*ml_reference_array7.borrow(), self::ml_array::number_index(ml_reference_position7)); ml_reference_projection7 }
                                }
                            }).clone().unwrap_or_else(|| String::from(""));
                            owner_3.clone()
                        }
                    } else {
                        owner.clone()
                    }
                };
                if self::ml_array::includes(&{ let ml_reference_value8 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("bool"), String::from("i8"), String::from("u8"), String::from("i16"), String::from("u16"), String::from("i32"), String::from("u32"), String::from("i64"), String::from("u64"), String::from("i128"), String::from("u128"), String::from("isize"), String::from("usize")]))).borrow().clone(); ml_reference_value8 }, &owner_4.clone(), 0f64, false, |a, b| a == b) {
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
    ((method == "from") && self::ml_array::includes(&{ let ml_reference_value9 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("bool"), String::from("i8"), String::from("u8"), String::from("i16"), String::from("u16"), String::from("i32"), String::from("u32"), String::from("i64"), String::from("u64"), String::from("i128"), String::from("u128"), String::from("isize"), String::from("usize"), String::from("f64")]))).borrow().clone(); ml_reference_value9 }, &owner.clone(), 0f64, false, |a, b| a == b))
}

pub fn read_rust_primitive_from_owner(segments: std::rc::Rc<std::cell::RefCell<Vec<String>>>, ordinary: bool, shadowed: bool, absolute: bool) -> String {
    if (!ordinary || shadowed) {
        String::from("")
    } else {
        if ((((({ let ml_reference_array10 = segments.clone(); let ml_reference_projection10 = ml_reference_array10.borrow().len() as f64; ml_reference_projection10 }) == (2f64)) && !absolute) && {
            let source_ml_source2_comparison_operand19 = {
                let source_ml_source2_array_read_value17 = segments.clone();
                {
                    let source_ml_source2_array_read_index18 = 0f64;
                    { let ml_reference_array11 = source_ml_source2_array_read_value17.clone(); let ml_reference_position11 = source_ml_source2_array_read_index18; let ml_reference_projection11 = self::ml_array::optional_at(&*ml_reference_array11.borrow(), self::ml_array::number_index(ml_reference_position11)); ml_reference_projection11 }
                }
            };
            {
                let source_ml_source2_comparison_operand20 = String::from("f64");
                match (source_ml_source2_comparison_operand19.clone(), source_ml_source2_comparison_operand20.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) || ((((({ let ml_reference_array12 = segments.clone(); let ml_reference_projection12 = ml_reference_array12.borrow().len() as f64; ml_reference_projection12 }) == (4f64)) && ({
            let source_ml_source2_comparison_operand23 = {
                let source_ml_source2_array_read_value21 = segments.clone();
                {
                    let source_ml_source2_array_read_index22 = 0f64;
                    { let ml_reference_array13 = source_ml_source2_array_read_value21.clone(); let ml_reference_position13 = source_ml_source2_array_read_index22; let ml_reference_projection13 = self::ml_array::optional_at(&*ml_reference_array13.borrow(), self::ml_array::number_index(ml_reference_position13)); ml_reference_projection13 }
                }
            };
            {
                let source_ml_source2_comparison_operand24 = String::from("std");
                match (source_ml_source2_comparison_operand23.clone(), source_ml_source2_comparison_operand24.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        } || {
            let source_ml_source2_comparison_operand27 = {
                let source_ml_source2_array_read_value25 = segments.clone();
                {
                    let source_ml_source2_array_read_index26 = 0f64;
                    { let ml_reference_array14 = source_ml_source2_array_read_value25.clone(); let ml_reference_position14 = source_ml_source2_array_read_index26; let ml_reference_projection14 = self::ml_array::optional_at(&*ml_reference_array14.borrow(), self::ml_array::number_index(ml_reference_position14)); ml_reference_projection14 }
                }
            };
            {
                let source_ml_source2_comparison_operand28 = String::from("core");
                match (source_ml_source2_comparison_operand27.clone(), source_ml_source2_comparison_operand28.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        })) && {
            let source_ml_source2_comparison_operand31 = {
                let source_ml_source2_array_read_value29 = segments.clone();
                {
                    let source_ml_source2_array_read_index30 = 1f64;
                    { let ml_reference_array15 = source_ml_source2_array_read_value29.clone(); let ml_reference_position15 = source_ml_source2_array_read_index30; let ml_reference_projection15 = self::ml_array::optional_at(&*ml_reference_array15.borrow(), self::ml_array::number_index(ml_reference_position15)); ml_reference_projection15 }
                }
            };
            {
                let source_ml_source2_comparison_operand32 = String::from("primitive");
                match (source_ml_source2_comparison_operand31.clone(), source_ml_source2_comparison_operand32.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) && {
            let source_ml_source2_comparison_operand35 = {
                let source_ml_source2_array_read_value33 = segments.clone();
                {
                    let source_ml_source2_array_read_index34 = 2f64;
                    { let ml_reference_array16 = source_ml_source2_array_read_value33.clone(); let ml_reference_position16 = source_ml_source2_array_read_index34; let ml_reference_projection16 = self::ml_array::optional_at(&*ml_reference_array16.borrow(), self::ml_array::number_index(ml_reference_position16)); ml_reference_projection16 }
                }
            };
            {
                let source_ml_source2_comparison_operand36 = String::from("f64");
                match (source_ml_source2_comparison_operand35.clone(), source_ml_source2_comparison_operand36.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        })) {
            String::from("f64")
        } else {
            self::read_rust_primitive_owner_path(segments.clone(), ordinary, shadowed, absolute)
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
                if ((target == "f64") && self::ml_array::includes(&{ let ml_reference_value17 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("i8"), String::from("i16"), String::from("i32"), String::from("u8"), String::from("u16"), String::from("u32")]))).borrow().clone(); ml_reference_value17 }, &source.clone(), 0f64, false, |a, b| a == b)) {
                    String::from("floating exact")
                } else {
                    {
                        let admitted = (((((((((source == "u8") && self::ml_array::includes(&{ let ml_reference_value18 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("u16"), String::from("u32"), String::from("u64"), String::from("u128"), String::from("usize"), String::from("i16"), String::from("i32"), String::from("i64"), String::from("i128"), String::from("isize")]))).borrow().clone(); ml_reference_value18 }, &target.clone(), 0f64, false, |a, b| a == b)) || ((source == "u16") && self::ml_array::includes(&{ let ml_reference_value19 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("u32"), String::from("u64"), String::from("u128"), String::from("usize"), String::from("i32"), String::from("i64"), String::from("i128")]))).borrow().clone(); ml_reference_value19 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "u32") && self::ml_array::includes(&{ let ml_reference_value20 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("u64"), String::from("u128"), String::from("i64"), String::from("i128")]))).borrow().clone(); ml_reference_value20 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "u64") && self::ml_array::includes(&{ let ml_reference_value21 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("u128"), String::from("i128")]))).borrow().clone(); ml_reference_value21 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i8") && self::ml_array::includes(&{ let ml_reference_value22 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("i16"), String::from("i32"), String::from("i64"), String::from("i128"), String::from("isize")]))).borrow().clone(); ml_reference_value22 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i16") && self::ml_array::includes(&{ let ml_reference_value23 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("i32"), String::from("i64"), String::from("i128"), String::from("isize")]))).borrow().clone(); ml_reference_value23 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i32") && self::ml_array::includes(&{ let ml_reference_value24 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("i64"), String::from("i128")]))).borrow().clone(); ml_reference_value24 }, &target.clone(), 0f64, false, |a, b| a == b))) || ((source == "i64") && (target == "i128")));
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
    if self::ml_array::includes(&{ let ml_reference_value25 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Option"), String::from("std::option::Option"), String::from("core::option::Option"), String::from("None"), String::from("Some"), String::from("Option::None"), String::from("Option::Some"), String::from("std::option::Option::None"), String::from("std::option::Option::Some"), String::from("core::option::Option::None"), String::from("core::option::Option::Some")]))).borrow().clone(); ml_reference_value25 }, &path.clone(), 0f64, false, |a, b| a == b) {
        String::from("std::option::Option")
    } else {
        if self::ml_array::includes(&{ let ml_reference_value26 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Result"), String::from("std::result::Result"), String::from("core::result::Result"), String::from("Ok"), String::from("Err"), String::from("Result::Ok"), String::from("Result::Err"), String::from("std::result::Result::Ok"), String::from("std::result::Result::Err"), String::from("core::result::Result::Ok"), String::from("core::result::Result::Err")]))).borrow().clone(); ml_reference_value26 }, &path.clone(), 0f64, false, |a, b| a == b) {
            String::from("std::result::Result")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_standard_sum_source_path_constructor(path: String) -> String {
    if self::ml_array::includes(&{ let ml_reference_value27 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("None"), String::from("Option::None"), String::from("std::option::Option::None"), String::from("core::option::Option::None")]))).borrow().clone(); ml_reference_value27 }, &path.clone(), 0f64, false, |a, b| a == b) {
        String::from("None")
    } else {
        if self::ml_array::includes(&{ let ml_reference_value28 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Some"), String::from("Option::Some"), String::from("std::option::Option::Some"), String::from("core::option::Option::Some")]))).borrow().clone(); ml_reference_value28 }, &path.clone(), 0f64, false, |a, b| a == b) {
            String::from("Some")
        } else {
            if self::ml_array::includes(&{ let ml_reference_value29 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Ok"), String::from("Result::Ok"), String::from("std::result::Result::Ok"), String::from("core::result::Result::Ok")]))).borrow().clone(); ml_reference_value29 }, &path.clone(), 0f64, false, |a, b| a == b) {
                String::from("Ok")
            } else {
                if self::ml_array::includes(&{ let ml_reference_value30 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Err"), String::from("Result::Err"), String::from("std::result::Result::Err"), String::from("core::result::Result::Err")]))).borrow().clone(); ml_reference_value30 }, &path.clone(), 0f64, false, |a, b| a == b) {
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
            let source_ml_source2_string_argument37 = ordinal;
            self::ml_number::js_number(source_ml_source2_string_argument37)
        }))), (String::from("_")))), ({
            let source_ml_source2_string_argument38 = collision;
            self::ml_number::js_number(source_ml_source2_string_argument38)
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

pub fn read_rust_standard_sum_source_depth(children: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, sum: bool) -> f64 {
    {
        let depth = 0f64;
        {
            let source_ml_source1_temporary_values1 = children.clone();
            {
                let source_ml_source1_temporary_index1 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s39 = self::source_ml_source1_temporary_read_rust_standard_sum_source_depth_loop2(depth, source_ml_source1_temporary_values1.clone(), source_ml_source1_temporary_index1.clone());
                    match source_ml_source2_s39.clone() {
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
        let source_ml_source1_temporary_values3 = std::rc::Rc::new(std::cell::RefCell::new(vec![nodes, units, additional_nodes, additional_units]));
        {
            let source_ml_source1_temporary_index3 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s40 = self::source_ml_source1_temporary_read_rust_standard_sum_source_materialization_error_loop4(source_ml_source1_temporary_values3.clone(), source_ml_source1_temporary_index3.clone());
                match source_ml_source2_s40.clone() {
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

pub fn accept_rust_standard_sum_source_arguments(owner: String, identities: std::rc::Rc<std::cell::RefCell<Vec<String>>>, direct_copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>) -> bool {
    {
        let count = self::read_rust_standard_sum_source_argument_count(owner.clone());
        if (((count < (0f64)) || (({ let ml_reference_array31 = identities.clone(); let ml_reference_projection31 = ml_reference_array31.borrow().len() as f64; ml_reference_projection31 }) != count)) || (({ let ml_reference_array32 = direct_copies.clone(); let ml_reference_projection32 = ml_reference_array32.borrow().len() as f64; ml_reference_projection32 }) != count)) {
            false
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source2_s41 = self::source_ml_source1_temporary_accept_rust_standard_sum_source_arguments_loop5(identities.clone(), direct_copies.clone(), count, at);
                    match source_ml_source2_s41.clone() {
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

pub fn read_rust_standard_sum_source_constructors(owner: String) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    if (owner == "std::option::Option") {
        std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("None"), String::from("Some")]))
    } else {
        if (owner == "std::result::Result") {
            std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("Ok"), String::from("Err")]))
        } else {
            std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()))
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

pub fn read_rust_standard_sum_primitive_question_conversion_plan(operand_owner: String, return_owner: String, ordinary: bool, role: String, wrappers: f64, direct_copy: bool, source_known: bool, target_known: bool, source: String, target: String, minor: f64) -> std::rc::Rc<std::cell::RefCell<Vec<String>>> {
    if ((((((((!ordinary || (operand_owner != "std::result::Result")) || (return_owner != "std::result::Result")) || (wrappers != (0f64))) || !source_known) || !target_known) || !(self::ml_math::is_integer(minor))) || (minor < (90f64))) || (minor >= (100f64))) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()))
    } else {
        if ((source == target) || ((role != "owned") && ((role != "copy") || !direct_copy))) {
            std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()))
        } else {
            {
                let plan = self::plan_rust_primitive_from(target.clone(), source.clone());
                if (plan == "") {
                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()))
                } else {
                    std::rc::Rc::new(std::cell::RefCell::new(vec![role.clone(), target.clone(), plan.clone()]))
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

pub fn accept_rust_source_array_owner(owner: String, shadowed: bool) -> bool {
    (!shadowed && ((owner == "Vec") || (owner == "std::vec::Vec")))
}

pub fn accept_rust_source_array_element(kind: String, indirected: bool) -> bool {
    (!indirected && (((((kind == "bool") || (kind == "float")) || (kind == "fixed")) || (kind == "unit")) || (kind == "string")))
}

pub fn accept_rust_source_array_coercion(actual: String, expected: String, borrowed: bool) -> bool {
    ((actual == expected) || ((borrowed && (actual == "owned vector")) && (expected == "immutable slice")))
}

pub fn accept_rust_source_array_result_model(model: String) -> bool {
    ((model == "") || (model == "immutable fresh scalar vector results 1"))
}

pub fn accept_rust_source_array_result(model: String, kind: String, owned: bool, fresh: bool) -> bool {
    ((((model == "immutable fresh scalar vector results 1") && owned) && fresh) && ((((kind == "bool") || (kind == "float")) || (kind == "fixed")) || (kind == "unit")))
}

pub fn accept_rust_fresh_array_result_tail(kind: String, constructor: bool, children: bool) -> bool {
    (((kind == "array") && constructor) || (((kind == "let") || (kind == "if")) && children))
}

pub fn accept_rust_borrowed_vector_completion_profiles(signature: String, profiles: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> bool {
    if ((!(self::accept_rust_borrowed_vector_signature_model(signature.clone())) || (({ let ml_reference_array33 = profiles.clone(); let ml_reference_projection33 = ml_reference_array33.borrow().len() as f64; ml_reference_projection33 }) != (11f64))) || {
        let source_ml_source2_comparison_operand44 = {
            let source_ml_source2_array_read_value42 = profiles.clone();
            {
                let source_ml_source2_array_read_index43 = 1f64;
                { let ml_reference_array34 = source_ml_source2_array_read_value42.clone(); let ml_reference_position34 = source_ml_source2_array_read_index43; let ml_reference_projection34 = self::ml_array::optional_at(&*ml_reference_array34.borrow(), self::ml_array::number_index(ml_reference_position34)); ml_reference_projection34 }
            }
        };
        {
            let source_ml_source2_comparison_operand45 = signature.clone();
            !(match (source_ml_source2_comparison_operand44.clone(), source_ml_source2_comparison_operand45.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    }) {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s46 = self::source_ml_source1_temporary_accept_rust_borrowed_vector_completion_profiles_loop6(profiles.clone(), index);
                match source_ml_source2_s46.clone() {
                    self::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Done => {
                        true
                    }
                    self::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Return(source_ml_source1_temporary_result6) => {
                        source_ml_source1_temporary_result6
                    }
                }
            }
        }
    }
}

pub fn accept_rust_borrowed_vector_inherent_profiles(profiles: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> bool {
    if ((({ let ml_reference_array35 = profiles.clone(); let ml_reference_projection35 = ml_reference_array35.borrow().len() as f64; ml_reference_projection35 }) != (11f64)) || !(self::accept_rust_borrowed_vector_signature_model(({
        let source_ml_source2_array_read_value47 = profiles.clone();
        {
            let source_ml_source2_array_read_index48 = 2f64;
            { let ml_reference_array36 = source_ml_source2_array_read_value47.clone(); let ml_reference_position36 = source_ml_source2_array_read_index48; let ml_reference_projection36 = self::ml_array::optional_at(&*ml_reference_array36.borrow(), self::ml_array::number_index(ml_reference_position36)); ml_reference_projection36 }
        }
    }).clone().unwrap_or_else(|| String::from(""))))) {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s49 = self::source_ml_source1_temporary_accept_rust_borrowed_vector_inherent_profiles_loop7(profiles.clone(), index);
                match source_ml_source2_s49.clone() {
                    self::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Done => {
                        true
                    }
                    self::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Return(source_ml_source1_temporary_result7) => {
                        source_ml_source1_temporary_result7
                    }
                }
            }
        }
    }
}

pub fn accept_rust_borrowed_vector_source(model: String, ordinary: bool, minor: f64) -> bool {
    ((((self::accept_rust_borrowed_vector_signature_model(model.clone()) && ordinary) && self::ml_math::is_integer(minor)) && (minor >= (90f64))) && (minor < (100f64)))
}

pub fn accept_rust_borrowed_vector_signature_model(model: String) -> bool {
    (model == "Rust source function signature 3")
}

pub fn retain_rust_borrowed_vector_element_position(wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, child: bool) -> bool {
    (self::ml_array::includes(&{ let ml_reference_value37 = wrappers.borrow().clone(); ml_reference_value37 }, &String::from("reference"), 0f64, false, |a, b| a == b) || child)
}

pub fn accept_rust_borrowed_vector_owned_interpretation(child: bool) -> bool {
    !child
}

pub fn accept_rust_borrowed_vector_position(wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, lifetimes: f64, nested: bool) -> bool {
    ((((({ let ml_reference_array38 = wrappers.clone(); let ml_reference_projection38 = ml_reference_array38.borrow().len() as f64; ml_reference_projection38 }) == (1f64)) && {
        let source_ml_source2_comparison_operand52 = {
            let source_ml_source2_array_read_value50 = wrappers.clone();
            {
                let source_ml_source2_array_read_index51 = 0f64;
                { let ml_reference_array39 = source_ml_source2_array_read_value50.clone(); let ml_reference_position39 = source_ml_source2_array_read_index51; let ml_reference_projection39 = self::ml_array::optional_at(&*ml_reference_array39.borrow(), self::ml_array::number_index(ml_reference_position39)); ml_reference_projection39 }
            }
        };
        {
            let source_ml_source2_comparison_operand53 = String::from("reference");
            match (source_ml_source2_comparison_operand52.clone(), source_ml_source2_comparison_operand53.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && (lifetimes == (1f64))) && !nested)
}

pub fn read_rust_borrowed_vector_position_error(model: String, child: bool) -> String {
    if (child && !(self::accept_rust_borrowed_vector_signature_model(model.clone()))) {
        String::from("a retained array element position requires the borrowed vector source signature profile")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_vector_element_position_error(kind: String, owner: String, path: String, arguments_count: f64, type_equal: bool, owner_equal: bool) -> String {
    if ((((((kind != "array") || (owner != "library")) || (path != "std::vec::Vec")) || (arguments_count != (1f64))) || !type_equal) || !owner_equal) {
        String::from("a retained array element position must match its complete canonical vector element type and owner")
    } else {
        String::from("")
    }
}

pub fn accept_rust_borrowed_vector_element(selected: bool, minor: f64, wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, lifetimes: std::rc::Rc<std::cell::RefCell<Vec<String>>>, known: bool, mutable: bool) -> bool {
    ((((((((selected && self::ml_math::is_integer(minor)) && (minor >= (90f64))) && (minor <= (99f64))) && known) && !mutable) && (({ let ml_reference_array40 = wrappers.clone(); let ml_reference_projection40 = ml_reference_array40.borrow().len() as f64; ml_reference_projection40 }) == (1f64))) && {
        let source_ml_source2_comparison_operand56 = {
            let source_ml_source2_array_read_value54 = wrappers.clone();
            {
                let source_ml_source2_array_read_index55 = 0f64;
                { let ml_reference_array41 = source_ml_source2_array_read_value54.clone(); let ml_reference_position41 = source_ml_source2_array_read_index55; let ml_reference_projection41 = self::ml_array::optional_at(&*ml_reference_array41.borrow(), self::ml_array::number_index(ml_reference_position41)); ml_reference_projection41 }
            }
        };
        {
            let source_ml_source2_comparison_operand57 = String::from("reference");
            match (source_ml_source2_comparison_operand56.clone(), source_ml_source2_comparison_operand57.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && (({ let ml_reference_array42 = lifetimes.clone(); let ml_reference_projection42 = ml_reference_array42.borrow().len() as f64; ml_reference_projection42 }) == (1f64)))
}

pub fn read_rust_borrowed_vector_position_budget_error(visits: f64, depth: f64) -> String {
    if (((((!(self::ml_math::is_safe_integer(visits)) || (visits < (0f64))) || (visits > (100000f64))) || !(self::ml_math::is_safe_integer(depth))) || (depth < (0f64))) || (depth > (128f64))) {
        String::from("retained borrowed vector element positions exceed their finite source profile")
    } else {
        String::from("")
    }
}

pub fn accept_rust_borrowed_vector_function_owner(model: String, owner: String, type_: String, paths: f64, children: f64, parameters: f64, reference: bool) -> bool {
    (self::accept_rust_borrowed_vector_signature_model(model.clone()) && self::accept_rust_retained_function_owner(String::from("Rust source function signature 2"), owner.clone(), type_.clone(), paths, children, parameters, reference))
}

pub fn read_rust_borrowed_vector_element_phase(selected: bool, reference: bool, header: bool, complete: bool, closed: bool) -> String {
    if (!selected || !reference) {
        String::from("")
    } else {
        if (header && !complete) {
            String::from("defer")
        } else {
            if (complete && closed) {
                String::from("admit")
            } else {
                String::from("")
            }
        }
    }
}

pub fn accept_rust_borrowed_vector_shape(selected: bool, kind: String, owner: String, path: String, arguments_count: f64, wrappers: f64, child: bool) -> bool {
    ((((((selected && (kind == "array")) && (owner == "library")) && (path == "std::vec::Vec")) && (arguments_count == (1f64))) && (wrappers == (0f64))) && child)
}

pub fn plan_rust_borrowed_vector_result_parameters(lifetime: String, references: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, lifetimes: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if ((({ let ml_reference_array43 = references.clone(); let ml_reference_projection43 = ml_reference_array43.borrow().len() as f64; ml_reference_projection43 }) != ({ let ml_reference_array44 = lifetimes.clone(); let ml_reference_projection44 = ml_reference_array44.borrow().len() as f64; ml_reference_projection44 })) || (({ let ml_reference_array45 = references.clone(); let ml_reference_projection45 = ml_reference_array45.borrow().len() as f64; ml_reference_projection45 }) > (65536f64))) {
        std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)]))
    } else {
        {
            let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
            {
                let index = 0f64;
                {
                    let result_2 = self::source_ml_source1_temporary_plan_rust_borrowed_vector_result_parameters_loop8(lifetime.clone(), references.clone(), lifetimes.clone(), result.clone(), index);
                    if (lifetime == "static") {
                        result_2.clone()
                    } else {
                        if ((lifetime == "") || (lifetime == "_")) {
                            if (({ let ml_reference_array46 = result_2.clone(); let ml_reference_projection46 = ml_reference_array46.borrow().len() as f64; ml_reference_projection46 }) == (1f64)) {
                                result_2.clone()
                            } else {
                                std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)]))
                            }
                        } else {
                            if { let ml_value = { let ml_reference_array48 = result_2.clone(); let ml_reference_projection48 = ml_reference_array48.borrow().len() as f64; ml_reference_projection48 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                result_2.clone()
                            } else {
                                std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)]))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_borrowed_vector_production_error(same: bool, reference: bool, available: bool, collecting: bool) -> String {
    if (!same || !reference) {
        String::from("a borrowed vector element needs its actual matching immutable reference position")
    } else {
        if (!collecting && !available) {
            String::from("a borrowed vector element needs its actual live source region")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_borrowed_vector_result_error(same: bool, produced: bool, allowed: bool, collecting: bool) -> String {
    if (!same || (!collecting && (!produced || !allowed))) {
        String::from("a borrowed vector result needs an actual producer and parameter-derived or static element regions")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_vector_element_count_error(count: f64) -> String {
    if ((!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) || (count > (65536f64))) {
        String::from("borrowed vector construction exceeds its finite actual element profile")
    } else {
        String::from("")
    }
}

pub fn accept_rust_borrowed_vector_literal_context(selected: bool, expected: bool, reference: bool, count: f64) -> bool {
    (((((selected && expected) && self::ml_math::is_safe_integer(count)) && (count >= (0f64))) && (count <= (65536f64))) && ((count == (0f64)) || reference))
}

pub fn read_rust_borrowed_vector_empty_constructor(selected: bool, owner: String, method: String, count: f64) -> String {
    if ((selected && self::accept_rust_source_array_owner(owner.clone(), false)) && (method == "new")) {
        if (count == (0f64)) {
            String::from("construct")
        } else {
            String::from("arity")
        }
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_vector_argument_error(actual: bool, expected: bool, same: bool, owner: bool, lifetime: String, required: String) -> String {
    if ((actual != expected) || (actual && ((!same || !owner) || ((required == "static") && (lifetime != "static"))))) {
        String::from("borrowed vector arguments must retain their actual reference element owner and region contract")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_vector_observation_error(borrowed: bool, operation: String) -> String {
    if (borrowed && !(self::ml_array::includes(&{ let ml_reference_value49 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("len"), String::from("is_empty"), String::from("transfer")]))).borrow().clone(); ml_reference_value49 }, &operation.clone(), 0f64, false, |a, b| a == b))) {
        String::from("borrowed vector observations need their independent reference-preserving source operation profile")
    } else {
        String::from("")
    }
}

pub fn accept_rust_borrowed_vector_produced_value(selected: bool, shape: bool, proven: bool, producer: bool) -> bool {
    (((selected && shape) && proven) && producer)
}

pub fn read_rust_borrowed_vector_call_region_error(reference: bool, regions: f64, collecting: bool) -> String {
    if ((!(self::ml_math::is_safe_integer(regions)) || (regions < (0f64))) || (regions > (65536f64))) {
        String::from("borrowed vector calls exceed their finite actual argument region profile")
    } else {
        if ((!collecting && reference) && (regions == (0f64))) {
            String::from("a borrowed vector call needs actual argument region provenance; anonymous temporary references need their separate promotion or extension proof")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_borrowed_expression_position_model_error(model: String, language: String) -> String {
    if ((model != "Rust borrowed expression source position 1") || (language != "Rust")) {
        String::from("a borrowed expression source position needs its exact Rust source interpretation")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_expression_lifetime_scope_error(lifetime: String, bindings: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> String {
    if ((((lifetime != "") && (lifetime != "_")) && (lifetime != "static")) && !(self::ml_array::includes(&{ let ml_reference_value50 = bindings.borrow().clone(); ml_reference_value50 }, &lifetime.clone(), 0f64, false, |a, b| a == b))) {
        String::from("a borrowed expression source lifetime needs its actual enclosing function binder")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_vector_call_lifetime(result: String, arguments_: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> String {
    if (result == "static") {
        String::from("static")
    } else {
        if (({ let ml_reference_array51 = arguments_.clone(); let ml_reference_projection51 = ml_reference_array51.borrow().len() as f64; ml_reference_projection51 }) == (0f64)) {
            String::from("")
        } else {
            {
                let first = ({
                    let source_ml_source2_array_read_value58 = arguments_.clone();
                    {
                        let source_ml_source2_array_read_index59 = 0f64;
                        { let ml_reference_array52 = source_ml_source2_array_read_value58.clone(); let ml_reference_position52 = source_ml_source2_array_read_index59; let ml_reference_projection52 = self::ml_array::optional_at(&*ml_reference_array52.borrow(), self::ml_array::number_index(ml_reference_position52)); ml_reference_projection52 }
                    }
                }).clone().unwrap_or_else(|| String::from(""));
                if ((first != "_") && { let ml_values53 = { let ml_reference_value54 = arguments_.borrow().clone(); ml_reference_value54 }; let ml_callback53 = { let first = first.clone(); std::rc::Rc::new(move |value: String| -> bool { (value == first) }) }; ml_values53.iter().cloned().enumerate().all(|(ml_index53, ml_value53)| ml_callback53(ml_value53)) }) {
                    first.clone()
                } else {
                    String::from("")
                }
            }
        }
    }
}

pub fn read_rust_borrowed_vector_static_reference_error(selected: bool, wrappers: std::rc::Rc<std::cell::RefCell<Vec<String>>>, lifetimes: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> String {
    if ((selected && {
        let source_ml_source2_comparison_operand62 = {
            let source_ml_source2_array_read_value60 = wrappers.clone();
            {
                let source_ml_source2_array_read_index61 = 0f64;
                { let ml_reference_array55 = source_ml_source2_array_read_value60.clone(); let ml_reference_position55 = source_ml_source2_array_read_index61; let ml_reference_projection55 = self::ml_array::optional_at(&*ml_reference_array55.borrow(), self::ml_array::number_index(ml_reference_position55)); ml_reference_projection55 }
            }
        };
        {
            let source_ml_source2_comparison_operand63 = String::from("reference");
            match (source_ml_source2_comparison_operand62.clone(), source_ml_source2_comparison_operand63.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand66 = {
            let source_ml_source2_array_read_value64 = lifetimes.clone();
            {
                let source_ml_source2_array_read_index65 = 0f64;
                { let ml_reference_array56 = source_ml_source2_array_read_value64.clone(); let ml_reference_position56 = source_ml_source2_array_read_index65; let ml_reference_projection56 = self::ml_array::optional_at(&*ml_reference_array56.borrow(), self::ml_array::number_index(ml_reference_position56)); ml_reference_projection56 }
            }
        };
        {
            let source_ml_source2_comparison_operand67 = String::from("static");
            match (source_ml_source2_comparison_operand66.clone(), source_ml_source2_comparison_operand67.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) {
        String::from("static source reference demands need their independent global or parameter region proof")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_vector_mutation_error(selected: bool) -> String {
    if selected {
        String::from("the borrowed vector source profile requires immutable bindings and explicit place-write provenance before source mutation")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_expression_position_reservation(nodes: f64, units: f64, added_nodes: f64, added_units: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let source_ml_source1_temporary_values9 = std::rc::Rc::new(std::cell::RefCell::new(vec![nodes, units, added_nodes, added_units]));
        {
            let source_ml_source1_temporary_index9 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s68 = self::source_ml_source1_temporary_read_rust_borrowed_expression_position_reservation_loop10(source_ml_source1_temporary_values9.clone(), source_ml_source1_temporary_index9.clone());
                match source_ml_source2_s68.clone() {
                    self::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Done => {
                        {
                            let next_nodes = (nodes + ((added_nodes + 2f64) * 4f64));
                            {
                                let next_units = (units + (((added_units + 13f64) + ((String::from("Rust borrowed expression source position 1")).encode_utf16().count() as f64)) * 4f64));
                                if !(self::read_rust_standard_sum_source_materialization_error(0f64, 0f64, next_nodes, next_units)).is_empty() {
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                } else {
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![next_nodes, next_units]))
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Return(source_ml_source1_temporary_result10) => {
                        source_ml_source1_temporary_result10.clone()
                    }
                }
            }
        }
    }
}

pub fn plan_rust_borrowed_inherent_position(actual: std::rc::Rc<std::cell::RefCell<Vec<String>>>, target: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let actual_boxes = 0f64;
        {
            let target_boxes = 0f64;
            {
                let actual_references = 0f64;
                {
                    let target_references = 0f64;
                    {
                        let source_ml_source1_temporary_values11 = actual.clone();
                        {
                            let source_ml_source1_temporary_index11 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s69 = self::source_ml_source1_temporary_plan_rust_borrowed_inherent_position_loop13(actual_boxes, actual_references, source_ml_source1_temporary_values11.clone(), source_ml_source1_temporary_index11.clone());
                                match source_ml_source2_s69.clone() {
                                    self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Done(actual_boxes_2, actual_references_2) => {
                                        {
                                            let source_ml_source1_temporary_values12 = target.clone();
                                            {
                                                let source_ml_source1_temporary_index12 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s70 = self::source_ml_source1_temporary_plan_rust_borrowed_inherent_position_loop14(target_boxes, target_references, source_ml_source1_temporary_values12.clone(), source_ml_source1_temporary_index12.clone());
                                                    match source_ml_source2_s70.clone() {
                                                        self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Done(target_boxes_2, target_references_2) => {
                                                            if (((({ let ml_reference_array57 = actual.clone(); let ml_reference_projection57 = ml_reference_array57.borrow().len() as f64; ml_reference_projection57 }) > (256f64)) || (({ let ml_reference_array58 = target.clone(); let ml_reference_projection58 = ml_reference_array58.borrow().len() as f64; ml_reference_projection58 }) > (256f64))) || (target_boxes_2 > actual_boxes_2)) {
                                                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                                            } else {
                                                                std::rc::Rc::new(std::cell::RefCell::new(vec![(actual_boxes_2 - target_boxes_2), self::ml_math::max((actual_references_2 - target_references_2), 0f64), self::ml_math::max((target_references_2 - actual_references_2), 0f64)]))
                                                            }
                                                        }
                                                        self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Return(source_ml_source1_temporary_result14) => {
                                                            source_ml_source1_temporary_result14.clone()
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Return(source_ml_source1_temporary_result14_2) => {
                                        source_ml_source1_temporary_result14_2.clone()
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result {
    MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Done(f64),
    MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Return(f64),
}

pub fn source_ml_source1_temporary_read_rust_standard_sum_source_depth_loop2(mut depth: f64, mut source_ml_source1_temporary_values1: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index1: self::ml::Big) -> self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result {
    loop {
        return if (source_ml_source1_temporary_index1 < ({ let ml_reference_array59 = source_ml_source1_temporary_values1.clone(); let ml_reference_projection59 = self::ml::Big::from_u128(ml_reference_array59.borrow().len() as u128); ml_reference_projection59 })) {
            {
                let child = {
                    let source_ml_source2_array_read_value71 = source_ml_source1_temporary_values1.clone();
                    {
                        let source_ml_source2_array_read_index72 = source_ml_source1_temporary_index1.clone();
                        { let ml_reference_array60 = source_ml_source2_array_read_value71.clone(); let ml_reference_position60 = source_ml_source2_array_read_index72.clone(); let ml_reference_projection60 = self::ml_array::at(&*ml_reference_array60.borrow(), ml_reference_position60.to_index()); ml_reference_projection60 }
                    }
                };
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

pub fn source_ml_source1_temporary_read_rust_standard_sum_source_materialization_error_loop4(mut source_ml_source1_temporary_values3: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index3: self::ml::Big) -> self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result {
    loop {
        return if (source_ml_source1_temporary_index3 < ({ let ml_reference_array61 = source_ml_source1_temporary_values3.clone(); let ml_reference_projection61 = self::ml::Big::from_u128(ml_reference_array61.borrow().len() as u128); ml_reference_projection61 })) {
            {
                let value = {
                    let source_ml_source2_array_read_value73 = source_ml_source1_temporary_values3.clone();
                    {
                        let source_ml_source2_array_read_index74 = source_ml_source1_temporary_index3.clone();
                        { let ml_reference_array62 = source_ml_source2_array_read_value73.clone(); let ml_reference_position62 = source_ml_source2_array_read_index74.clone(); let ml_reference_projection62 = self::ml_array::at(&*ml_reference_array62.borrow(), ml_reference_position62.to_index()); ml_reference_projection62 }
                    }
                };
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

pub fn source_ml_source1_temporary_accept_rust_standard_sum_source_arguments_loop5(mut identities: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut direct_copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut count: f64, mut at: f64) -> self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result {
    loop {
        return if (at < count) {
            if (((({
                let source_ml_source2_array_read_value77 = identities.clone();
                {
                    let source_ml_source2_array_read_index78 = at;
                    { let ml_reference_array63 = source_ml_source2_array_read_value77.clone(); let ml_reference_position63 = source_ml_source2_array_read_index78; let ml_reference_projection63 = self::ml_array::optional_at(&*ml_reference_array63.borrow(), self::ml_array::number_index(ml_reference_position63)); ml_reference_projection63 }
                }
            }).clone().unwrap_or_else(|| String::from(""))) == "") || {
                let source_ml_source2_comparison_operand81 = {
                    let source_ml_source2_array_read_value79 = direct_copies.clone();
                    {
                        let source_ml_source2_array_read_index80 = at;
                        { let ml_reference_array64 = source_ml_source2_array_read_value79.clone(); let ml_reference_position64 = source_ml_source2_array_read_index80; let ml_reference_projection64 = self::ml_array::optional_at(&*ml_reference_array64.borrow(), self::ml_array::number_index(ml_reference_position64)); ml_reference_projection64 }
                    }
                };
                {
                    let source_ml_source2_comparison_operand82 = true;
                    !(match (source_ml_source2_comparison_operand81.clone(), source_ml_source2_comparison_operand82) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result {
    MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Done,
    MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Return(bool),
}

pub fn source_ml_source1_temporary_accept_rust_borrowed_vector_completion_profiles_loop6(mut profiles: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut index: f64) -> self::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result {
    loop {
        return if (index < ({ let ml_reference_array65 = profiles.clone(); let ml_reference_projection65 = ml_reference_array65.borrow().len() as f64; ml_reference_projection65 })) {
            if ((index != (1f64)) && {
                let source_ml_source2_comparison_operand85 = {
                    let source_ml_source2_array_read_value83 = profiles.clone();
                    {
                        let source_ml_source2_array_read_index84 = index;
                        { let ml_reference_array66 = source_ml_source2_array_read_value83.clone(); let ml_reference_position66 = source_ml_source2_array_read_index84; let ml_reference_projection66 = self::ml_array::optional_at(&*ml_reference_array66.borrow(), self::ml_array::number_index(ml_reference_position66)); ml_reference_projection66 }
                    }
                };
                {
                    let source_ml_source2_comparison_operand86 = String::from("");
                    !(match (source_ml_source2_comparison_operand85.clone(), source_ml_source2_comparison_operand86.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            }) {
                self::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Return(false)
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
            self::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result {
    MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Done,
    MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Return(bool),
}

pub fn source_ml_source1_temporary_accept_rust_borrowed_vector_inherent_profiles_loop7(mut profiles: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut index: f64) -> self::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result {
    loop {
        return if (index < ({ let ml_reference_array67 = profiles.clone(); let ml_reference_projection67 = ml_reference_array67.borrow().len() as f64; ml_reference_projection67 })) {
            if ((index != (2f64)) && {
                let source_ml_source2_comparison_operand89 = {
                    let source_ml_source2_array_read_value87 = profiles.clone();
                    {
                        let source_ml_source2_array_read_index88 = index;
                        { let ml_reference_array68 = source_ml_source2_array_read_value87.clone(); let ml_reference_position68 = source_ml_source2_array_read_index88; let ml_reference_projection68 = self::ml_array::optional_at(&*ml_reference_array68.borrow(), self::ml_array::number_index(ml_reference_position68)); ml_reference_projection68 }
                    }
                };
                {
                    let source_ml_source2_comparison_operand90 = String::from("");
                    !(match (source_ml_source2_comparison_operand89.clone(), source_ml_source2_comparison_operand90.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            }) {
                self::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Return(false)
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
            self::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Done
        };
    }
}

pub fn source_ml_source1_temporary_plan_rust_borrowed_vector_result_parameters_loop8(mut lifetime: String, mut references: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut lifetimes: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (index < ({ let ml_reference_array69 = references.clone(); let ml_reference_projection69 = ml_reference_array69.borrow().len() as f64; ml_reference_projection69 })) {
            if match {
                let source_ml_source2_logical97 = {
                    let source_ml_source2_array_read_value91 = references.clone();
                    {
                        let source_ml_source2_array_read_index92 = index;
                        { let ml_reference_array70 = source_ml_source2_array_read_value91.clone(); let ml_reference_position70 = source_ml_source2_array_read_index92; let ml_reference_projection70 = self::ml_array::optional_at(&*ml_reference_array70.borrow(), self::ml_array::number_index(ml_reference_position70)); ml_reference_projection70 }
                    }
                };
                if match source_ml_source2_logical97.clone() { Some(ml_present) => ml_present, None => false } {
                    Some(if ((lifetime != "") && (lifetime != "_")) {
                        {
                            let source_ml_source2_comparison_operand95 = {
                                let source_ml_source2_array_read_value93 = lifetimes.clone();
                                {
                                    let source_ml_source2_array_read_index94 = index;
                                    { let ml_reference_array71 = source_ml_source2_array_read_value93.clone(); let ml_reference_position71 = source_ml_source2_array_read_index94; let ml_reference_projection71 = self::ml_array::optional_at(&*ml_reference_array71.borrow(), self::ml_array::number_index(ml_reference_position71)); ml_reference_projection71 }
                                }
                            };
                            {
                                let source_ml_source2_comparison_operand96 = lifetime.clone();
                                match (source_ml_source2_comparison_operand95.clone(), source_ml_source2_comparison_operand96.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                            }
                        }
                    } else {
                        true
                    })
                } else {
                    source_ml_source2_logical97.clone()
                }
            } { Some(ml_present) => ml_present, None => false } {
                {
                    let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value72 = result.borrow().clone(); ml_reference_value72 }, vec![index])));
                    {
                        let index_2 = (index + 1f64);
                        {
                            (result, index) = (result_2.clone(), index_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let index_3 = (index + 1f64);
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result {
    MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Done,
    MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_read_rust_borrowed_expression_position_reservation_loop10(mut source_ml_source1_temporary_values9: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index9: self::ml::Big) -> self::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result {
    loop {
        return if (source_ml_source1_temporary_index9 < ({ let ml_reference_array73 = source_ml_source1_temporary_values9.clone(); let ml_reference_projection73 = self::ml::Big::from_u128(ml_reference_array73.borrow().len() as u128); ml_reference_projection73 })) {
            {
                let value = {
                    let source_ml_source2_array_read_value98 = source_ml_source1_temporary_values9.clone();
                    {
                        let source_ml_source2_array_read_index99 = source_ml_source1_temporary_index9.clone();
                        { let ml_reference_array74 = source_ml_source2_array_read_value98.clone(); let ml_reference_position74 = source_ml_source2_array_read_index99.clone(); let ml_reference_projection74 = self::ml_array::at(&*ml_reference_array74.borrow(), ml_reference_position74.to_index()); ml_reference_projection74 }
                    }
                };
                if (!(self::ml_math::is_safe_integer(value)) || (value < (0f64))) {
                    self::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                } else {
                    {
                        let source_ml_source1_temporary_index9_2 = source_ml_source1_temporary_index9.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index9 = source_ml_source1_temporary_index9_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result {
    MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Done(f64, f64),
    MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_plan_rust_borrowed_inherent_position_loop14(mut target_boxes: f64, mut target_references: f64, mut source_ml_source1_temporary_values12: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut source_ml_source1_temporary_index12: self::ml::Big) -> self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result {
    loop {
        return if (source_ml_source1_temporary_index12 < ({ let ml_reference_array75 = source_ml_source1_temporary_values12.clone(); let ml_reference_projection75 = self::ml::Big::from_u128(ml_reference_array75.borrow().len() as u128); ml_reference_projection75 })) {
            {
                let layer = {
                    let source_ml_source2_array_read_value100 = source_ml_source1_temporary_values12.clone();
                    {
                        let source_ml_source2_array_read_index101 = source_ml_source1_temporary_index12.clone();
                        { let ml_reference_array76 = source_ml_source2_array_read_value100.clone(); let ml_reference_position76 = source_ml_source2_array_read_index101.clone(); let ml_reference_projection76 = self::ml_array::at(&*ml_reference_array76.borrow(), ml_reference_position76.to_index()); ml_reference_projection76 }
                    }
                };
                if (layer == "box") {
                    {
                        let target_boxes_2 = (target_boxes + 1f64);
                        {
                            let source_ml_source1_temporary_index12_2 = source_ml_source1_temporary_index12.add(&self::ml::Big::from_i128(1));
                            {
                                (target_boxes, source_ml_source1_temporary_index12) = (target_boxes_2, source_ml_source1_temporary_index12_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    if (layer == "reference") {
                        {
                            let target_references_2 = (target_references + 1f64);
                            {
                                let source_ml_source1_temporary_index12_3 = source_ml_source1_temporary_index12.add(&self::ml::Big::from_i128(1));
                                {
                                    (target_references, source_ml_source1_temporary_index12) = (target_references_2, source_ml_source1_temporary_index12_3.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Done(target_boxes, target_references)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result {
    MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Done(f64, f64),
    MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_plan_rust_borrowed_inherent_position_loop13(mut actual_boxes: f64, mut actual_references: f64, mut source_ml_source1_temporary_values11: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut source_ml_source1_temporary_index11: self::ml::Big) -> self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result {
    loop {
        return if (source_ml_source1_temporary_index11 < ({ let ml_reference_array77 = source_ml_source1_temporary_values11.clone(); let ml_reference_projection77 = self::ml::Big::from_u128(ml_reference_array77.borrow().len() as u128); ml_reference_projection77 })) {
            {
                let layer = {
                    let source_ml_source2_array_read_value102 = source_ml_source1_temporary_values11.clone();
                    {
                        let source_ml_source2_array_read_index103 = source_ml_source1_temporary_index11.clone();
                        { let ml_reference_array78 = source_ml_source2_array_read_value102.clone(); let ml_reference_position78 = source_ml_source2_array_read_index103.clone(); let ml_reference_projection78 = self::ml_array::at(&*ml_reference_array78.borrow(), ml_reference_position78.to_index()); ml_reference_projection78 }
                    }
                };
                if (layer == "box") {
                    {
                        let actual_boxes_2 = (actual_boxes + 1f64);
                        {
                            let source_ml_source1_temporary_index11_2 = source_ml_source1_temporary_index11.add(&self::ml::Big::from_i128(1));
                            {
                                (actual_boxes, source_ml_source1_temporary_index11) = (actual_boxes_2, source_ml_source1_temporary_index11_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    if (layer == "reference") {
                        {
                            let actual_references_2 = (actual_references + 1f64);
                            {
                                let source_ml_source1_temporary_index11_3 = source_ml_source1_temporary_index11.add(&self::ml::Big::from_i128(1));
                                {
                                    (actual_references, source_ml_source1_temporary_index11) = (actual_references_2, source_ml_source1_temporary_index11_3.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Done(actual_boxes, actual_references)
        };
    }
}

fn ml_main() {
    assert!(((self::accept_rust_borrowed_vector_signature_model(String::from("Rust source function signature 3"))) == (true)), "assertion 1");
    assert!(((self::accept_rust_borrowed_vector_signature_model(String::from("Rust source function signature 2"))) == (false)), "assertion 2");
    assert!(((self::read_rust_borrowed_vector_position_error(String::from("Rust source function signature 1"), true)) == "a retained array element position requires the borrowed vector source signature profile"), "assertion 3");
    assert!(((self::read_rust_borrowed_vector_position_error(String::from("Rust source function signature 2"), true)) == "a retained array element position requires the borrowed vector source signature profile"), "assertion 4");
    assert!(((self::read_rust_borrowed_vector_position_error(String::from("Rust source function signature 3"), true)) == ""), "assertion 5");
    assert!(((self::read_rust_borrowed_vector_position_error(String::from("Rust source function signature 2"), false)) == ""), "assertion 6");
    assert!(((self::retain_rust_borrowed_vector_element_position(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])), false)) == (true)), "assertion 7");
    assert!(((self::retain_rust_borrowed_vector_element_position(std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())), true)) == (true)), "assertion 8");
    assert!(((self::retain_rust_borrowed_vector_element_position(std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())), false)) == (false)), "assertion 9");
    assert!(((self::read_rust_borrowed_vector_element_position_error(String::from("array"), String::from("library"), String::from("std::vec::Vec"), 1f64, true, true)) == ""), "assertion 10");
    assert!(((self::read_rust_borrowed_vector_element_position_error(String::from("array"), String::from("declaration"), String::from("std::vec::Vec"), 1f64, true, true)) == "a retained array element position must match its complete canonical vector element type and owner"), "assertion 11");
    assert!(((self::read_rust_borrowed_vector_element_position_error(String::from("array"), String::from("library"), String::from("std::vec::Vec"), 1f64, true, false)) == "a retained array element position must match its complete canonical vector element type and owner"), "assertion 12");
    assert!(((self::accept_rust_borrowed_vector_element(true, 90f64, std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")])), true, false)) == (true)), "assertion 13");
    assert!(((self::accept_rust_borrowed_vector_element(true, 99f64, std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("")])), true, false)) == (true)), "assertion 14");
    assert!(((self::accept_rust_borrowed_vector_element(true, 100f64, std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")])), true, false)) == (false)), "assertion 15");
    assert!(((self::accept_rust_borrowed_vector_element(true, 90f64, std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")])), true, true)) == (false)), "assertion 16");
    assert!(((self::accept_rust_borrowed_vector_element(true, 90f64, std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("box")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")])), true, false)) == (false)), "assertion 17");
    assert!(((self::read_rust_borrowed_vector_position_budget_error(100000f64, 128f64)) == ""), "assertion 18");
    assert!(((self::read_rust_borrowed_vector_position_budget_error(100001f64, 128f64)) == "retained borrowed vector element positions exceed their finite source profile"), "assertion 19");
    assert!(((self::read_rust_borrowed_vector_position_budget_error(f64::NAN, 0f64)) == "retained borrowed vector element positions exceed their finite source profile"), "assertion 20");
    assert!(((self::read_rust_borrowed_vector_position_budget_error(1f64, 1.5f64)) == "retained borrowed vector element positions exceed their finite source profile"), "assertion 21");
    assert!(((self::accept_rust_borrowed_vector_completion_profiles(String::from("Rust source function signature 3"), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from(""), String::from("Rust source function signature 3"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")])))) == (true)), "assertion 22");
    assert!(((self::accept_rust_borrowed_vector_completion_profiles(String::from("Rust source function signature 3"), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from(""), String::from("Rust source function signature 3"), String::from("other"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")])))) == (false)), "assertion 23");
    assert!(((self::accept_rust_borrowed_vector_source(String::from("Rust source function signature 3"), true, 90f64)) == (true)), "assertion 24");
    assert!(((self::accept_rust_borrowed_vector_source(String::from("Rust source function signature 3"), true, 99f64)) == (true)), "assertion 25");
    assert!(((self::accept_rust_borrowed_vector_source(String::from("Rust source function signature 3"), true, 100f64)) == (false)), "assertion 26");
    assert!(((self::accept_rust_borrowed_vector_owned_interpretation(false)) == (true)), "assertion 27");
    assert!(((self::accept_rust_borrowed_vector_owned_interpretation(true)) == (false)), "assertion 28");
    assert!(((self::plan_rust_borrowed_vector_result_parameters(String::from("a"), std::rc::Rc::new(std::cell::RefCell::new(vec![true, false, true])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a"), String::from(""), String::from("a")])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 2f64])))), "assertion 29");
    assert!(((self::plan_rust_borrowed_vector_result_parameters(String::from(""), std::rc::Rc::new(std::cell::RefCell::new(vec![true, false])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from(""), String::from("")])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])))), "assertion 30");
    assert!(((self::plan_rust_borrowed_vector_result_parameters(String::from(""), std::rc::Rc::new(std::cell::RefCell::new(vec![true, true])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from(""), String::from("")])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)])))), "assertion 31");
    assert!(((self::plan_rust_borrowed_vector_result_parameters(String::from("static"), std::rc::Rc::new(std::cell::RefCell::new(vec![true, true])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("static"), String::from("a")])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])))), "assertion 32");
    assert!(((self::read_rust_borrowed_vector_argument_error(true, true, true, true, String::from("a"), String::from("static"))) == "borrowed vector arguments must retain their actual reference element owner and region contract"), "assertion 33");
    assert!(((self::read_rust_borrowed_vector_argument_error(true, false, true, true, String::from("a"), String::from(""))) == "borrowed vector arguments must retain their actual reference element owner and region contract"), "assertion 34");
    assert!(((self::read_rust_borrowed_vector_argument_error(true, true, true, true, String::from("a"), String::from("b"))) == ""), "assertion 35");
    assert!(((self::accept_rust_borrowed_vector_literal_context(true, true, false, 0f64)) == (true)), "assertion 36");
    assert!(((self::accept_rust_borrowed_vector_literal_context(true, true, true, 2f64)) == (true)), "assertion 37");
    assert!(((self::accept_rust_borrowed_vector_literal_context(true, true, false, 1f64)) == (false)), "assertion 38");
    assert!(((self::accept_rust_borrowed_vector_literal_context(true, false, false, 0f64)) == (false)), "assertion 39");
    assert!(((self::accept_rust_borrowed_vector_literal_context(false, true, false, 0f64)) == (false)), "assertion 40");
    assert!(((self::accept_rust_borrowed_vector_literal_context(true, true, true, 65537f64)) == (false)), "assertion 41");
    assert!(((self::read_rust_borrowed_vector_empty_constructor(true, String::from("Vec"), String::from("new"), 0f64)) == "construct"), "assertion 42");
    assert!(((self::read_rust_borrowed_vector_empty_constructor(true, String::from("std::vec::Vec"), String::from("new"), 0f64)) == "construct"), "assertion 43");
    assert!(((self::read_rust_borrowed_vector_empty_constructor(true, String::from("std::vec::Vec"), String::from("new"), 1f64)) == "arity"), "assertion 44");
    assert!(((self::read_rust_borrowed_vector_empty_constructor(false, String::from("Vec"), String::from("new"), 0f64)) == ""), "assertion 45");
    assert!(((self::read_rust_borrowed_vector_empty_constructor(true, String::from("my::Vec"), String::from("new"), 0f64)) == ""), "assertion 46");
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
fn all_literal_borrowed_vector_position_controls() { main(); }
