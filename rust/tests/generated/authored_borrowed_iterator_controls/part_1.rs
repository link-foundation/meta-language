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
