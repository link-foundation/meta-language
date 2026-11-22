// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result;
use super::source_ml_source1_temporary_read_java_script_semantic_value_error_loop1;

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

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSemanticValue {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    Integer(self::ml::Big),
    Text(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    SourceText(String, String),
    Symbol(self::ml::Big),
    Object(self::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JavaScriptArrayConstructorFact {
    ArrayConstructorFact(self::ml::Big, bool, self::ml::Big, bool, Option<(Option<String>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptConstructorRealm {
    ConstructorRealm(self::ml::Big, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Option<(Option<self::JavaScriptSemanticValue>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>, Option<(Option<self::JavaScriptSemanticValue>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptArraySpeciesDirectory {
    ArraySpeciesDirectory(self::ml::Big, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptArrayConstructorFact>>>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptConstructorRealm>>>>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceUnit {
    CallSourceUnit(String, Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceSpan {
    CallSourceSpan(String, f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallInstruction {
    Constant(f64, Box<self::JavaScriptSemanticValue>),
    Copy(f64, f64),
    Argument(f64, f64),
    ArgumentCount(f64),
    ArgumentAt(f64, f64),
    FunctionBind(f64),
    BoundFunctionCall(f64),
    FunctionToString(f64),
    PushFinally(f64),
    FinishFinally,
    CompleteJump(f64, f64),
    Jump(f64),
    Branch(f64, f64, f64),
    Call(f64, f64, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    ArgumentListOpen(f64),
    ArgumentListAppend(f64, f64),
    ArgumentListCall(f64, f64, f64, f64, String),
    Construct(f64, f64, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    ConstructorCheck(f64, f64, bool),
    ReadNewTarget(f64),
    ArrayJoinEnter(f64, f64),
    ArrayJoinLeave(f64),
    ConstructErrorClass(f64, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    CallClassConstructor(f64, f64, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    ConstructCollection(f64, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    CollectionAllocate(f64, f64, String),
    CollectionIteratorAllocate(f64, f64, String, String, Option<(Option<f64>,)>),
    ArgumentRest(f64, f64, f64, self::ml::Big),
    IteratorProtocol(f64, f64, f64, f64, f64, String, self::ml::Big, Option<(Option<bool>,)>),
    CollectionIteratorResult(f64, String, f64, f64),
    CollectionOperation(f64, String),
    PrimitiveMethod(f64, String),
    CollectionForEach(f64, String),
    ArrayConcatenation(f64, Option<(Option<f64>,)>, f64, Box<self::JavaScriptArraySpeciesDirectory>),
    ArraySlice(f64, Option<(Option<f64>,)>, Box<self::JavaScriptArraySpeciesDirectory>),
    ArrayCreation(f64, String, f64, f64),
    ArrayCallback(f64, String, Option<(Option<f64>,)>),
    ArraySearch(f64, String, Option<(Option<f64>,)>),
    ArrayTransform(f64, String, Box<self::JavaScriptArraySpeciesDirectory>, Option<(Option<f64>,)>),
    ObjectEnumeration(f64, String, f64),
    PropertyDefinitions(f64, String, f64, f64),
    PropertyReflection(f64, String, f64, f64, f64, f64),
    InstanceOf(f64, f64, f64, f64, String),
    ObjectObservation(f64, String, f64, f64),
    CollectionCopy(f64, f64, f64, f64, f64, f64, f64, f64),
    ConstructErrorValue(f64, f64, f64, f64, f64, f64, self::ml::Big, String, bool),
    ReadDerivedReceiver(f64),
    InvokeErrorIntrinsic(f64),
    InvokeHostCapability(f64),
    Return(f64),
    Throw(f64),
    PushHandler(f64, f64),
    PopHandler,
    NumberBinary(f64, f64, f64, String),
    SameValue(f64, f64, f64),
    StrictEqual(f64, f64, f64),
    RequireWrite(f64),
    Primitive(f64, String, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    OrdinaryAllocate(f64, f64),
    LiteralPrototype(f64, f64, f64),
    LiteralAccessor(f64, f64, f64, f64, String, self::ml::Big),
    ObjectFromEntries(f64, f64, f64, f64),
    PrimitiveWrapper(f64, f64, f64),
    SymbolAllocate(f64, f64),
    SymbolDescription(f64, f64),
    SymbolRegistry(f64, f64, String),
    FunctionName(f64, f64, f64, String, self::ml::Big),
    ArrayAllocate(f64, f64, f64),
    ClosureAllocate(f64, self::ml::Big, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>, f64),
    ArraySpread(f64, f64, f64, f64, f64, self::ml::Big),
    ObjectSpread(f64, f64, f64, self::ml::Big, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, Option<(Option<String>,)>),
    OrdinaryDataProperty(f64, f64, f64, f64, self::ml::Big),
    OrdinaryGet(f64, f64, f64, f64),
    OrdinaryDelete(f64, f64, f64),
    OrdinaryHas(f64, f64, f64),
    OrdinarySet(f64, f64, f64, f64, f64),
    OrdinaryDefine(f64, f64, f64, std::rc::Rc<std::cell::RefCell<Vec<f64>>>, self::ml::Big),
    PropertyKey(f64, f64, self::ml::Big),
    PropertyDefinition(f64, f64, f64, f64, self::ml::Big, Option<(Option<String>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallFunction {
    CallFunction(self::ml::Big, self::ml::Big, String, Box<self::JavaScriptSemanticValue>, f64, f64, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>, Option<(Option<self::JavaScriptSemanticValue>,)>, Option<(Option<self::JavaScriptCallSourceSpan>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallTemplate {
    CallTemplate(self::ml::Big, f64, self::ml::Big, f64, f64, f64, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, f64, Option<(Option<String>,)>, Option<(Option<self::JavaScriptCallSourceSpan>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallProgram {
    CallProgram(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, Box<self::JavaScriptSemanticValue>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,)>, Option<(Option<self::JavaScriptArraySpeciesDirectory>,)>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSource {
    SourceConstant(Box<self::JavaScriptSemanticValue>),
    SourceNewTarget,
    SourceReceiver,
    SourceArgument(f64),
    SourceDerivedReceiver,
    SourceErrorIntrinsic,
    SourceConstruct(Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>),
    SourceConstructErrorClass(Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>),
    SourceClassConstructorCall(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>),
    SourceConstructCollection(Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>),
    SourceVariadicCall(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, String, bool),
    SourceArgumentSpread(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, self::ml::Big),
    SourceConstructErrorValue(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, self::ml::Big, String, bool),
    SourceThrow(Box<self::JavaScriptCallSource>),
    SourceReturn(Box<self::JavaScriptCallSource>),
    SourceAssignBinding(f64, Box<self::JavaScriptCallSource>),
    SourceImmutableBindingWrite(Box<self::JavaScriptCallSource>),
    SourceWhile(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Option<(Option<f64>,)>, Option<(Option<self::JavaScriptCallSource>,)>, Option<(Option<bool>,)>, Option<(Option<self::JavaScriptCallSource>,)>),
    SourceCloneBindingCell(f64, self::ml::Big, Option<(Option<bool>,)>),
    SourceControlJump(f64, bool),
    SourceControlBlock(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceSwitch(f64, Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>, Box<self::JavaScriptCallSource>),
    SourceBindingScope(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallBindingDeclaration>>>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceLexicalScope(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>>,)>, self::ml::Big, Box<self::JavaScriptCallSource>),
    SourceWriteExisting(f64, bool, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceInitializeCell(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceInitializeBinding(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceIteratorBinding(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, self::ml::Big),
    SourceIteratorStep(f64, self::ml::Big, bool),
    SourceIteratorRemainder(f64, Box<self::JavaScriptCallSource>, self::ml::Big),
    SourceForOf(f64, f64, f64, bool, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, self::ml::Big, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>),
    SourceIteratorLoop(f64, f64, f64, bool, Box<self::JavaScriptCallSource>, self::ml::Big, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>),
    SourceIteratorClose(f64, self::ml::Big),
    SourceFinally(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceTry(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceBinding(f64),
    SourceLet(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceLocalLet(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, bool),
    SourceArgumentRest(f64, Box<self::JavaScriptCallSource>, self::ml::Big),
    SourceParameterRest(f64, f64, Box<self::JavaScriptCallSource>, self::ml::Big, Box<self::JavaScriptCallSource>),
    SourceParameterDefault(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceIf(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceNumberBinary(String, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceSameValue(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceStrictEqual(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceSnapshotStrictEqual(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourcePrimitive(String, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>),
    SourceRecur(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>),
    SourceAllocate(Box<self::JavaScriptCallSource>),
    SourceClosure(self::ml::Big, f64, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, Box<self::JavaScriptCallSource>),
    SourceObject(Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceProperty>>>, self::ml::Big),
    SourceArrayAccumulation(Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArrayPart>>>, Box<self::JavaScriptCallSource>, self::ml::Big),
    SourceArray(Box<self::JavaScriptCallSource>, f64, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceArrayElement>>>, self::ml::Big),
    SourcePropertyKey(Box<self::JavaScriptCallSource>, self::ml::Big),
    SourcePropertyDefinition(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, self::ml::Big),
    SourceSet(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceSnapshotSet(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceStringGet(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceGet(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Option<(Option<self::JavaScriptCallSource>,)>),
    SourceInstanceOf(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceDelete(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceHas(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceCall(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>),
    SourceSnapshotCall(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceProperty {
    SourceObjectProperty(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Option<(Option<bool>,)>),
    SourceObjectAccessor(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, String, Option<(Option<bool>,)>),
    SourceLiteralPrototype(Box<self::JavaScriptCallSource>),
    SourceObjectSpread(Box<self::JavaScriptCallSource>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallArrayPart {
    SourceArrayValue(Box<self::JavaScriptCallSource>),
    SourceArraySpread(Box<self::JavaScriptCallSource>),
    SourceArrayHole,
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceArrayElement {
    SourceArrayElement(f64, Box<self::JavaScriptCallSource>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceSwitchClause {
    SourceSwitchCase(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceSwitchDefault(Box<self::JavaScriptCallSource>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceControlScope {
    SourceControlScope(f64, String, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceControlTransfer {
    SourceControlTransfer(f64, f64, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallCellInitializer {
    SourceCellInitializer(f64, Box<self::JavaScriptCallSource>, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallBindingDeclaration {
    SourceBindingDeclaration(f64, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceBinding {
    SourceBindingRegister(f64, f64, bool, Option<(Option<bool>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceBuilder {
    SourceBuilder(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>, f64, f64, f64, f64, String, Option<(Option<bool>,)>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlScope>>>>,)>, Option<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlTransfer>>>>,)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceResult {
    SourceResult(Box<self::JavaScriptCallSourceBuilder>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceCompilation {
    SourceCompiled(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>, f64, String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_text_profile_error(profile: &str, length: f64) -> String {
    if (profile != "inspectable UTF16 source text 1") {
        String::from("source text requires its explicit inspectable UTF16 resource profile")
    } else {
        if (((!(self::ml_math::is_safe_integer(length)) || (length < (0f64))) || (length > (16777216f64))) || ((length == (0f64)) && (((1f64 / length)) < (0f64)))) {
            String::from("source text exceeds the explicit 16777216 UTF16 unit budget")
        } else {
            String::from("")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_value_error(value: self::JavaScriptSemanticValue) -> String {
    match value.clone() {
        self::JavaScriptSemanticValue::Text(units) => {
            if (({ let ml_reference_array1 = units.clone(); let ml_reference_projection1 = ml_reference_array1.borrow().len() as f64; ml_reference_projection1 }) > (1048576f64)) {
                String::from("semantic text exceeds the explicit UTF16 unit budget")
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s5 = self::source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(units.clone(), at);
                        match source_ml_source2_s5.clone() {
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done => {
                                String::from("")
                            }
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(source_ml_source1_temporary_result1) => {
                                source_ml_source1_temporary_result1.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(profile, text) => {
            self::read_java_script_source_text_profile_error(&(profile), (text.encode_utf16().count() as f64))
        }
        self::JavaScriptSemanticValue::Symbol(identity) => {
            if ((identity > (self::ml::Big::from_i128(0))) && (identity <= (self::ml::Big::from_i128(18446744073709551615)))) {
                String::from("")
            } else {
                String::from("a symbol requires a positive unsigned64 global link identity")
            }
        }
        self::JavaScriptSemanticValue::Object(identity_2, _) => {
            if ((identity_2 > (self::ml::Big::from_i128(0))) && (identity_2 <= (self::ml::Big::from_i128(18446744073709551615)))) {
                String::from("")
            } else {
                String::from("an object requires a positive unsigned64 global link identity")
            }
        }
        _ => {
            String::from("")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_source_coordinate(value: f64, limit: f64) -> bool {
    (((self::ml_math::is_safe_integer(value) && (value >= (0f64))) && (value < limit)) && ((value != (0f64)) || (((1f64 / value)) > (0f64))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_code_coordinate(value: f64, limit: f64) -> bool {
    self::test_java_script_source_coordinate(value, limit)
}
