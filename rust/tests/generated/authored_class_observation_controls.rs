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

pub mod ml_source_exception {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SourceException {
        pub class_name: String,
        pub message_units: Vec<u16>,
        pub message_present: bool,
    }
    impl SourceException {
        pub fn raise(self) -> ! {
            std::panic::panic_any((self.class_name, self.message_units, self.message_present))
        }
        pub fn from_text(class_name: &str, message: &str) -> Self {
            Self { class_name: class_name.to_owned(), message_units: message.encode_utf16().collect(), message_present: true }
        }
    }
    impl std::fmt::Display for SourceException {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.message_units))
        }
    }
    impl std::error::Error for SourceException {}
}

pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required Number fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

pub fn refuse_required_text_fact() -> String {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required text fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required Boolean fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

pub fn read_required_number_fact(values: Vec<f64>, index: f64) -> f64 {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required Number fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_number_fact())
    }
}

pub fn read_required_text_fact(values: Vec<String>, index: f64) -> String {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required text fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

pub fn read_required_boolean_fact(values: Vec<bool>, index: f64) -> bool {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ((values.len() as f64)))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required Boolean fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        (self::ml_array::optional_at(&values, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::refuse_required_boolean_fact())
    }
}

pub fn dependency_function1(starts: Vec<f64>, counts: Vec<f64>, labels: Vec<String>, targets: Vec<f64>, node: f64, field: String) -> f64 {
    if (node < (0f64)) {
        (-1f64)
    } else {
        {
            let start = self::read_required_number_fact(starts, node);
            {
                let count = self::read_required_number_fact(counts, node);
                {
                    let at = start;
                    {
                        let source_ml_source2_s1 = self::source_ml_source1_temporary_dependency_function1_loop1(labels, targets, field.clone(), start, count, at);
                        match source_ml_source2_s1.clone() {
                            self::MlSource1TemporaryDependencyFunction1Loop1Result::MlSource1TemporaryDependencyFunction1Loop1Done => {
                                (-1f64)
                            }
                            self::MlSource1TemporaryDependencyFunction1Loop1Result::MlSource1TemporaryDependencyFunction1Loop1Return(source_ml_source1_temporary_result1) => {
                                source_ml_source1_temporary_result1
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn dependency_function2(starts: Vec<f64>, counts: Vec<f64>, targets: Vec<f64>, node: f64, index: f64) -> f64 {
    if (((node < (0f64)) || (index < (0f64))) || (index >= (self::read_required_number_fact(counts, node)))) {
        (-1f64)
    } else {
        self::read_required_number_fact(targets, (self::read_required_number_fact(starts, node) + index))
    }
}

pub fn read_collection_construction_kind(texts: Vec<String>, starts: Vec<f64>, counts: Vec<f64>, labels: Vec<String>, targets: Vec<f64>, node: f64) -> String {
    {
        let kind = self::dependency_function1(starts, counts, labels, targets, node, String::from("kind"));
        if (kind < (0f64)) {
            String::from("")
        } else {
            self::read_required_text_fact(texts, kind)
        }
    }
}

pub fn plan_collection_construction_inference(texts: Vec<String>, starts: Vec<f64>, counts: Vec<f64>, labels: Vec<String>, targets: Vec<f64>, kind: String, expected: f64) -> Vec<f64> {
    {
        let context = ((self::read_collection_construction_kind(texts, starts.clone(), counts.clone(), labels.clone(), targets.clone(), expected)) == kind);
        if (kind == "set") {
            vec![1f64, if context {
                1f64
            } else {
                0f64
            }, if context {
                self::dependency_function1(starts, counts, labels, targets, expected, String::from("element"))
            } else {
                (-1f64)
            }, (-1f64)]
        } else {
            if (kind == "map") {
                vec![2f64, if context {
                    1f64
                } else {
                    0f64
                }, if context {
                    self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), expected, String::from("key"))
                } else {
                    (-1f64)
                }, if context {
                    self::dependency_function1(starts, counts, labels, targets, expected, String::from("value"))
                } else {
                    (-1f64)
                }]
            } else {
                vec![0f64, 0f64, (-1f64), (-1f64)]
            }
        }
    }
}

pub fn plan_collection_construction_checked(texts: Vec<String>, starts: Vec<f64>, counts: Vec<f64>, labels: Vec<String>, targets: Vec<f64>, kind: String, items: f64) -> Vec<f64> {
    {
        let array = ((self::read_collection_construction_kind(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), items)) == "array");
        {
            let element = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), items, String::from("element"));
            if (kind == "set") {
                if array {
                    vec![0f64, element, (-1f64)]
                } else {
                    vec![1f64, (-1f64), (-1f64)]
                }
            } else {
                {
                    let tuple = ((self::read_collection_construction_kind(texts, starts.clone(), counts.clone(), labels.clone(), targets.clone(), element)) == "tuple");
                    {
                        let elements = self::dependency_function1(starts.clone(), counts.clone(), labels, targets.clone(), element, String::from("elements"));
                        if (((!array || !tuple) || (elements < (0f64))) || ((self::read_required_number_fact(counts.clone(), elements)) != (2f64))) {
                            vec![2f64, (-1f64), (-1f64)]
                        } else {
                            vec![0f64, self::dependency_function2(starts.clone(), counts.clone(), targets.clone(), elements, 0f64), self::dependency_function2(starts, counts, targets, elements, 1f64)]
                        }
                    }
                }
            }
        }
    }
}

pub fn read_collection_construction_key_admission(kind: String, primitive: bool) -> f64 {
    if primitive {
        0f64
    } else {
        if (kind == "set") {
            3f64
        } else {
            4f64
        }
    }
}

pub fn read_collection_construction_diagnostic(code: f64, part: String) -> String {
    if ((part == "message") && (code == (1f64))) {
        String::from("Set constructor needs an iterable array")
    } else {
        if ((part == "message") && (code == (2f64))) {
            String::from("Map constructor needs an iterable of key/value pairs")
        } else {
            if ((part == "message") && (code == (3f64))) {
                String::from("set keys need explicit reference identities for structured values")
            } else {
                if ((part == "message") && (code == (4f64))) {
                    String::from("map keys need explicit reference identities for structured values")
                } else {
                    if ((part == "construct") && (code == (3f64))) {
                        String::from("Set object identity")
                    } else {
                        if ((part == "construct") && (code == (4f64))) {
                            String::from("Map object identity")
                        } else {
                            String::from("")
                        }
                    }
                }
            }
        }
    }
}

pub fn read_collection_construction_binding(target: String, kind: String, role: String) -> String {
    {
        let set = (kind == "set");
        if (target == "Rocq") {
            if (role == "previous") {
                if set {
                    String::from("ml_previous")
                } else {
                    String::from("(fst ml_previous)")
                }
            } else {
                if set {
                    String::from("ml_value")
                } else {
                    String::from("(fst ml_value)")
                }
            }
        } else {
            if (role == "previous") {
                if set {
                    String::from("ml_previous")
                } else {
                    String::from("ml_previous.1")
                }
            } else {
                if set {
                    String::from("ml_value")
                } else {
                    String::from("ml_value.1")
                }
            }
        }
    }
}

pub fn render_collection_construction_type(target: String, kind: String, first: String, second: String) -> String {
    if (target == "Lean") {
        if (kind == "set") {
            format!("{}{}", (format!("{}{}", (String::from("(Array ")), first)), (String::from(")")))
        } else {
            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(Array (")), first)), (String::from(" × ")))), second)), (String::from("))")))
        }
    } else {
        if (target == "Rocq") {
            if (kind == "set") {
                format!("{}{}", (format!("{}{}", (String::from("(list ")), first)), (String::from(")")))
            } else {
                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(list (")), first)), (String::from(" * ")))), second)), (String::from("))%type")))
            }
        } else {
            String::from("")
        }
    }
}

pub fn render_collection_construction(target: String, kind: String, key_kind: String, values: String, collection_type: String, element_type: String, comparison: String) -> String {
    {
        let set = (kind == "set");
        if (target == "JavaScript") {
            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("new ")), (if set {
                String::from("Set")
            } else {
                String::from("Map")
            }))), (String::from("(")))), values)), (String::from(")")))
        } else {
            if (target == "Rust") {
                {
                    let comparator = if (key_kind == "float") {
                        String::from("|a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())")
                    } else {
                        String::from("|a, b| a == b")
                    };
                    {
                        let values_3 = if (key_kind == "float") {
                            {
                                let values_2 = format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), values)), (String::from(").into_iter().map(|ml_value| ")))), (if set {
                                    String::from("if ml_value == 0f64 { 0f64 } else { ml_value }")
                                } else {
                                    String::from("(if ml_value.0 == 0f64 { 0f64 } else { ml_value.0 }, ml_value.1)")
                                }))), (String::from(").collect::<Vec<_>>()")));
                                values_2.clone()
                            }
                        } else {
                            values.clone()
                        };
                        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_array::")), kind)), (String::from("(")))), values_3)), (String::from(", ")))), comparator)), (String::from(")")))
                    }
                }
            } else {
                {
                    let value = self::read_collection_construction_binding(target.clone(), kind.clone(), String::from("value"));
                    {
                        let previous = self::read_collection_construction_binding(target.clone(), kind.clone(), String::from("previous"));
                        if (target == "Lean") {
                            {
                                let equal = if (key_kind == "float") {
                                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), previous)), (String::from(" == ")))), value)), (String::from(" || (")))), previous)), (String::from(".isNaN && ")))), value)), (String::from(".isNaN))")))
                                } else {
                                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), previous)), (String::from(" == ")))), value)), (String::from(")")))
                                };
                                {
                                    let normalized = if (key_kind == "float") {
                                        if set {
                                            String::from("(if ml_original == (0 : Float) then (0 : Float) else ml_original)")
                                        } else {
                                            String::from("((if ml_original.1 == (0 : Float) then (0 : Float) else ml_original.1), ml_original.2)")
                                        }
                                    } else {
                                        String::from("ml_original")
                                    };
                                    {
                                        let existing = if set {
                                            String::from("ml_accumulator")
                                        } else {
                                            format!("{}{}", (format!("{}{}", (String::from("(ml_accumulator.map (fun ml_previous => if ")), equal)), (String::from(" then (ml_previous.1, ml_value.2) else ml_previous))")))
                                        };
                                        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(")), values)), (String::from(".foldl (fun ml_accumulator ml_original => let ml_value := ")))), normalized)), (String::from("; if ml_accumulator.any (fun ml_previous => ")))), equal)), (String::from(") then ")))), existing)), (String::from(" else ml_accumulator.push ml_value) (#[] : ")))), collection_type)), (String::from("))")))
                                    }
                                }
                            }
                        } else {
                            {
                                let equal_2 = comparison.clone();
                                {
                                    let equal_4 = if (key_kind == "float") {
                                        {
                                            let equal_3 = format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(orb ")), equal_2)), (String::from(" (andb (PrimFloat.is_nan ")))), previous)), (String::from(") (PrimFloat.is_nan ")))), value)), (String::from(")))")));
                                            equal_3.clone()
                                        }
                                    } else {
                                        equal_2.clone()
                                    };
                                    {
                                        let normalized_2 = if (key_kind == "float") {
                                            if set {
                                                String::from("(if PrimFloat.eqb ml_original 0%float then 0%float else ml_original)")
                                            } else {
                                                String::from("((if PrimFloat.eqb (fst ml_original) 0%float then 0%float else fst ml_original), snd ml_original)")
                                            }
                                        } else {
                                            String::from("ml_original")
                                        };
                                        {
                                            let existing_2 = if set {
                                                String::from("ml_accumulator")
                                            } else {
                                                format!("{}{}", (format!("{}{}", (String::from("(List.map (fun ml_previous => if ")), equal_4)), (String::from(" then (fst ml_previous, snd ml_value) else ml_previous) ml_accumulator)")))
                                            };
                                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(List.fold_left (fun ml_accumulator ml_original => let ml_value := ")), normalized_2)), (String::from(" in if List.existsb (fun ml_previous => ")))), equal_4)), (String::from(") ml_accumulator then ")))), existing_2)), (String::from(" else List.app ml_accumulator (ml_value :: nil)) ")))), values)), (String::from(" (@nil ")))), element_type)), (String::from("))")))
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

pub fn read_java_script_closed_class_observation_operands(kind: String, proof: String) -> String {
    if self::ml_array::includes(&vec![String::from("eq"), String::from("ne"), String::from("lt"), String::from("le"), String::from("gt"), String::from("ge")], &proof.clone(), 0f64, false, |a, b| a == b) {
        String::from("left right")
    } else {
        if (proof == "forall") {
            String::from("binders")
        } else {
            if ((kind == "toString") || (kind == "optionalText")) {
                String::from("arg")
            } else {
                if (kind == "strictEqual") {
                    String::from("left right")
                } else {
                    if ((kind == "print") || (kind == "output")) {
                        String::from("expr")
                    } else {
                        String::from("")
                    }
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_observation_operation_operands(kind: String, proof: String, operation: String) -> String {
    if ((kind == "binary") && self::ml_array::includes(&vec![String::from("eq"), String::from("ne")], &operation.clone(), 0f64, false, |a, b| a == b)) {
        String::from("left right")
    } else {
        self::read_java_script_closed_class_observation_operands(kind.clone(), proof.clone())
    }
}

pub fn contains_java_script_closed_class_brand(texts: Vec<String>, starts: Vec<f64>, counts: Vec<f64>, labels: Vec<String>, targets: Vec<f64>, root: f64, declarations: f64, brands: Vec<String>) -> bool {
    if (((brands.len() as f64)) == (0f64)) {
        false
    } else {
        {
            let pending = vec![root];
            {
                let seen = Vec::<f64>::new();
                {
                    let source_ml_source2_s2 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop2(texts, starts, counts, labels, targets, declarations, brands, pending, seen);
                    match source_ml_source2_s2.clone() {
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Done => {
                            false
                        }
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(source_ml_source1_temporary_result2) => {
                            source_ml_source1_temporary_result2
                        }
                    }
                }
            }
        }
    }
}

pub fn has_java_script_closed_class_exported_brand(texts: Vec<String>, starts: Vec<f64>, counts: Vec<f64>, labels: Vec<String>, targets: Vec<f64>, declarations: f64, exports: Vec<String>, brands: Vec<String>) -> bool {
    if (((exports.len() as f64)) == (0f64)) {
        false
    } else {
        if (declarations < (0f64)) {
            true
        } else {
            {
                let source_ml_source1_temporary_values8 = exports;
                {
                    let source_ml_source1_temporary_index8 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s3 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop9(texts, starts, counts, labels, targets, declarations, brands, source_ml_source1_temporary_values8, source_ml_source1_temporary_index8.clone());
                        match source_ml_source2_s3.clone() {
                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Done => {
                                false
                            }
                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(source_ml_source1_temporary_result9) => {
                                source_ml_source1_temporary_result9
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDependencyFunction1Loop1Result {
    MlSource1TemporaryDependencyFunction1Loop1Done,
    MlSource1TemporaryDependencyFunction1Loop1Return(f64),
}

pub fn source_ml_source1_temporary_dependency_function1_loop1(mut labels: Vec<String>, mut targets: Vec<f64>, mut field: String, mut start: f64, mut count: f64, mut at: f64) -> self::MlSource1TemporaryDependencyFunction1Loop1Result {
    loop {
        return if (at < ((start + count))) {
            if ((self::read_required_text_fact(labels.clone(), at)) == field) {
                self::MlSource1TemporaryDependencyFunction1Loop1Result::MlSource1TemporaryDependencyFunction1Loop1Return(self::read_required_number_fact(targets, at))
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
            self::MlSource1TemporaryDependencyFunction1Loop1Result::MlSource1TemporaryDependencyFunction1Loop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Done,
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Done(Vec<f64>),
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(bool),
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop3(mut texts: Vec<String>, mut starts: Vec<f64>, mut counts: Vec<f64>, mut labels: Vec<String>, mut targets: Vec<f64>, mut brands: Vec<String>, mut pending: Vec<f64>, mut fields: f64, mut at: f64) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result {
    loop {
        return if (at < (self::read_required_number_fact(counts.clone(), fields))) {
            {
                let row = self::dependency_function2(starts.clone(), counts.clone(), targets.clone(), fields, at);
                {
                    let name = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, String::from("name"));
                    if (name < (0f64)) {
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(true)
                    } else {
                        if self::ml_array::includes(&brands, &self::read_required_text_fact(texts.clone(), name), 0f64, false, |a, b| a == b) {
                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(true)
                        } else {
                            {
                                let pending_2 = {
                                    let source_ml_source2_concat_value4 = pending;
                                    {
                                        let source_ml_source2_concat_value5 = vec![self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, String::from("type"))];
                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value4), source_ml_source2_concat_value5)
                                    }
                                };
                                {
                                    let at_2 = (at + 1f64);
                                    {
                                        (pending, at) = (pending_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Done(pending)
        };
    }
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop4(mut starts: Vec<f64>, mut counts: Vec<f64>, mut targets: Vec<f64>, mut pending: Vec<f64>, mut children: f64, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < (self::read_required_number_fact(counts.clone(), children))) {
            {
                let pending_2 = {
                    let source_ml_source2_concat_value6 = pending;
                    {
                        let source_ml_source2_concat_value7 = vec![self::dependency_function2(starts.clone(), counts.clone(), targets.clone(), children, at)];
                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value6), source_ml_source2_concat_value7)
                    }
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        (pending, at) = (pending_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            pending
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Done(Vec<f64>, bool),
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Done(Vec<f64>),
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Return(bool),
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop7(mut starts: Vec<f64>, mut counts: Vec<f64>, mut labels: Vec<String>, mut targets: Vec<f64>, mut pending: Vec<f64>, mut fields: f64, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (self::read_required_number_fact(counts.clone(), fields))) {
            {
                let pending_2 = {
                    let source_ml_source2_concat_value8 = pending;
                    {
                        let source_ml_source2_concat_value9 = vec![self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function2(starts.clone(), counts.clone(), targets.clone(), fields, index), String::from("type"))];
                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value8), source_ml_source2_concat_value9)
                    }
                };
                {
                    let index_2 = (index + 1f64);
                    {
                        (pending, index) = (pending_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            pending
        };
    }
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop6(mut starts: Vec<f64>, mut counts: Vec<f64>, mut labels: Vec<String>, mut targets: Vec<f64>, mut pending: Vec<f64>, mut constructors: f64, mut ctor: f64) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result {
    loop {
        return if (ctor < (self::read_required_number_fact(counts.clone(), constructors))) {
            {
                let fields = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), self::dependency_function2(starts.clone(), counts.clone(), targets.clone(), constructors, ctor), String::from("fields"));
                if (fields < (0f64)) {
                    self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Return(true)
                } else {
                    {
                        let index = 0f64;
                        {
                            let pending_2 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop7(starts.clone(), counts.clone(), labels.clone(), targets.clone(), pending, fields, index);
                            {
                                let ctor_2 = (ctor + 1f64);
                                {
                                    (pending, ctor) = (pending_2, ctor_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Done(pending)
        };
    }
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop5(mut texts: Vec<String>, mut starts: Vec<f64>, mut counts: Vec<f64>, mut labels: Vec<String>, mut targets: Vec<f64>, mut declarations: f64, mut pending: Vec<f64>, mut name: f64, mut found: bool, mut at: f64) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result {
    loop {
        return if (at < (self::read_required_number_fact(counts.clone(), declarations))) {
            {
                let row = self::dependency_function2(starts.clone(), counts.clone(), targets.clone(), declarations, at);
                {
                    let full_name = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, String::from("fullName"));
                    if ((full_name < (0f64)) || ((self::read_required_text_fact(texts.clone(), full_name)) != (self::read_required_text_fact(texts.clone(), name)))) {
                        {
                            let at_2 = (at + 1f64);
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            let constructors = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), row, String::from("ctors"));
                            if (constructors < (0f64)) {
                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(true)
                            } else {
                                {
                                    let found_2 = true;
                                    {
                                        let ctor = 0f64;
                                        {
                                            let source_ml_source2_s10 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop6(starts.clone(), counts.clone(), labels.clone(), targets.clone(), pending, constructors, ctor);
                                            match source_ml_source2_s10.clone() {
                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Done(pending_2) => {
                                                    {
                                                        let at_3 = (at + 1f64);
                                                        {
                                                            (pending, found, at) = (pending_2, found_2, at_3);
                                                            continue;
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop6Return(source_ml_source1_temporary_result6) => {
                                                    self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(source_ml_source1_temporary_result6)
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
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Done(pending, found)
        };
    }
}

pub fn source_ml_source1_temporary_contains_java_script_closed_class_brand_loop2(mut texts: Vec<String>, mut starts: Vec<f64>, mut counts: Vec<f64>, mut labels: Vec<String>, mut targets: Vec<f64>, mut declarations: f64, mut brands: Vec<String>, mut pending: Vec<f64>, mut seen: Vec<f64>) -> self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result {
    loop {
        return if { let ml_value = (pending.len() as f64); ml_value != 0f64 && !ml_value.is_nan() } {
            if ((((seen.len() as f64)) >= (100000f64)) || (((pending.len() as f64)) > (100000f64))) {
                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
            } else {
                {
                    let node = self::read_required_number_fact(pending.clone(), ((pending.len() as f64) - 1f64));
                    {
                        let pending_2 = self::ml_array::slice(&pending, 0f64, (-1f64));
                        if ((node < (0f64)) || (node >= ((starts.len() as f64)))) {
                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                        } else {
                            if self::ml_array::includes(&seen, &node, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) {
                                {
                                    pending = pending_2;
                                    continue;
                                }
                            } else {
                                {
                                    let seen_2 = {
                                        let source_ml_source2_concat_value11 = seen;
                                        {
                                            let source_ml_source2_concat_value12 = vec![node];
                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value11), source_ml_source2_concat_value12)
                                        }
                                    };
                                    {
                                        let owner = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, String::from("kind"));
                                        if (owner < (0f64)) {
                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                        } else {
                                            {
                                                let kind = self::read_required_text_fact(texts.clone(), owner);
                                                if (kind == "record") {
                                                    {
                                                        let fields = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, String::from("fields"));
                                                        if (fields < (0f64)) {
                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                        } else {
                                                            {
                                                                let at = 0f64;
                                                                {
                                                                    let source_ml_source2_s13 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop3(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), brands.clone(), pending_2, fields, at);
                                                                    match source_ml_source2_s13.clone() {
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Done(pending_3) => {
                                                                            {
                                                                                (pending, seen) = (pending_3, seen_2);
                                                                                continue;
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(source_ml_source1_temporary_result3) => {
                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(source_ml_source1_temporary_result3)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if self::ml_array::includes(&vec![String::from("array"), String::from("option"), String::from("set"), String::from("cell")], &kind.clone(), 0f64, false, |a, b| a == b) {
                                                        {
                                                            let pending_4 = {
                                                                let source_ml_source2_concat_value14 = pending_2;
                                                                {
                                                                    let source_ml_source2_concat_value15 = vec![self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, String::from("element"))];
                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value14), source_ml_source2_concat_value15)
                                                                }
                                                            };
                                                            {
                                                                (pending, seen) = (pending_4, seen_2);
                                                                continue;
                                                            }
                                                        }
                                                    } else {
                                                        if (kind == "map") {
                                                            {
                                                                let pending_5 = {
                                                                    let source_ml_source2_concat_value16 = pending_2;
                                                                    {
                                                                        let source_ml_source2_concat_value17 = vec![self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, String::from("key")), self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, String::from("value"))];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value16), source_ml_source2_concat_value17)
                                                                    }
                                                                };
                                                                {
                                                                    (pending, seen) = (pending_5, seen_2);
                                                                    continue;
                                                                }
                                                            }
                                                        } else {
                                                            if (((kind == "tuple") || (kind == "union")) || (kind == "function")) {
                                                                {
                                                                    let children = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, if (kind == "tuple") {
                                                                        String::from("elements")
                                                                    } else {
                                                                        if (kind == "union") {
                                                                            String::from("alternatives")
                                                                        } else {
                                                                            String::from("params")
                                                                        }
                                                                    });
                                                                    if (children < (0f64)) {
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                    } else {
                                                                        {
                                                                            let at_2 = 0f64;
                                                                            {
                                                                                let pending_6 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop4(starts.clone(), counts.clone(), targets.clone(), pending_2, children, at_2);
                                                                                if (kind == "function") {
                                                                                    {
                                                                                        let pending_7 = {
                                                                                            let source_ml_source2_concat_value18 = pending_6;
                                                                                            {
                                                                                                let source_ml_source2_concat_value19 = vec![self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, String::from("ret"))];
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value18), source_ml_source2_concat_value19)
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            (pending, seen) = (pending_7, seen_2);
                                                                                            continue;
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    {
                                                                                        (pending, seen) = (pending_6, seen_2);
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if (kind == "data") {
                                                                    {
                                                                        let name = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), node, String::from("name"));
                                                                        if ((name < (0f64)) || (declarations < (0f64))) {
                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                        } else {
                                                                            {
                                                                                let found = false;
                                                                                {
                                                                                    let at_3 = 0f64;
                                                                                    {
                                                                                        let source_ml_source2_s20 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop5(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, pending_2, name, found, at_3);
                                                                                        match source_ml_source2_s20.clone() {
                                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Done(pending_8, found_2) => {
                                                                                                if !found_2 {
                                                                                                    self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                                                } else {
                                                                                                    {
                                                                                                        (pending, seen) = (pending_8, seen_2);
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop5Return(source_ml_source1_temporary_result5) => {
                                                                                                self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(source_ml_source1_temporary_result5)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    if !(self::ml_array::includes(&vec![String::from("unit"), String::from("bool"), String::from("float"), String::from("int"), String::from("nat"), String::from("fixed"), String::from("string")], &kind.clone(), 0f64, false, |a, b| a == b)) {
                                                                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(true)
                                                                    } else {
                                                                        {
                                                                            (pending, seen) = (pending_2, seen_2);
                                                                            continue;
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
            self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result {
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Done,
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result {
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Done(bool),
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result {
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Done,
    MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Return(bool),
}

pub fn source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop11(mut texts: Vec<String>, mut starts: Vec<f64>, mut counts: Vec<f64>, mut labels: Vec<String>, mut targets: Vec<f64>, mut declarations: f64, mut brands: Vec<String>, mut params: f64, mut at: f64) -> self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result {
    loop {
        return if (at < (self::read_required_number_fact(counts.clone(), params))) {
            {
                let parameter = self::dependency_function2(starts.clone(), counts.clone(), targets.clone(), params, at);
                {
                    let type_ = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), parameter, String::from("type"));
                    if self::contains_java_script_closed_class_brand(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), type_, declarations, brands.clone()) {
                        self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Return(true)
                    } else {
                        {
                            let at_2 = (at + 1f64);
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Done
        };
    }
}

pub fn source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop10(mut texts: Vec<String>, mut starts: Vec<f64>, mut counts: Vec<f64>, mut labels: Vec<String>, mut targets: Vec<f64>, mut declarations: f64, mut brands: Vec<String>, mut exported: String, mut found: bool, mut index: f64) -> self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result {
    loop {
        return if (index < (self::read_required_number_fact(counts.clone(), declarations))) {
            {
                let declaration = self::dependency_function2(starts.clone(), counts.clone(), targets.clone(), declarations, index);
                {
                    let name = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), declaration, String::from("name"));
                    if ((name < (0f64)) || ((self::read_required_text_fact(texts.clone(), name)) != exported)) {
                        {
                            let index_2 = (index + 1f64);
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            let kind = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), declaration, String::from("k"));
                            if ((kind < (0f64)) || ((self::read_required_text_fact(texts.clone(), kind)) != "fn")) {
                                {
                                    let index_3 = (index + 1f64);
                                    {
                                        index = index_3;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let params = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), declaration, String::from("params"));
                                    {
                                        let ret = self::dependency_function1(starts.clone(), counts.clone(), labels.clone(), targets.clone(), declaration, String::from("ret"));
                                        if ((found || (params < (0f64))) || self::contains_java_script_closed_class_brand(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), ret, declarations, brands.clone())) {
                                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(true)
                                        } else {
                                            {
                                                let found_2 = true;
                                                {
                                                    let at = 0f64;
                                                    {
                                                        let source_ml_source2_s21 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop11(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, brands.clone(), params, at);
                                                        match source_ml_source2_s21.clone() {
                                                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Done => {
                                                                {
                                                                    let index_4 = (index + 1f64);
                                                                    {
                                                                        (found, index) = (found_2, index_4);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop11Return(source_ml_source1_temporary_result11) => {
                                                                self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(source_ml_source1_temporary_result11)
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
            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Done(found)
        };
    }
}

pub fn source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop9(mut texts: Vec<String>, mut starts: Vec<f64>, mut counts: Vec<f64>, mut labels: Vec<String>, mut targets: Vec<f64>, mut declarations: f64, mut brands: Vec<String>, mut source_ml_source1_temporary_values8: Vec<String>, mut source_ml_source1_temporary_index8: self::ml::Big) -> self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result {
    loop {
        return if (source_ml_source1_temporary_index8 < (self::ml::Big::from_u128(source_ml_source1_temporary_values8.len() as u128))) {
            {
                let exported = self::ml_array::at(&source_ml_source1_temporary_values8, source_ml_source1_temporary_index8.to_index());
                {
                    let found = false;
                    {
                        let index = 0f64;
                        {
                            let source_ml_source2_s22 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop10(texts.clone(), starts.clone(), counts.clone(), labels.clone(), targets.clone(), declarations, brands.clone(), exported.clone(), found, index);
                            match source_ml_source2_s22.clone() {
                                self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Done(found_2) => {
                                    if !found_2 {
                                        self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(true)
                                    } else {
                                        {
                                            let source_ml_source1_temporary_index8_2 = source_ml_source1_temporary_index8.add(&self::ml::Big::from_i128(1));
                                            {
                                                source_ml_source1_temporary_index8 = source_ml_source1_temporary_index8_2.clone();
                                                continue;
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop10Return(source_ml_source1_temporary_result10) => {
                                    self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(source_ml_source1_temporary_result10)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Done
        };
    }
}

fn ml_main() {
    assert!(((self::read_java_script_closed_class_observation_operands(String::from("toString"), String::from(""))) == "arg"), "assertion 1");
    assert!(((self::read_java_script_closed_class_observation_operands(String::from("strictEqual"), String::from(""))) == "left right"), "assertion 2");
    assert!(((self::read_java_script_closed_class_observation_operands(String::from(""), String::from("eq"))) == "left right"), "assertion 3");
    assert!(((self::read_java_script_closed_class_observation_operands(String::from(""), String::from("forall"))) == "binders"), "assertion 4");
    assert!(((self::read_java_script_closed_class_observation_operands(String::from("recordField"), String::from(""))) == ""), "assertion 5");
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
fn all_literal_class_observation_controls() { main(); }
