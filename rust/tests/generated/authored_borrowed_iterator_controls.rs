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

pub fn render_primitive_string_representation(units: bool, form: String, value: String, other: String, operation: String) -> String {
    if (form == "type") {
        if units {
            String::from("crate::ml_string::Text")
        } else {
            String::from("String")
        }
    } else {
        if (form == "literal") {
            if units {
                format!("{}{}", (format!("{}{}", (String::from("crate::ml_string::Text::from_units(vec![")), value)), (String::from("])")))
            } else {
                format!("{}{}", (format!("{}{}", (String::from("String::from(")), value)), (String::from(")")))
            }
        } else {
            if (form == "constant") {
                if units {
                    format!("{}{}", (format!("{}{}", (String::from("crate::ml_string::Text::from_text(")), value)), (String::from(")")))
                } else {
                    format!("{}{}", (format!("{}{}", (String::from("String::from(")), value)), (String::from(")")))
                }
            } else {
                if (form == "examples") {
                    if units {
                        String::from("vec![crate::ml_string::Text::from_text(\"\"), crate::ml_string::Text::from_text(\"a\"), crate::ml_string::Text::from_text(\"ab\")]")
                    } else {
                        String::from("vec![String::new(), String::from(\"a\"), String::from(\"ab\")]")
                    }
                } else {
                    if (form == "text") {
                        if units {
                            format!("{}{}", (format!("{}{}", (String::from("crate::ml_string::Text::from_text(&")), value)), (String::from(")")))
                        } else {
                            value.clone()
                        }
                    } else {
                        if (form == "concat") {
                            if units {
                                format!("{}{}", (format!("{}{}", (format!("{}{}", value, (String::from(".concat(")))), other)), (String::from(")")))
                            } else {
                                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("format!(\"{}{}\", ")), value)), (String::from(", ")))), other)), (String::from(")")))
                            }
                        } else {
                            if !units {
                                String::from("")
                            } else {
                                if (form == "length") {
                                    format!("{}{}", value, (String::from(".length()")))
                                } else {
                                    if (form == "test") {
                                        {
                                            let method = if (operation == "startsWith") {
                                                String::from("starts_with")
                                            } else {
                                                if (operation == "endsWith") {
                                                    String::from("ends_with")
                                                } else {
                                                    String::from("contains")
                                                }
                                            };
                                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", value, (String::from(".")))), method)), (String::from("(")))), other)), (String::from(")")))
                                        }
                                    } else {
                                        if (form == "map") {
                                            if ((operation == "toLowerCase") || (operation == "toUpperCase")) {
                                                format!("{}{}", (format!("{}{}", (format!("{}{}", value, (String::from(".map_case(")))), (if (operation == "toUpperCase") {
                                                    String::from("true")
                                                } else {
                                                    String::from("false")
                                                }))), (String::from(")")))
                                            } else {
                                                {
                                                    let start = ((operation != "trimEnd") && (operation != "rustTrimEnd"));
                                                    {
                                                        let end = ((operation != "trimStart") && (operation != "rustTrimStart"));
                                                        {
                                                            let rust = operation.starts_with("rust");
                                                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", value, (String::from(".trim(")))), (if start {
                                                                String::from("true")
                                                            } else {
                                                                String::from("false")
                                                            }))), (String::from(", ")))), (if end {
                                                                String::from("true")
                                                            } else {
                                                                String::from("false")
                                                            }))), (String::from(", ")))), (if rust {
                                                                String::from("true")
                                                            } else {
                                                                String::from("false")
                                                            }))), (String::from(")")))
                                                        }
                                                    }
                                                }
                                            }
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

pub fn read_primitive_copy_slice_form(kind: String) -> String {
    if ((kind == "array") || (kind == "string")) {
        String::from("identity")
    } else {
        if (kind == "record") {
            String::from("method")
        } else {
            String::from("refuse")
        }
    }
}

pub fn read_primitive_method_minimum_arguments(receiver: String, operation: String) -> f64 {
    if ((receiver == "array") && (((operation == "slice") || (operation == "join")) || (operation == "at"))) {
        0f64
    } else {
        if ((receiver == "array") && (((operation == "includes") || (operation == "indexOf")) || (operation == "lastIndexOf"))) {
            1f64
        } else {
            if (receiver != "string") {
                (-1f64)
            } else {
                if (((((operation == "split") || (operation == "slice")) || (operation == "charCodeAt")) || (operation == "charAt")) || (operation == "at")) {
                    0f64
                } else {
                    if ((((((operation == "indexOf") || (operation == "lastIndexOf")) || (operation == "repeat")) || (operation == "padStart")) || (operation == "padEnd")) || (operation == "codePointAt")) {
                        1f64
                    } else {
                        if ((operation == "replace") || (operation == "replaceAll")) {
                            2f64
                        } else {
                            (-1f64)
                        }
                    }
                }
            }
        }
    }
}

pub fn read_primitive_method_maximum_arguments(receiver: String, operation: String) -> f64 {
    if (receiver == "array") {
        if ((operation == "join") || (operation == "at")) {
            1f64
        } else {
            if ((((operation == "slice") || (operation == "includes")) || (operation == "indexOf")) || (operation == "lastIndexOf")) {
                2f64
            } else {
                (-1f64)
            }
        }
    } else {
        if (receiver != "string") {
            (-1f64)
        } else {
            if (((((operation == "repeat") || (operation == "codePointAt")) || (operation == "charCodeAt")) || (operation == "charAt")) || (operation == "at")) {
                1f64
            } else {
                if ((((((((operation == "split") || (operation == "slice")) || (operation == "indexOf")) || (operation == "lastIndexOf")) || (operation == "replace")) || (operation == "replaceAll")) || (operation == "padStart")) || (operation == "padEnd")) {
                    2f64
                } else {
                    (-1f64)
                }
            }
        }
    }
}

pub fn read_primitive_method_argument_count(receiver: String, operation: String) -> f64 {
    self::read_primitive_method_maximum_arguments(receiver.clone(), operation.clone())
}

pub fn read_primitive_method_checked_argument_kind(receiver: String, operation: String, index: f64) -> String {
    if ((receiver == "array") && (operation == "at")) {
        String::from("number-value")
    } else {
        if ((receiver == "array") && (operation == "join")) {
            String::from("string")
        } else {
            if ((receiver == "array") && (operation == "slice")) {
                String::from("float")
            } else {
                if ((receiver == "array") && (((operation == "includes") || (operation == "indexOf")) || (operation == "lastIndexOf"))) {
                    if (index == (0f64)) {
                        String::from("element")
                    } else {
                        String::from("float")
                    }
                } else {
                    if (receiver != "string") {
                        String::from("")
                    } else {
                        if (((operation == "charCodeAt") || (operation == "charAt")) || (operation == "at")) {
                            String::from("number-value")
                        } else {
                            if ((operation == "replace") || (operation == "replaceAll")) {
                                String::from("string")
                            } else {
                                if ((index == (0f64)) && (((operation == "split") || (operation == "indexOf")) || (operation == "lastIndexOf"))) {
                                    String::from("string")
                                } else {
                                    if ((index == (1f64)) && ((operation == "padStart") || (operation == "padEnd"))) {
                                        String::from("string")
                                    } else {
                                        String::from("float")
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

pub fn read_primitive_method_default_form(receiver: String, operation: String, index: f64) -> String {
    if ((receiver == "array") && (operation == "at")) {
        String::from("float")
    } else {
        if ((receiver == "array") && (operation == "join")) {
            String::from("string")
        } else {
            if (((receiver == "array") && (((operation == "includes") || (operation == "indexOf")) || (operation == "lastIndexOf"))) && (index == (1f64))) {
                String::from("float")
            } else {
                if (((receiver == "string") && (operation == "split")) && (index == (0f64))) {
                    String::from("absent")
                } else {
                    if (((receiver == "string") && (operation == "split")) && (index == (1f64))) {
                        String::from("float")
                    } else {
                        if (((receiver == "string") && (((operation == "charCodeAt") || (operation == "charAt")) || (operation == "at"))) && (index == (0f64))) {
                            String::from("float")
                        } else {
                            if (((receiver == "string") || (receiver == "array")) && (operation == "slice")) {
                                String::from("float")
                            } else {
                                if (((receiver == "string") && ((operation == "indexOf") || (operation == "lastIndexOf"))) && (index == (1f64))) {
                                    String::from("float")
                                } else {
                                    if (((receiver == "string") && ((operation == "padStart") || (operation == "padEnd"))) && (index == (1f64))) {
                                        String::from("string")
                                    } else {
                                        String::from("required")
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

pub fn read_primitive_method_default_value(receiver: String, operation: String, index: f64) -> String {
    if ((receiver == "array") && (operation == "at")) {
        String::from("0")
    } else {
        if ((receiver == "array") && (operation == "join")) {
            String::from(",")
        } else {
            if (((receiver == "array") && ((operation == "includes") || (operation == "indexOf"))) && (index == (1f64))) {
                String::from("0")
            } else {
                if (((receiver == "array") && (operation == "lastIndexOf")) && (index == (1f64))) {
                    String::from("Infinity")
                } else {
                    if (((receiver == "string") && (operation == "split")) && (index == (1f64))) {
                        String::from("4294967295")
                    } else {
                        if (((receiver == "string") && (((operation == "charCodeAt") || (operation == "charAt")) || (operation == "at"))) && (index == (0f64))) {
                            String::from("0")
                        } else {
                            if (((receiver == "string") || (receiver == "array")) && (operation == "slice")) {
                                if (index == (0f64)) {
                                    String::from("0")
                                } else {
                                    String::from("Infinity")
                                }
                            } else {
                                if (((receiver == "string") && (operation == "indexOf")) && (index == (1f64))) {
                                    String::from("0")
                                } else {
                                    if (((receiver == "string") && (operation == "lastIndexOf")) && (index == (1f64))) {
                                        String::from("Infinity")
                                    } else {
                                        if (((receiver == "string") && ((operation == "padStart") || (operation == "padEnd"))) && (index == (1f64))) {
                                            String::from(" ")
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

pub fn read_primitive_method_checked_result_form(receiver: String, operation: String) -> String {
    if ((receiver == "array") && (operation == "at")) {
        String::from("optional-element")
    } else {
        if ((receiver == "array") && (operation == "join")) {
            String::from("string")
        } else {
            if ((receiver == "array") && (operation == "slice")) {
                String::from("receiver")
            } else {
                if ((receiver == "array") && (operation == "includes")) {
                    String::from("bool")
                } else {
                    if ((receiver == "array") && ((operation == "indexOf") || (operation == "lastIndexOf"))) {
                        String::from("float")
                    } else {
                        if (receiver != "string") {
                            String::from("")
                        } else {
                            if (operation == "split") {
                                String::from("array-string")
                            } else {
                                if (((operation == "indexOf") || (operation == "lastIndexOf")) || (operation == "charCodeAt")) {
                                    String::from("float")
                                } else {
                                    if (operation == "codePointAt") {
                                        String::from("optional-float")
                                    } else {
                                        if (operation == "at") {
                                            String::from("optional-string")
                                        } else {
                                            String::from("string")
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

pub fn read_primitive_method_argument_obligation(receiver: String, operation: String, element: String) -> String {
    if ((receiver == "array") && (operation == "join")) {
        {
            let form = self::read_primitive_array_join_element_form(element.clone());
            if ((form == "refuse") || (form == "optional")) {
                String::from("joining structured values needs their JavaScript string conversion and cycle semantics")
            } else {
                String::from("")
            }
        }
    } else {
        if ((receiver != "array") || (((operation != "includes") && (operation != "indexOf")) && (operation != "lastIndexOf"))) {
            String::from("")
        } else {
            if ((((((element == "array") || (element == "record")) || (element == "map")) || (element == "set")) || (element == "data")) || (element == "function")) {
                String::from("object identity must be represented before searching arrays of objects")
            } else {
                String::from("")
            }
        }
    }
}

pub fn render_primitive_array_search_comparator(operation: String, element: String, absence: String) -> String {
    {
        let present = if ((operation == "includes") && (element == "float")) {
            String::from("a == b || (a.is_nan() && b.is_nan())")
        } else {
            String::from("a == b")
        };
        if (absence == "both") {
            format!("{}{}", (format!("{}{}", (String::from("|a, b| match (a, b) { (crate::MlOptional::Value(a), crate::MlOptional::Value(b)) => ")), present)), (String::from(", (crate::MlOptional::Null, crate::MlOptional::Null) | (crate::MlOptional::Undefined, crate::MlOptional::Undefined) => true, _ => false }")))
        } else {
            if ((absence == "null") || (absence == "undefined")) {
                format!("{}{}", (format!("{}{}", (String::from("|a, b| match (a, b) { (Some(a), Some(b)) => ")), present)), (String::from(", (None, None) => true, _ => false }")))
            } else {
                if ((operation == "includes") && (element == "float")) {
                    format!("{}{}", (String::from("|a: &f64, b: &f64| ")), present)
                } else {
                    format!("{}{}", (String::from("|a, b| ")), present)
                }
            }
        }
    }
}

pub fn render_primitive_array_search_expression(operation: String, object: String, search: String, from: String, element: String, absence: String) -> String {
    {
        let comparator = self::render_primitive_array_search_comparator(operation.clone(), element.clone(), absence.clone());
        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_array::")), (if (operation == "includes") {
            String::from("includes")
        } else {
            String::from("index_of")
        }))), (String::from("(")))), object)), (String::from(", &")))), search)), (String::from(", ")))), from)), (String::from(", ")))), (if (operation == "lastIndexOf") {
            String::from("true")
        } else {
            String::from("false")
        }))), (String::from(", ")))), comparator)), (String::from(")")))
    }
}

pub fn read_primitive_method_rendering_refusal(target: String, receiver: String, operation: String) -> String {
    if ((target != "Lean") && (target != "Rocq")) {
        String::from("")
    } else {
        if (receiver != "array") {
            String::from("this target needs a UTF-16 operational string encoding for this operation")
        } else {
            if ((((operation == "slice") || (operation == "includes")) || (operation == "indexOf")) || (operation == "lastIndexOf")) {
                format!("{}{}", (String::from("this array operation needs an operational encoding in ")), target)
            } else {
                String::from("")
            }
        }
    }
}

pub fn render_primitive_method_argument(target: String, receiver: String, operation: String, index: f64, absent: bool, text: String) -> String {
    if ((target == "JavaScript") && absent) {
        String::from("undefined")
    } else {
        if ((((target == "Rust") && (receiver == "string")) && (operation == "split")) && (index == (0f64))) {
            if absent {
                String::from("None")
            } else {
                format!("{}{}", (format!("{}{}", (String::from("Some(")), text)), (String::from(")")))
            }
        } else {
            text.clone()
        }
    }
}

pub fn render_primitive_method_expression(target: String, receiver: String, operation: String, object: String, arguments_text: String) -> String {
    if (target == "JavaScript") {
        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), object)), (String::from(").")))), operation)), (String::from("(")))), arguments_text)), (String::from(")")))
    } else {
        if (((target == "Rust") && (receiver == "array")) && (operation == "slice")) {
            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_array::slice(")), object)), (String::from(", ")))), arguments_text)), (String::from(")")))
        } else {
            if ((target != "Rust") || (receiver != "string")) {
                String::from("")
            } else {
                if (operation == "indexOf") {
                    format!("{}{}", (format!("{}{}", (format!("{}{}", object, (String::from(".index_of(")))), arguments_text)), (String::from(")")))
                } else {
                    if (operation == "lastIndexOf") {
                        format!("{}{}", (format!("{}{}", (format!("{}{}", object, (String::from(".last_index_of(")))), arguments_text)), (String::from(")")))
                    } else {
                        if (operation == "codePointAt") {
                            format!("{}{}", (format!("{}{}", (format!("{}{}", object, (String::from(".code_point_at(")))), arguments_text)), (String::from(")")))
                        } else {
                            if ((operation == "replace") || (operation == "replaceAll")) {
                                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", object, (String::from(".replace(")))), arguments_text)), (String::from(", ")))), (if (operation == "replaceAll") {
                                    String::from("true")
                                } else {
                                    String::from("false")
                                }))), (String::from(")")))
                            } else {
                                if ((operation == "padStart") || (operation == "padEnd")) {
                                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", object, (String::from(".pad(")))), arguments_text)), (String::from(", ")))), (if (operation == "padStart") {
                                        String::from("true")
                                    } else {
                                        String::from("false")
                                    }))), (String::from(")")))
                                } else {
                                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", object, (String::from(".")))), operation)), (String::from("(")))), arguments_text)), (String::from(")")))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn require_primitive_method_utf16(expression: String, receiver: String) -> bool {
    ((expression == "method") && (receiver == "string"))
}

pub fn read_primitive_method_lowering_form(receiver: String, operation: String) -> String {
    if ((receiver == "array") && (operation == "at")) {
        String::from("relative-element-template")
    } else {
        if ((receiver == "string") && (operation == "at")) {
            String::from("relative-character-template")
        } else {
            if ((receiver == "string") && (operation == "charAt")) {
                String::from("character-template")
            } else {
                if ((receiver == "string") && (operation == "charCodeAt")) {
                    String::from("code-unit-template")
                } else {
                    String::from("method")
                }
            }
        }
    }
}

pub fn read_primitive_array_join_element_form(kind: String) -> String {
    if (kind == "string") {
        String::from("string")
    } else {
        if (kind == "unit") {
            String::from("empty")
        } else {
            if (kind == "option") {
                String::from("optional")
            } else {
                if (((((kind == "bool") || (kind == "float")) || (kind == "fixed")) || (kind == "int")) || (kind == "nat")) {
                    String::from("text")
                } else {
                    String::from("refuse")
                }
            }
        }
    }
}

pub fn read_primitive_method_argument_obligation_feature(operation: String) -> String {
    if (operation == "join") {
        String::from("array join")
    } else {
        String::from("array identity search")
    }
}

pub fn read_primitive_array_join_operand_form(units: bool, string_element: bool, operand: String) -> String {
    if (units && ((operand == "separator") || string_element)) {
        String::from("borrow")
    } else {
        String::from("receiver")
    }
}

pub fn render_primitive_array_join_element(kind: String, absence: String, name: String, present: String, empty: String) -> String {
    {
        let form = self::read_primitive_array_join_element_form(kind.clone());
        if (form == "empty") {
            empty.clone()
        } else {
            if (form == "optional") {
                if (absence == "both") {
                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("match ")), name)), (String::from(" { crate::MlOptional::Value(ml_present) => ")))), present)), (String::from(", _ => ")))), empty)), (String::from(" }")))
                } else {
                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("match ")), name)), (String::from(" { Some(ml_present) => ")))), present)), (String::from(", None => ")))), empty)), (String::from(" }")))
                }
            } else {
                present.clone()
            }
        }
    }
}

pub fn render_primitive_array_join_expression(units: bool, string_element: bool, object: String, separator: String, element_text: String) -> String {
    if string_element {
        if units {
            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_string::join(")), object)), (String::from(", ")))), separator)), (String::from(")")))
        } else {
            format!("{}{}", (format!("{}{}", (format!("{}{}", object, (String::from(".join(")))), separator)), (String::from(".as_str())")))
        }
    } else {
        {
            let values = format!("{}{}", (format!("{}{}", (format!("{}{}", object, (String::from(".iter().cloned().map(|ml_value| ")))), element_text)), (String::from(").collect::<Vec<_>>()")));
            if units {
                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_string::join(&")), values)), (String::from(", ")))), separator)), (String::from(")")))
            } else {
                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), values)), (String::from(").join(")))), separator)), (String::from(".as_str())")))
            }
        }
    }
}

pub fn accept_rust_borrowed_iterator_receiver(selected: bool, method: String, count: f64, kind: String, owner: String, path: String, arguments_count: f64, closed: bool) -> bool {
    (((((((selected && (method == "iter")) && (count == (0f64))) && (kind == "array")) && (owner == "library")) && (path == "std::vec::Vec")) && (arguments_count == (1f64))) && closed)
}

pub fn read_rust_borrowed_iterator_continuation(operation: String) -> String {
    if (operation == "map") {
        String::from("map")
    } else {
        if (operation == "chain") {
            String::from("chain")
        } else {
            if (operation == "collect") {
                String::from("collect")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_rust_borrowed_iterator_projection_syntax(kind: String) -> String {
    if (kind == "name") {
        String::from("name")
    } else {
        if (kind == "match") {
            String::from("match")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_borrowed_iterator_callback_error(selected: bool, parameters: f64, immutable: bool, reference: bool, closed: bool, captures: f64, body_projection: bool) -> String {
    if ((((((selected && (parameters == (1f64))) && immutable) && reference) && closed) && (captures == (0f64))) && body_projection) {
        String::from("")
    } else {
        String::from("a readonly iterator callback needs one actual immutable reference parameter, a closed reference result and a capture-free parameter projection")
    }
}

pub fn project_rust_borrowed_iterator_callback_regions(loans: Vec<String>, existing: Vec<String>, local: Vec<String>, receiver: Vec<String>) -> Vec<String> {
    {
        let result = Vec::<String>::new();
        {
            let source_ml_source1_temporary_values1 = loans;
            {
                let source_ml_source1_temporary_index1 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s1 = self::source_ml_source1_temporary_project_rust_borrowed_iterator_callback_regions_loop3(existing, local, receiver, result, source_ml_source1_temporary_values1, source_ml_source1_temporary_index1.clone());
                    match source_ml_source2_s1.clone() {
                        self::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Done(result_2) => {
                            result_2
                        }
                        self::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Return(source_ml_source1_temporary_result3) => {
                            source_ml_source1_temporary_result3
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_borrowed_iterator_region_error(actual: f64, projected: f64, collecting: bool) -> String {
    if (!collecting && ((actual == (0f64)) || (projected == (0f64)))) {
        String::from("a readonly iterator value needs its actual live receiver-derived regions")
    } else {
        String::from("")
    }
}

pub fn accept_rust_borrowed_iterator_once(path: String, shadowed: bool, arguments_count: f64) -> bool {
    (((path == "std::iter::once") && !shadowed) && (arguments_count == (1f64)))
}

pub fn accept_rust_borrowed_iterator_box_reference(selected: bool, canonical: bool, arguments_count: f64, reference: bool, live: bool, collecting: bool) -> bool {
    ((((selected && canonical) && (arguments_count == (0f64))) && reference) && (live || collecting))
}

pub fn read_rust_borrowed_iterator_budget_error(stages: f64, nodes: f64, regions: f64) -> String {
    {
        let source_ml_source1_temporary_values5 = vec![stages, nodes, regions];
        {
            let source_ml_source1_temporary_index5 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s2 = self::source_ml_source1_temporary_read_rust_borrowed_iterator_budget_error_loop6(source_ml_source1_temporary_values5, source_ml_source1_temporary_index5.clone());
                match source_ml_source2_s2.clone() {
                    self::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Done => {
                        if (((stages > (64f64)) || (nodes > (100000f64))) || (regions > (65536f64))) {
                            String::from("the readonly iterator source pipeline exceeds its finite profile")
                        } else {
                            String::from("")
                        }
                    }
                    self::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Return(source_ml_source1_temporary_result6) => {
                        source_ml_source1_temporary_result6.clone()
                    }
                }
            }
        }
    }
}

pub fn read_rust_borrowed_iterator_collection_error(type_: bool, owner: bool, reference: bool, context: bool) -> String {
    if (((type_ && owner) && reference) && context) {
        String::from("")
    } else {
        String::from("readonly collection needs its actual matching borrowed vector result position")
    }
}

pub fn accept_rust_borrowed_iterator_production(operation: String, source: bool, result: bool, projection: bool) -> bool {
    (((self::ml_array::includes(&vec![String::from("slice"), String::from("map"), String::from("concat")], &operation.clone(), 0f64, false, |a, b| a == b) && source) && result) && projection)
}

pub fn accept_rust_borrowed_iterator_whole_slice(kinds: Vec<String>, types: Vec<String>, values: Vec<String>) -> bool {
    (((((((((((kinds.len() as f64)) == (2f64)) && (((types.len() as f64)) == (2f64))) && (((values.len() as f64)) == (2f64))) && {
        let source_ml_source2_comparison_operand3 = self::ml_array::optional_at(&kinds, self::ml_array::number_index(0f64));
        {
            let source_ml_source2_comparison_operand4 = String::from("lit");
            match (source_ml_source2_comparison_operand3.clone(), source_ml_source2_comparison_operand4.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand5 = self::ml_array::optional_at(&kinds, self::ml_array::number_index(1f64));
        {
            let source_ml_source2_comparison_operand6 = String::from("lit");
            match (source_ml_source2_comparison_operand5.clone(), source_ml_source2_comparison_operand6.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand7 = self::ml_array::optional_at(&types, self::ml_array::number_index(0f64));
        {
            let source_ml_source2_comparison_operand8 = String::from("float");
            match (source_ml_source2_comparison_operand7.clone(), source_ml_source2_comparison_operand8.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand9 = self::ml_array::optional_at(&types, self::ml_array::number_index(1f64));
        {
            let source_ml_source2_comparison_operand10 = String::from("float");
            match (source_ml_source2_comparison_operand9.clone(), source_ml_source2_comparison_operand10.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand11 = self::ml_array::optional_at(&values, self::ml_array::number_index(0f64));
        {
            let source_ml_source2_comparison_operand12 = self::read_primitive_method_default_value(String::from("array"), String::from("slice"), 0f64);
            match (source_ml_source2_comparison_operand11.clone(), source_ml_source2_comparison_operand12.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    }) && {
        let source_ml_source2_comparison_operand13 = self::ml_array::optional_at(&values, self::ml_array::number_index(1f64));
        {
            let source_ml_source2_comparison_operand14 = self::read_primitive_method_default_value(String::from("array"), String::from("slice"), 1f64);
            match (source_ml_source2_comparison_operand13.clone(), source_ml_source2_comparison_operand14.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    })
}

pub fn read_rust_borrowed_iterator_snapshot_error(count: f64, text: f64) -> String {
    if (((((((!(self::ml_math::is_safe_integer(count)) || !(self::ml_math::is_safe_integer(text))) || (count < (0f64))) || (text < (0f64))) || {
        let source_ml_source2_same_value_operand15 = count;
        {
            let source_ml_source2_same_value_operand16 = (-0f64);
            (((source_ml_source2_same_value_operand15 == source_ml_source2_same_value_operand16) && ((source_ml_source2_same_value_operand15 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand15)) == ((1f64 / source_ml_source2_same_value_operand16))))) || ((source_ml_source2_same_value_operand15 != source_ml_source2_same_value_operand15) && (source_ml_source2_same_value_operand16 != source_ml_source2_same_value_operand16)))
        }
    }) || {
        let source_ml_source2_same_value_operand17 = text;
        {
            let source_ml_source2_same_value_operand18 = (-0f64);
            (((source_ml_source2_same_value_operand17 == source_ml_source2_same_value_operand18) && ((source_ml_source2_same_value_operand17 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand17)) == ((1f64 / source_ml_source2_same_value_operand18))))) || ((source_ml_source2_same_value_operand17 != source_ml_source2_same_value_operand17) && (source_ml_source2_same_value_operand18 != source_ml_source2_same_value_operand18)))
        }
    }) || (count > (100000f64))) || (text > (16777216f64))) {
        String::from("readonly callback ownership snapshots exceed their finite source profile")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_iterator_parameter_error(name: String, raw_: String) -> String {
    if raw_.starts_with("r#") {
        String::from("")
    } else {
        if self::ml_array::includes(&vec![String::from("_"), String::from("as"), String::from("break"), String::from("const"), String::from("continue"), String::from("crate"), String::from("else"), String::from("enum"), String::from("extern"), String::from("false"), String::from("fn"), String::from("for"), String::from("if"), String::from("impl"), String::from("in"), String::from("let"), String::from("loop"), String::from("match"), String::from("mod"), String::from("move"), String::from("mut"), String::from("pub"), String::from("ref"), String::from("return"), String::from("self"), String::from("Self"), String::from("static"), String::from("struct"), String::from("super"), String::from("trait"), String::from("true"), String::from("type"), String::from("unsafe"), String::from("use"), String::from("where"), String::from("while"), String::from("async"), String::from("await"), String::from("dyn"), String::from("abstract"), String::from("become"), String::from("box"), String::from("do"), String::from("final"), String::from("gen"), String::from("macro"), String::from("override"), String::from("priv"), String::from("try"), String::from("typeof"), String::from("unsized"), String::from("virtual"), String::from("yield")], &name.clone(), 0f64, false, |a, b| a == b) {
            String::from("the readonly callback needs a fresh direct immutable binding identifier")
        } else {
            String::from("")
        }
    }
}

pub fn read_rust_borrowed_iterator_binding_owner_error(value_kind: String, declared_type: bool) -> String {
    if (((value_kind == "const") || (value_kind == "ctor")) || declared_type) {
        String::from("readonly callback parameters require a fresh binder rather than a constructor or constant pattern")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_iterator_region_work_error(count: f64) -> String {
    if (((!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) || {
        let source_ml_source2_same_value_operand19 = count;
        {
            let source_ml_source2_same_value_operand20 = (-0f64);
            (((source_ml_source2_same_value_operand19 == source_ml_source2_same_value_operand20) && ((source_ml_source2_same_value_operand19 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand19)) == ((1f64 / source_ml_source2_same_value_operand20))))) || ((source_ml_source2_same_value_operand19 != source_ml_source2_same_value_operand19) && (source_ml_source2_same_value_operand20 != source_ml_source2_same_value_operand20)))
        }
    }) || (count > (256f64))) {
        String::from("readonly iterator region union exceeds its bounded source work profile")
    } else {
        String::from("")
    }
}

pub fn read_rust_borrowed_iterator_box_reference_error(canonical: bool, count: f64, admitted: bool, projection: bool) -> String {
    if !canonical {
        String::from("")
    } else {
        if (count != (0f64)) {
            String::from("canonical Box::as_ref expects zero arguments")
        } else {
            if (!admitted || !projection) {
                String::from("canonical Box::as_ref requires its actual live readonly receiver projection")
            } else {
                String::from("")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result {
    MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Done(Vec<String>),
    MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Return(Vec<String>),
}

pub fn source_ml_source1_temporary_project_rust_borrowed_iterator_callback_regions_loop4(mut result: Vec<String>, mut source_ml_source1_temporary_values2: Vec<String>, mut source_ml_source1_temporary_index2: self::ml::Big) -> Vec<String> {
    loop {
        return if (source_ml_source1_temporary_index2 < (self::ml::Big::from_u128(source_ml_source1_temporary_values2.len() as u128))) {
            {
                let root = self::ml_array::at(&source_ml_source1_temporary_values2, source_ml_source1_temporary_index2.to_index());
                if !(self::ml_array::includes(&result, &root.clone(), 0f64, false, |a, b| a == b)) {
                    {
                        let result_2 = { let (ml_append_right, ml_append_left) = (vec![root.clone()], result); self::ml_array::append(ml_append_left, ml_append_right) };
                        {
                            let source_ml_source1_temporary_index2_2 = source_ml_source1_temporary_index2.add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_source1_temporary_index2) = (result_2, source_ml_source1_temporary_index2_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index2_3 = source_ml_source1_temporary_index2.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index2 = source_ml_source1_temporary_index2_3.clone();
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

pub fn source_ml_source1_temporary_project_rust_borrowed_iterator_callback_regions_loop3(mut existing: Vec<String>, mut local: Vec<String>, mut receiver: Vec<String>, mut result: Vec<String>, mut source_ml_source1_temporary_values1: Vec<String>, mut source_ml_source1_temporary_index1: self::ml::Big) -> self::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result {
    loop {
        return if (source_ml_source1_temporary_index1 < (self::ml::Big::from_u128(source_ml_source1_temporary_values1.len() as u128))) {
            {
                let loan = self::ml_array::at(&source_ml_source1_temporary_values1, source_ml_source1_temporary_index1.to_index());
                if self::ml_array::includes(&receiver, &loan.clone(), 0f64, false, |a, b| a == b) {
                    if !(self::ml_array::includes(&result, &loan.clone(), 0f64, false, |a, b| a == b)) {
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![loan.clone()], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_source1_temporary_index1_2 = source_ml_source1_temporary_index1.add(&self::ml::Big::from_i128(1));
                                {
                                    (result, source_ml_source1_temporary_index1) = (result_2, source_ml_source1_temporary_index1_2.clone());
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
                } else {
                    if (self::ml_array::includes(&local, &loan.clone(), 0f64, false, |a, b| a == b) && !(self::ml_array::includes(&existing, &loan.clone(), 0f64, false, |a, b| a == b))) {
                        {
                            let source_ml_source1_temporary_values2 = receiver.clone();
                            {
                                let source_ml_source1_temporary_index2 = self::ml::Big::from_i128(0);
                                {
                                    let result_3 = self::source_ml_source1_temporary_project_rust_borrowed_iterator_callback_regions_loop4(result, source_ml_source1_temporary_values2, source_ml_source1_temporary_index2.clone());
                                    {
                                        let source_ml_source1_temporary_index1_4 = source_ml_source1_temporary_index1.add(&self::ml::Big::from_i128(1));
                                        {
                                            (result, source_ml_source1_temporary_index1) = (result_3, source_ml_source1_temporary_index1_4.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        self::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Return(Vec::<String>::new())
                    }
                }
            }
        } else {
            self::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result {
    MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Done,
    MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Return(String),
}

pub fn source_ml_source1_temporary_read_rust_borrowed_iterator_budget_error_loop6(mut source_ml_source1_temporary_values5: Vec<f64>, mut source_ml_source1_temporary_index5: self::ml::Big) -> self::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result {
    loop {
        return if (source_ml_source1_temporary_index5 < (self::ml::Big::from_u128(source_ml_source1_temporary_values5.len() as u128))) {
            {
                let count = self::ml_array::at(&source_ml_source1_temporary_values5, source_ml_source1_temporary_index5.to_index());
                if ((!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) || {
                    let source_ml_source2_same_value_operand21 = count;
                    {
                        let source_ml_source2_same_value_operand22 = (-0f64);
                        (((source_ml_source2_same_value_operand21 == source_ml_source2_same_value_operand22) && ((source_ml_source2_same_value_operand21 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand21)) == ((1f64 / source_ml_source2_same_value_operand22))))) || ((source_ml_source2_same_value_operand21 != source_ml_source2_same_value_operand21) && (source_ml_source2_same_value_operand22 != source_ml_source2_same_value_operand22)))
                    }
                }) {
                    self::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Return(String::from("invalid readonly iterator source resource count"))
                } else {
                    {
                        let source_ml_source1_temporary_index5_2 = source_ml_source1_temporary_index5.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index5 = source_ml_source1_temporary_index5_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Done
        };
    }
}

fn ml_main() {
    assert!(((self::accept_rust_borrowed_iterator_receiver(true, String::from("iter"), 0f64, String::from("array"), String::from("library"), String::from("std::vec::Vec"), 1f64, true)) == (true)), "assertion 1");
    assert!(((self::accept_rust_borrowed_iterator_receiver(false, String::from("iter"), 0f64, String::from("array"), String::from("library"), String::from("std::vec::Vec"), 1f64, true)) == (false)), "assertion 2");
    assert!(((self::accept_rust_borrowed_iterator_receiver(true, String::from("into_iter"), 0f64, String::from("array"), String::from("library"), String::from("std::vec::Vec"), 1f64, true)) == (false)), "assertion 3");
    assert!(((self::accept_rust_borrowed_iterator_receiver(true, String::from("iter"), 0f64, String::from("array"), String::from("declaration"), String::from("std::vec::Vec"), 1f64, true)) == (false)), "assertion 4");
    assert!(((self::accept_rust_borrowed_iterator_receiver(true, String::from("iter"), 0f64, String::from("array"), String::from("library"), String::from("std::vec::Vec"), 1f64, false)) == (false)), "assertion 5");
    assert!(((self::read_rust_borrowed_iterator_continuation(String::from("map"))) == "map"), "assertion 6");
    assert!(((self::read_rust_borrowed_iterator_continuation(String::from("copied"))) == ""), "assertion 7");
    assert!(((self::read_rust_borrowed_iterator_projection_syntax(String::from("match"))) == "match"), "assertion 8");
    assert!(((self::read_rust_borrowed_iterator_projection_syntax(String::from("let"))) == ""), "assertion 9");
    assert!(((self::read_rust_borrowed_iterator_projection_syntax(String::from("app"))) == ""), "assertion 10");
    assert!(((self::read_rust_borrowed_iterator_callback_error(true, 1f64, true, true, true, 0f64, true)) == ""), "assertion 11");
    assert!(((self::read_rust_borrowed_iterator_callback_error(true, 1f64, true, true, true, 1f64, true)) == "a readonly iterator callback needs one actual immutable reference parameter, a closed reference result and a capture-free parameter projection"), "assertion 12");
    assert!(((self::read_rust_borrowed_iterator_callback_error(true, 1f64, true, false, true, 0f64, true)) == "a readonly iterator callback needs one actual immutable reference parameter, a closed reference result and a capture-free parameter projection"), "assertion 13");
    assert!(((self::project_rust_borrowed_iterator_callback_regions(vec![String::from("parameter")], vec![String::from("receiver")], vec![String::from("parameter")], vec![String::from("receiver")])) == (vec![String::from("receiver")])), "assertion 14");
    assert!(((self::project_rust_borrowed_iterator_callback_regions(vec![String::from("receiver"), String::from("parameter")], vec![String::from("receiver")], vec![String::from("parameter")], vec![String::from("receiver")])) == (vec![String::from("receiver")])), "assertion 15");
    assert!(((self::project_rust_borrowed_iterator_callback_regions(vec![String::from("old unrelated")], vec![String::from("old unrelated")], vec![String::from("old unrelated")], vec![String::from("receiver")])) == (Vec::<String>::new())), "assertion 16");
    assert!(((self::project_rust_borrowed_iterator_callback_regions(vec![String::from("unknown")], Vec::<String>::new(), vec![String::from("parameter")], vec![String::from("receiver")])) == (Vec::<String>::new())), "assertion 17");
    assert!(((self::read_rust_borrowed_iterator_region_error(1f64, 1f64, false)) == ""), "assertion 18");
    assert!(((self::read_rust_borrowed_iterator_region_error(1f64, 0f64, false)) == "a readonly iterator value needs its actual live receiver-derived regions"), "assertion 19");
    assert!(((self::accept_rust_borrowed_iterator_once(String::from("std::iter::once"), false, 1f64)) == (true)), "assertion 20");
    assert!(((self::accept_rust_borrowed_iterator_once(String::from("std::iter::once"), true, 1f64)) == (false)), "assertion 21");
    assert!(((self::accept_rust_borrowed_iterator_once(String::from("my::once"), false, 1f64)) == (false)), "assertion 22");
    assert!(((self::accept_rust_borrowed_iterator_box_reference(true, true, 0f64, true, true, false)) == (true)), "assertion 23");
    assert!(((self::accept_rust_borrowed_iterator_box_reference(true, false, 0f64, true, true, false)) == (false)), "assertion 24");
    assert!(((self::accept_rust_borrowed_iterator_box_reference(true, true, 0f64, true, false, false)) == (false)), "assertion 25");
    assert!(((self::read_rust_borrowed_iterator_budget_error(64f64, 100000f64, 65536f64)) == ""), "assertion 26");
    assert!(((self::read_rust_borrowed_iterator_budget_error(65f64, 0f64, 0f64)) == "the readonly iterator source pipeline exceeds its finite profile"), "assertion 27");
    assert!(((self::read_rust_borrowed_iterator_budget_error(0f64, f64::NAN, 0f64)) == "invalid readonly iterator source resource count"), "assertion 28");
    assert!(((self::read_rust_borrowed_iterator_budget_error(0f64, 0f64, (-0f64))) == "invalid readonly iterator source resource count"), "assertion 29");
    assert!(((self::read_rust_borrowed_iterator_budget_error(0f64, f64::INFINITY, 0f64)) == "invalid readonly iterator source resource count"), "assertion 30");
    assert!(((self::read_rust_borrowed_iterator_budget_error(0f64, 0.5f64, 0f64)) == "invalid readonly iterator source resource count"), "assertion 31");
    assert!(((self::read_rust_borrowed_iterator_budget_error(0f64, (-1f64), 0f64)) == "invalid readonly iterator source resource count"), "assertion 32");
    assert!(((self::read_rust_borrowed_iterator_budget_error(0f64, 9007199254740992f64, 0f64)) == "invalid readonly iterator source resource count"), "assertion 33");
    assert!(((self::accept_rust_borrowed_iterator_production(String::from("map"), true, true, true)) == (true)), "assertion 34");
    assert!(((self::accept_rust_borrowed_iterator_production(String::from("map"), true, true, false)) == (false)), "assertion 35");
    assert!(((self::accept_rust_borrowed_iterator_whole_slice(vec![String::from("lit"), String::from("lit")], vec![String::from("float"), String::from("float")], vec![String::from("0"), String::from("Infinity")])) == (true)), "assertion 36");
    assert!(((self::accept_rust_borrowed_iterator_whole_slice(vec![String::from("lit"), String::from("lit")], vec![String::from("float"), String::from("float")], vec![String::from("1"), String::from("Infinity")])) == (false)), "assertion 37");
    assert!(((self::accept_rust_borrowed_iterator_whole_slice(vec![String::from("lit"), String::from("lit")], vec![String::from("fixed"), String::from("float")], vec![String::from("0"), String::from("Infinity")])) == (false)), "assertion 38");
    assert!(((self::read_rust_borrowed_iterator_snapshot_error(100000f64, 16777216f64)) == ""), "assertion 39");
    assert!(((self::read_rust_borrowed_iterator_snapshot_error(100001f64, 0f64)) == "readonly callback ownership snapshots exceed their finite source profile"), "assertion 40");
    assert!(((self::read_rust_borrowed_iterator_snapshot_error(0f64, (-0f64))) == "readonly callback ownership snapshots exceed their finite source profile"), "assertion 41");
    assert!(((self::read_rust_borrowed_iterator_parameter_error(String::from("field"), String::from("field"))) == ""), "assertion 42");
    assert!(((self::read_rust_borrowed_iterator_parameter_error(String::from("mut"), String::from("mut"))) == "the readonly callback needs a fresh direct immutable binding identifier"), "assertion 43");
    assert!(((self::read_rust_borrowed_iterator_parameter_error(String::from("mut"), String::from("r#mut"))) == ""), "assertion 44");
    assert!(((self::read_rust_borrowed_iterator_binding_owner_error(String::from("ctor"), false)) == "readonly callback parameters require a fresh binder rather than a constructor or constant pattern"), "assertion 45");
    assert!(((self::read_rust_borrowed_iterator_binding_owner_error(String::from(""), true)) == "readonly callback parameters require a fresh binder rather than a constructor or constant pattern"), "assertion 46");
    assert!(((self::read_rust_borrowed_iterator_binding_owner_error(String::from("fn"), false)) == ""), "assertion 47");
    assert!(((self::read_rust_borrowed_iterator_region_work_error(256f64)) == ""), "assertion 48");
    assert!(((self::read_rust_borrowed_iterator_region_work_error(257f64)) == "readonly iterator region union exceeds its bounded source work profile"), "assertion 49");
    assert!(((self::read_rust_borrowed_iterator_region_work_error((-0f64))) == "readonly iterator region union exceeds its bounded source work profile"), "assertion 50");
    assert!(((self::read_rust_borrowed_iterator_region_work_error(f64::NAN)) == "readonly iterator region union exceeds its bounded source work profile"), "assertion 51");
    assert!(((self::read_rust_borrowed_iterator_box_reference_error(true, 1f64, false, false)) == "canonical Box::as_ref expects zero arguments"), "assertion 52");
    assert!(((self::read_rust_borrowed_iterator_box_reference_error(true, 0f64, false, false)) == "canonical Box::as_ref requires its actual live readonly receiver projection"), "assertion 53");
    assert!(((self::read_rust_borrowed_iterator_box_reference_error(false, 1f64, false, false)) == ""), "assertion 54");
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
fn all_literal_borrowed_iterator_controls() { main(); }
