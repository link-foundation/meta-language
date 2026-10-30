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
    Text(Vec<f64>),
    Symbol(self::ml::Big),
    Object(self::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallInstruction {
    Constant(f64, Box<self::JavaScriptSemanticValue>),
    Copy(f64, f64),
    Argument(f64, f64),
    Jump(f64),
    Branch(f64, f64, f64),
    Call(f64, f64, f64, Vec<f64>),
    Return(f64),
    Throw(f64),
    PushHandler(f64, f64),
    PopHandler,
    NumberBinary(f64, f64, f64, String),
    SameValue(f64, f64, f64),
    StrictEqual(f64, f64, f64),
    RequireWrite(f64),
    Primitive(f64, String, Vec<f64>),
    OrdinaryGet(f64, f64, f64, f64),
    OrdinarySet(f64, f64, f64, f64, f64),
    PropertyKey(f64, f64, self::ml::Big),
    PropertyDefinition(f64, f64, f64, f64, self::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallFunction {
    CallFunction(self::ml::Big, self::ml::Big, String, Box<self::JavaScriptSemanticValue>, f64, f64, Vec<self::JavaScriptSemanticValue>, Vec<self::JavaScriptCallInstruction>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallProgram {
    CallProgram(Vec<self::JavaScriptCallFunction>, Vec<self::ml::Big>, Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallPrimitiveResult {
    PrimitiveValue(Box<self::JavaScriptSemanticValue>),
    PrimitiveInvalid(String),
    PrimitiveError(String, String),
    PrimitiveResourceLimit(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSource {
    SourceConstant(Box<self::JavaScriptSemanticValue>),
    SourceReceiver,
    SourceArgument(f64),
    SourceThrow(Box<self::JavaScriptCallSource>),
    SourceTry(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceBinding(f64),
    SourceLet(f64, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceIf(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceNumberBinary(String, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceSameValue(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceStrictEqual(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourcePrimitive(String, Vec<self::JavaScriptCallSource>),
    SourceRecur(Vec<self::JavaScriptCallSource>),
    SourceSet(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceGet(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>),
    SourceCall(Box<self::JavaScriptCallSource>, Box<self::JavaScriptCallSource>, Vec<self::JavaScriptCallSource>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceBinding {
    SourceBindingRegister(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceBuilder {
    SourceBuilder(Vec<self::JavaScriptCallInstruction>, f64, f64, f64, f64, String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceResult {
    SourceResult(Box<self::JavaScriptCallSourceBuilder>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallSourceCompilation {
    SourceCompiled(Vec<self::JavaScriptCallInstruction>, f64, String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_value_error(value: self::JavaScriptSemanticValue) -> String {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            String::from("")
        }
        self::JavaScriptSemanticValue::Null => {
            String::from("")
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1) => {
            String::from("")
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f2) => {
            String::from("")
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f3) => {
            String::from("")
        }
        self::JavaScriptSemanticValue::Text(units) => {
            if (((units.len() as f64)) > (1048576f64)) {
                String::from("semantic text exceeds the explicit UTF16 unit budget")
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s5 = self::source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(&(units), at);
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
        self::JavaScriptSemanticValue::Symbol(identity) => {
            if ((identity > (self::ml::Big::from_i128(0))) && (identity <= (self::ml::Big::from_i128(18446744073709551615)))) {
                String::from("")
            } else {
                String::from("a symbol requires a positive unsigned64 global link identity")
            }
        }
        self::JavaScriptSemanticValue::Object(identity_2, source_ml_source2_f4) => {
            if ((identity_2 > (self::ml::Big::from_i128(0))) && (identity_2 <= (self::ml::Big::from_i128(18446744073709551615)))) {
                String::from("")
            } else {
                String::from("an object requires a positive unsigned64 global link identity")
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_code_coordinate(value: f64, limit: f64) -> bool {
    (((self::ml_math::is_safe_integer(value) && (value >= (0f64))) && (value < limit)) && ((value != (0f64)) || (((1f64 / value)) > (0f64))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_constant_error(value: self::JavaScriptSemanticValue) -> String {
    self::read_java_script_semantic_value_error(value.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_integer_arity(operation: &str) -> f64 {
    if (((operation == "negate integer") || (operation == "complement integer bits")) || (operation == "complement number bits")) {
        1f64
    } else {
        if self::ml_array::includes(&vec![String::from("add integers"), String::from("subtract integers"), String::from("multiply integers"), String::from("divide integers"), String::from("read integer remainder"), String::from("raise integer power"), String::from("compare integers"), String::from("compare number integer"), String::from("compare text integer"), String::from("conjoin integer bits"), String::from("disjoin integer bits"), String::from("exclude equal integer bits"), String::from("shift integer bits left"), String::from("shift integer bits right"), String::from("conjoin number bits"), String::from("disjoin number bits"), String::from("exclude equal number bits"), String::from("shift number bits left"), String::from("shift number bits right"), String::from("shift number bits right unsigned")], &operation.to_owned(), 0f64, false, |a, b| a == b) {
            2f64
        } else {
            0f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_exception_class(operation: &str) -> String {
    if (operation == "raiseJavaScriptError") {
        String::from("Error")
    } else {
        if (operation == "raiseJavaScriptTypeError") {
            String::from("TypeError")
        } else {
            if (operation == "raiseJavaScriptRangeError") {
                String::from("RangeError")
            } else {
                if (operation == "raiseJavaScriptSyntaxError") {
                    String::from("SyntaxError")
                } else {
                    if (operation == "raiseJavaScriptReferenceError") {
                        String::from("ReferenceError")
                    } else {
                        if (operation == "raiseJavaScriptURIError") {
                            String::from("URIError")
                        } else {
                            if (operation == "raiseJavaScriptEvalError") {
                                String::from("EvalError")
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_unary_number_operation(operation: &str) -> bool {
    self::ml_array::includes(&vec![String::from("negate number"), String::from("truncate number"), String::from("read absolute number"), String::from("floor number"), String::from("ceil number"), String::from("round number"), String::from("read number sign"), String::from("read square root"), String::from("test integer number"), String::from("test safe integer number")], &operation.to_owned(), 0f64, false, |a, b| a == b)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_primitive_arity_error(operation: &str, count: f64) -> String {
    if ((!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) || (count > (4096f64))) {
        String::from("primitive operands exceed their explicit call register budget")
    } else {
        if ((self::read_java_script_source_exception_class(operation)) != "") {
            if (count == (2f64)) {
                String::from("")
            } else {
                String::from("a source Error requires its actual text message and own-message presence")
            }
        } else {
            {
                let integer_arity = self::read_java_script_call_integer_arity(operation);
                if (integer_arity != (0f64)) {
                    if (count == integer_arity) {
                        String::from("")
                    } else {
                        String::from("primitive instructions require every actual operand coordinate")
                    }
                } else {
                    if (((operation == "read minimum number") || (operation == "read maximum number")) || (operation == "construct text units")) {
                        String::from("")
                    } else {
                        if self::test_java_script_call_unary_number_operation(operation) {
                            if (count == (1f64)) {
                                String::from("")
                            } else {
                                String::from("primitive instructions require every actual operand coordinate")
                            }
                        } else {
                            {
                                let unary = vec![String::from("read text length"), String::from("trim text"), String::from("trim text start"), String::from("trim text end"), String::from("convert explicit number"), String::from("coerce primitive number"), String::from("convert primitive text"), String::from("convert explicit integer"), String::from("coerce primitive integer")];
                                {
                                    let binary = vec![String::from("repeat text"), String::from("read text code point"), String::from("read text character"), String::from("read relative text character"), String::from("concatenate text"), String::from("compare text"), String::from("test text prefix"), String::from("test text suffix"), String::from("test text inclusion")];
                                    {
                                        let ternary = vec![String::from("pad text start"), String::from("pad text end"), String::from("slice text"), String::from("find text"), String::from("find last text"), String::from("replace text"), String::from("replace all text")];
                                        {
                                            let arity = if self::ml_array::includes(&unary, &operation.to_owned(), 0f64, false, |a, b| a == b) {
                                                1f64
                                            } else {
                                                if self::ml_array::includes(&binary, &operation.to_owned(), 0f64, false, |a, b| a == b) {
                                                    2f64
                                                } else {
                                                    if self::ml_array::includes(&ternary, &operation.to_owned(), 0f64, false, |a, b| a == b) {
                                                        3f64
                                                    } else {
                                                        (-1f64)
                                                    }
                                                }
                                            };
                                            if (arity < (0f64)) {
                                                String::from("primitive instructions require their actual registered operation")
                                            } else {
                                                if (count != arity) {
                                                    String::from("primitive instructions require every actual operand coordinate")
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_call_source(builder: self::JavaScriptCallSourceBuilder, error: &str) -> self::JavaScriptCallSourceResult {
    self::JavaScriptCallSourceResult::SourceResult(Box::new({
        let source_ml_source2_nominal_spread361 = builder.clone();
        {
            let source_ml_source2_nominal_field362 = match source_ml_source2_nominal_spread361.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value363, _, _, _, _, _) => {
                    source_ml_source2_data_property_value363
                }
            };
            {
                let source_ml_source2_nominal_field364 = match source_ml_source2_nominal_spread361.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value365, _, _, _, _) => {
                        source_ml_source2_data_property_value365
                    }
                };
                {
                    let source_ml_source2_nominal_field366 = match source_ml_source2_nominal_spread361.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value367, _, _, _) => {
                            source_ml_source2_data_property_value367
                        }
                    };
                    {
                        let source_ml_source2_nominal_field368 = match source_ml_source2_nominal_spread361.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value369, _, _) => {
                                source_ml_source2_data_property_value369
                            }
                        };
                        {
                            let source_ml_source2_nominal_field370 = match source_ml_source2_nominal_spread361.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value371, _) => {
                                    source_ml_source2_data_property_value371
                                }
                            };
                            {
                                let source_ml_source2_nominal_field372 = match source_ml_source2_nominal_spread361.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value373) => {
                                        source_ml_source2_data_property_value373.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_override374 = error.to_owned();
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field362, source_ml_source2_nominal_field364, source_ml_source2_nominal_field366, source_ml_source2_nominal_field368, source_ml_source2_nominal_field370, source_ml_source2_nominal_override374.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
    }), 0f64)
}
