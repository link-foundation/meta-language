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

pub fn read_rust_copy_closure_parameter_form(role: String) -> String {
    if (role == "_") {
        String::from("wildcard")
    } else {
        if (role == "(") {
            String::from("product")
        } else {
            if (role == "binding") {
                String::from("binding")
            } else {
                String::from("unsupported")
            }
        }
    }
}

pub fn read_rust_copy_closure_header_error(ordinary: bool, names: Vec<String>, copies: Vec<bool>, result_written: bool, result_copy: bool) -> String {
    {
        let error = self::read_rust_copy_closure_parameter_header_error(ordinary, names, copies);
        if !error.is_empty() {
            error.clone()
        } else {
            if !result_written {
                String::from("the closure result requires its actual explicit type")
            } else {
                if result_copy {
                    String::from("")
                } else {
                    String::from("the closure result requires a direct immutable Copy source type")
                }
            }
        }
    }
}

pub fn read_rust_copy_closure_capture_error(collecting: bool, copies: Vec<bool>, assigned: f64) -> String {
    if (assigned != (0f64)) {
        String::from("mutable closure captures require their shared ownership interpretation")
    } else {
        {
            let source_ml_source1_s2 = if !collecting {
                {
                    let index = 0f64;
                    {
                        let source_ml_source1_s3 = self::source_ml_temporary_read_rust_copy_closure_capture_error_loop2(copies, index);
                        match source_ml_source1_s3.clone() {
                            self::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Result::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Done => {
                                self::MlTemporaryReadRustCopyClosureCaptureErrorJoin1::MlTemporaryReadRustCopyClosureCaptureErrorJoin1Next
                            }
                            self::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Result::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Return(source_ml_temporary_result2) => {
                                self::MlTemporaryReadRustCopyClosureCaptureErrorJoin1::MlTemporaryReadRustCopyClosureCaptureErrorJoin1Return(source_ml_temporary_result2.clone())
                            }
                        }
                    }
                }
            } else {
                self::MlTemporaryReadRustCopyClosureCaptureErrorJoin1::MlTemporaryReadRustCopyClosureCaptureErrorJoin1Next
            };
            match source_ml_source1_s2.clone() {
                self::MlTemporaryReadRustCopyClosureCaptureErrorJoin1::MlTemporaryReadRustCopyClosureCaptureErrorJoin1Next => {
                    String::from("")
                }
                self::MlTemporaryReadRustCopyClosureCaptureErrorJoin1::MlTemporaryReadRustCopyClosureCaptureErrorJoin1Return(source_ml_temporary_result1) => {
                    source_ml_temporary_result1.clone()
                }
            }
        }
    }
}

pub fn accept_rust_copy_closure_value(ordinary: bool, names: Vec<String>, parameters: Vec<bool>, known: bool, result: bool, captures: Vec<bool>, loans: Vec<bool>, assigned: f64) -> bool {
    if ((((((self::read_rust_copy_closure_parameter_header_error(ordinary, names, parameters)) != "") || !known) || !result) || (assigned != (0f64))) || (((captures.len() as f64)) != ((loans.len() as f64)))) {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source1_s4 = self::source_ml_temporary_accept_rust_copy_closure_value_loop3(captures, loans, at);
                match source_ml_source1_s4.clone() {
                    self::MlTemporaryAcceptRustCopyClosureValueLoop3Result::MlTemporaryAcceptRustCopyClosureValueLoop3Done => {
                        true
                    }
                    self::MlTemporaryAcceptRustCopyClosureValueLoop3Result::MlTemporaryAcceptRustCopyClosureValueLoop3Return(source_ml_temporary_result3) => {
                        source_ml_temporary_result3
                    }
                }
            }
        }
    }
}

pub fn read_rust_copy_closure_loan_capture_error(captures: f64, loans: Vec<bool>) -> String {
    if (captures != ((loans.len() as f64))) {
        String::from("a Copy closure environment needs every actual capture loan fact")
    } else {
        {
            let source_ml_temporary_values4 = loans;
            {
                let source_ml_temporary_index4 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s5 = self::source_ml_temporary_read_rust_copy_closure_loan_capture_error_loop5(source_ml_temporary_values4, source_ml_temporary_index4.clone());
                    match source_ml_source1_s5.clone() {
                        self::MlTemporaryReadRustCopyClosureLoanCaptureErrorLoop5Result::MlTemporaryReadRustCopyClosureLoanCaptureErrorLoop5Done => {
                            String::from("")
                        }
                        self::MlTemporaryReadRustCopyClosureLoanCaptureErrorLoop5Result::MlTemporaryReadRustCopyClosureLoanCaptureErrorLoop5Return(source_ml_temporary_result5) => {
                            source_ml_temporary_result5.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_copy_closure_value_borrow_error(states: Vec<String>) -> String {
    {
        let source_ml_temporary_values6 = states;
        {
            let source_ml_temporary_index6 = self::ml::Big::from_i128(0);
            {
                let source_ml_source1_s6 = self::source_ml_temporary_read_rust_copy_closure_value_borrow_error_loop7(source_ml_temporary_values6, source_ml_temporary_index6.clone());
                match source_ml_source1_s6.clone() {
                    self::MlTemporaryReadRustCopyClosureValueBorrowErrorLoop7Result::MlTemporaryReadRustCopyClosureValueBorrowErrorLoop7Done => {
                        String::from("")
                    }
                    self::MlTemporaryReadRustCopyClosureValueBorrowErrorLoop7Result::MlTemporaryReadRustCopyClosureValueBorrowErrorLoop7Return(source_ml_temporary_result7) => {
                        source_ml_temporary_result7.clone()
                    }
                }
            }
        }
    }
}

pub fn read_rust_copy_closure_returned_borrow_error(borrows: f64) -> String {
    if (borrows == (0f64)) {
        String::from("")
    } else {
        String::from("returning a borrowed callable environment needs its explicit callable result lifetime contract")
    }
}

pub fn accept_rust_copy_closure_invocation(kind: String) -> bool {
    (kind == "function")
}

pub fn read_rust_copy_closure_join_error(first: bool, second: bool, same: bool) -> String {
    if ((first || second) && !same) {
        String::from("distinct anonymous closure types require an actual source coercion")
    } else {
        String::from("")
    }
}

pub fn read_rust_copy_closure_parameter_header_error(ordinary: bool, names: Vec<String>, copies: Vec<bool>) -> String {
    if !ordinary {
        String::from("closure ownership requires its separate active source profile")
    } else {
        if ((((names.len() as f64)) != ((copies.len() as f64))) || (((names.len() as f64)) > (1024f64))) {
            String::from("the complete typed closure header exceeds its source budget")
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source1_s7 = self::source_ml_temporary_read_rust_copy_closure_parameter_header_error_loop8(names, copies, index);
                    match source_ml_source1_s7.clone() {
                        self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop8Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop8Done => {
                            String::from("")
                        }
                        self::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop8Result::MlTemporaryReadRustCopyClosureParameterHeaderErrorLoop8Return(source_ml_temporary_result8) => {
                            source_ml_temporary_result8.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn test_rust_copy_closure_deferred_result(requested: bool, written: bool, known: bool) -> bool {
    ((requested && !written) && !known)
}

pub fn read_rust_copy_closure_deferred_source_error(kind: String, parameters: f64, layers: f64, identity: bool) -> String {
    if ((((kind == "lambda") && (parameters == (0f64))) && (layers == (0f64))) && identity) {
        String::from("")
    } else {
        String::from("a deferred fallback needs its actual zero-argument closure source and identity")
    }
}

pub fn read_rust_copy_closure_deferred_result_error(kind: String, pending: bool, known: bool, copy: bool) -> String {
    if ((((kind == "lambda") && pending) && known) && copy) {
        String::from("")
    } else {
        String::from("a deferred closure needs its actual complete immutable Copy result context")
    }
}

pub fn read_rust_copy_closure_result_context_role(kind: String, operation: String) -> String {
    if (kind == "let") {
        String::from("binding")
    } else {
        if (kind == "name") {
            String::from("name")
        } else {
            if (kind == "if") {
                String::from("branches")
            } else {
                if (kind == "match") {
                    String::from("rows")
                } else {
                    if ((kind == "unary") && (operation == "neg")) {
                        String::from("operand")
                    } else {
                        if ((kind == "binary") && self::ml_array::includes(&vec![String::from("add"), String::from("sub"), String::from("mul"), String::from("div"), String::from("rem")], &operation.clone(), 0f64, false, |a, b| a == b)) {
                            String::from("operands")
                        } else {
                            String::from("leaf")
                        }
                    }
                }
            }
        }
    }
}

pub fn read_rust_copy_closure_literal_result_error(kind: String, negative: bool, typed: bool, result: String, signed: bool) -> String {
    if ((negative && (result == "fixed")) && !signed) {
        String::from("an unsigned Rust result cannot use unary negation")
    } else {
        if (((kind == "num") && !typed) && (result != "fixed")) {
            String::from("an unsuffixed Rust integer literal needs an actual integer result context")
        } else {
            String::from("")
        }
    }
}

pub fn test_rust_copy_closure_result_binding(path: Vec<String>, name: String) -> bool {
    ((((path.len() as f64)) == (1f64)) && {
        let source_ml_source1_comparison_operand8 = self::ml_array::optional_at(&path, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand9 = name.clone();
            match (source_ml_source1_comparison_operand8.clone(), source_ml_source1_comparison_operand9.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
        }
    })
}

pub fn read_rust_copy_closure_reference_pattern_depth(token: String) -> f64 {
    if (token == "&") {
        1f64
    } else {
        if (token == "&&") {
            2f64
        } else {
            0f64
        }
    }
}

pub fn read_rust_copy_closure_reference_pattern_error(depth: f64, layers: f64, references: bool, parameter: bool) -> String {
    if (depth == (0f64)) {
        String::from("")
    } else {
        if (((((((self::ml_math::is_integer(depth) && (depth >= (1f64))) && (depth <= (256f64))) && self::ml_math::is_integer(layers)) && (layers >= depth)) && (layers <= (256f64))) && references) && parameter) {
            String::from("")
        } else {
            String::from("a closure reference pattern needs its actual immutable Copy scalar reference layers")
        }
    }
}

pub fn read_rust_copy_closure_pattern_binding_error(names: Vec<String>) -> String {
    if (((names.len() as f64)) > (1024f64)) {
        String::from("the complete typed closure header exceeds its source budget")
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s10 = self::source_ml_temporary_read_rust_copy_closure_pattern_binding_error_loop10(names, index);
                match source_ml_source1_s10.clone() {
                    self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop10Result::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop10Done => {
                        String::from("")
                    }
                    self::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop10Result::MlTemporaryReadRustCopyClosurePatternBindingErrorLoop10Return(source_ml_temporary_result10) => {
                        source_ml_temporary_result10.clone()
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosureCaptureErrorJoin1 {
    MlTemporaryReadRustCopyClosureCaptureErrorJoin1Next,
    MlTemporaryReadRustCopyClosureCaptureErrorJoin1Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosureCaptureErrorLoop2Result {
    MlTemporaryReadRustCopyClosureCaptureErrorLoop2Done,
    MlTemporaryReadRustCopyClosureCaptureErrorLoop2Return(String),
}

pub fn source_ml_temporary_read_rust_copy_closure_capture_error_loop2(mut copies: Vec<bool>, mut index: f64) -> self::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Result {
    loop {
        return if (index < ((copies.len() as f64))) {
            if !(self::read_required_boolean_fact(copies.clone(), index)) {
                self::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Result::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Return(String::from("closure captures require proven direct immutable Copy source values"))
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
            self::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Result::MlTemporaryReadRustCopyClosureCaptureErrorLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustCopyClosureValueLoop3Result {
    MlTemporaryAcceptRustCopyClosureValueLoop3Done,
    MlTemporaryAcceptRustCopyClosureValueLoop3Return(bool),
}

pub fn source_ml_temporary_accept_rust_copy_closure_value_loop3(mut captures: Vec<bool>, mut loans: Vec<bool>, mut at: f64) -> self::MlTemporaryAcceptRustCopyClosureValueLoop3Result {
    loop {
        return if (at < ((captures.len() as f64))) {
            if (!(self::read_required_boolean_fact(captures.clone(), at)) || {
                let source_ml_source1_comparison_operand11 = self::ml_array::optional_at(&loans, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand12 = false;
                    !(match (source_ml_source1_comparison_operand11.clone(), source_ml_source1_comparison_operand12) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            }) {
                self::MlTemporaryAcceptRustCopyClosureValueLoop3Result::MlTemporaryAcceptRustCopyClosureValueLoop3Return(false)
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
            self::MlTemporaryAcceptRustCopyClosureValueLoop3Result::MlTemporaryAcceptRustCopyClosureValueLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosureLoanCaptureErrorLoop5Result {
    MlTemporaryReadRustCopyClosureLoanCaptureErrorLoop5Done,
    MlTemporaryReadRustCopyClosureLoanCaptureErrorLoop5Return(String),
}

pub fn source_ml_temporary_read_rust_copy_closure_loan_capture_error_loop5(mut source_ml_temporary_values4: Vec<bool>, mut source_ml_temporary_index4: self::ml::Big) -> self::MlTemporaryReadRustCopyClosureLoanCaptureErrorLoop5Result {
    loop {
        return if (source_ml_temporary_index4 < (self::ml::Big::from_u128(source_ml_temporary_values4.len() as u128))) {
            {
                let loan = self::ml_array::at(&source_ml_temporary_values4, source_ml_temporary_index4.to_index());
                if (loan != (false)) {
                    self::MlTemporaryReadRustCopyClosureLoanCaptureErrorLoop5Result::MlTemporaryReadRustCopyClosureLoanCaptureErrorLoop5Return(String::from("a Copy closure environment cannot discard borrowed-place or local loan provenance"))
                } else {
                    {
                        let source_ml_temporary_index4_2 = source_ml_temporary_index4.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index4 = source_ml_temporary_index4_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustCopyClosureLoanCaptureErrorLoop5Result::MlTemporaryReadRustCopyClosureLoanCaptureErrorLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCopyClosureValueBorrowErrorLoop7Result {
    MlTemporaryReadRustCopyClosureValueBorrowErrorLoop7Done,
    MlTemporaryReadRustCopyClosureValueBorrowErrorLoop7Return(String),
}
