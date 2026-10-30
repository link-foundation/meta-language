// Generated from complete js/src/translation/string-number-rules.js. Do not edit by hand.
use super::MlSource1TemporaryAcceptStringNumberInfinityLoop17Result;
use super::MlSource1TemporaryConvertStringNumberRationalJoin9;
use super::MlSource1TemporaryConvertStringNumberRationalJoin10;
use super::MlSource1TemporaryConvertStringNumberRationalLoop11Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop12Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop15Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop16Result;
use super::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result;
use super::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result;
use super::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Result;
use super::source_ml_source1_temporary_accept_string_number_infinity_loop17;
use super::source_ml_source1_temporary_convert_string_number_rational_loop11;
use super::source_ml_source1_temporary_convert_string_number_rational_loop12;
use super::source_ml_source1_temporary_convert_string_number_rational_loop13;
use super::source_ml_source1_temporary_convert_string_number_rational_loop14;
use super::source_ml_source1_temporary_convert_string_number_rational_loop15;
use super::source_ml_source1_temporary_convert_string_number_rational_loop16;
use super::source_ml_source1_temporary_read_java_script_utf8_string_units_loop1;
use super::source_ml_source1_temporary_read_java_script_utf8_string_units_loop2;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop7;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop8;
use super::source_ml_source1_temporary_read_string_number_decimal_power_loop5;
use super::source_ml_source1_temporary_read_string_number_exact_mantissa_loop6;

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
            for limb in &mut remainder {
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

        #[must_use]
        pub fn zero() -> Self {
            Self::from_parts(false, Vec::new())
        }

        #[must_use]
        pub fn from_i128(value: i128) -> Self {
            let mut big = Self::from_u128(value.unsigned_abs());
            big.negative = value < 0;
            big
        }

        #[must_use]
        pub fn from_u128(mut value: u128) -> Self {
            let mut magnitude = Vec::new();
            while value > 0 {
                magnitude.push(value as u32);
                value >>= 32;
            }
            Self::from_parts(false, magnitude)
        }

        /// A decimal literal with an optional leading minus sign.
        #[must_use]
        pub fn parse(text: &str) -> Self {
            let (negative, digits) = text.strip_prefix('-').map_or((false, text), |rest| (true, rest));
            let ten = Self::from_i128(10);
            let mut value = Self::zero();
            for digit in digits.chars() {
                let digit = digit.to_digit(10).expect("decimal literal");
                value = value.mul(&ten).add(&Self::from_i128(i128::from(digit)));
            }
            if negative { value.neg() } else { value }
        }

        #[must_use]
        pub const fn is_zero(&self) -> bool {
            self.magnitude.is_empty()
        }

        #[must_use]
        pub const fn is_negative(&self) -> bool {
            self.negative
        }

        /// The array index this names, if it is one.
        #[must_use]
        pub fn to_index(&self) -> Option<usize> {
            if self.negative || self.magnitude.len() > 2 {
                return None;
            }
            let value = self.magnitude.iter().rev().fold(0u64, |acc, &digit| (acc << 32) | u64::from(digit));
            usize::try_from(value).ok()
        }

        #[must_use]
        pub fn neg(&self) -> Self {
            Self::from_parts(!self.negative, self.magnitude.clone())
        }

        #[must_use]
        pub fn add(&self, other: &Self) -> Self {
            if self.negative == other.negative {
                return Self::from_parts(self.negative, add_magnitude(&self.magnitude, &other.magnitude));
            }
            match compare_magnitude(&self.magnitude, &other.magnitude) {
                Ordering::Less => Self::from_parts(other.negative, sub_magnitude(&other.magnitude, &self.magnitude)),
                _ => Self::from_parts(self.negative, sub_magnitude(&self.magnitude, &other.magnitude)),
            }
        }

        #[must_use]
        pub fn sub(&self, other: &Self) -> Self {
            self.add(&other.neg())
        }

        #[must_use]
        pub fn mul(&self, other: &Self) -> Self {
            Self::from_parts(self.negative != other.negative, mul_magnitude(&self.magnitude, &other.magnitude))
        }

        /// Division rounding toward zero; the divisor is not zero.
        #[must_use]
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
                std::fmt::Write::write_fmt(&mut text, format_args!("{chunk:09}")).expect("writing to a String cannot fail");
            }
            formatter.write_str(&text)
        }
    }

    /// Natural subtraction: truncated at zero.
    #[must_use]
    pub fn nat_sub(a: &Big, b: &Big) -> Big {
        if a > b { a.sub(b) } else { Big::zero() }
    }

    #[must_use]
    pub fn pred(a: &Big) -> Big {
        a.sub(&Big::from_i128(1))
    }

    /// Integer division with the source's rounding ("trunc", "floor" or
    /// "euclid"); by zero it aborts with the source's message or is total
    /// (x / 0 = 0, x % 0 = x).
    #[must_use]
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

    #[must_use]
    pub fn to_nat_checked(value: Big, message: &str) -> Big {
        assert!(!value.is_negative(), "{message}");
        value
    }

    #[must_use]
    pub fn clamp_nat(value: Big) -> Big {
        if value.is_negative() { Big::zero() } else { value }
    }

    /// Bounded domains for executable theorem checks.
    #[must_use]
    pub fn range(low: i128, high: i128) -> Vec<Big> {
        (low..=high).map(Big::from_i128).collect()
    }
}

/// The Math functions that Rust's f64 methods do not share with JavaScript.
pub mod ml_math {
    /// Math.round: the nearest integer, the one towards +Infinity on a tie,
    /// with the sign of a zero result from the argument.
    #[must_use]
    pub fn round(x: f64) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        if x > 0.0 && x < 0.5 {
            return 0.0;
        }
        if (-0.5..0.0).contains(&x) {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 {
            floor + 1.0
        } else {
            floor
        }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where `f64::signum` does not.
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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

    #[must_use]
    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    #[must_use]
    pub fn min_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| min(a, b))
    }

    #[must_use]
    pub fn is_integer(x: f64) -> bool {
        x.is_finite() && x.trunc() == x
    }

    #[must_use]
    pub fn is_safe_integer(x: f64) -> bool {
        is_integer(x) && x.abs() <= 9_007_199_254_740_991.0
    }
}

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    #[must_use]
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    #[must_use]
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        index.and_then(|index| values.get(index)).map_or_else(|| panic!("array index out of range"), Clone::clone)
    }

    /// The index a Number names: a non-negative integer, -0 included.
    #[must_use]
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9_007_199_254_740_992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    #[must_use]
    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    const fn integer(value: f64) -> f64 { if value.is_nan() { 0.0 } else { value.trunc() } }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 { (length as f64 + value).max(0.0) as usize } else { value.min(length as f64) as usize }
    }
    #[must_use]
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len()); let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    #[must_use]
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
    #[must_use]
    pub fn includes<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    #[must_use]
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values { if !out.iter().any(|previous| equal(previous, &value)) { out.push(value); } }
        out
    }
    #[must_use]
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) { out[index].1 = value; }
            else { out.push((key, value)); }
        }
        out
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_whitespace_unit(unit: f64) -> bool {
    (((((((((((9f64..=13f64).contains(&unit)) || (unit == 32f64)) || (unit == 160f64)) || (unit == 5760f64)) || ((8192f64..=8202f64).contains(&unit))) || (unit == 8232f64)) || (unit == 8233f64)) || (unit == 8239f64)) || (unit == 8287f64)) || (unit == 12_288f64)) || (unit == 65_279f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_numeric_digit_unit(unit: f64) -> f64 {
    if (48f64..=57f64).contains(&unit) {
        unit - 48f64
    } else if (65f64..=70f64).contains(&unit) {
            unit - 55f64
        } else if (97f64..=102f64).contains(&unit) {
                unit - 87f64
            } else {
                -1f64
            }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_utf8_string_units(bytes: &[f64]) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s1 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop1(bytes, index);
            match source_ml_source2_s1 {
                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Done => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source2_s2 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop2(bytes, units, index_2);
                                match source_ml_source2_s2 {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Done(units_2) => {
                                        units_2
                                    }
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Return(source_ml_source1_temporary_result2) => {
                                        source_ml_source1_temporary_result2
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Return(source_ml_source1_temporary_result4) => {
                    source_ml_source1_temporary_result4
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_digit(unit: f64) -> f64 {
    self::read_java_script_numeric_digit_unit(unit)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_decimal_power(exponent: f64) -> self::ml::Big {
    {
        let value = self::ml::Big::from_i128(1);
        {
            let base = self::ml::Big::from_i128(10);
            {
                self::source_ml_source1_temporary_read_string_number_decimal_power_loop5(exponent, value, base)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_exact_mantissa(value: self::ml::Big) -> f64 {
    {
        let result = 0f64;
        {
            let scale = 1f64;
            {
                self::source_ml_source1_temporary_read_string_number_exact_mantissa_loop6(value.clone(), result, scale)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_binary_power(exponent: f64) -> f64 {
    {
        let result = 1f64;
        {
            let source_ml_source2_s3 = self::source_ml_source1_temporary_read_string_number_binary_power_loop7(exponent, result);
            match source_ml_source2_s3 {
                self::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Done(exponent_2, result_2) => {
                    {
                        self::source_ml_source1_temporary_read_string_number_binary_power_loop8(exponent_2, result_2)
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_string_number_rational(numerator: self::ml::Big, denominator: self::ml::Big) -> f64 {
    if (numerator < (self::ml::Big::from_i128(0))) || (denominator <= (self::ml::Big::from_i128(0))) {
        f64::NAN
    } else if numerator == (self::ml::Big::from_i128(0)) {
            0f64
        } else {
            {
                let exponent = 0f64;
                {
                    let source_ml_source2_s6 = if numerator >= denominator {
                        {
                            let scale = denominator.clone();
                            {
                                let source_ml_source2_s7 = self::source_ml_source1_temporary_convert_string_number_rational_loop15(numerator.clone(), exponent, scale);
                                match source_ml_source2_s7 {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop15Result::MlSource1TemporaryConvertStringNumberRationalLoop15Done(exponent_2) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Next(exponent_2)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop15Result::MlSource1TemporaryConvertStringNumberRationalLoop15Return(source_ml_source1_temporary_result15) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Return(source_ml_source1_temporary_result15)
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let scale_2 = numerator.clone();
                            {
                                let source_ml_source2_s8 = self::source_ml_source1_temporary_convert_string_number_rational_loop16(denominator.clone(), exponent, scale_2);
                                match source_ml_source2_s8 {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop16Result::MlSource1TemporaryConvertStringNumberRationalLoop16Done(exponent_3) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Next(exponent_3)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop16Result::MlSource1TemporaryConvertStringNumberRationalLoop16Return(source_ml_source1_temporary_result16) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Return(source_ml_source1_temporary_result16)
                                    }
                                }
                            }
                        }
                    };
                    match source_ml_source2_s6 {
                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Next(exponent_4) => {
                            {
                                let shift = if exponent_4 >= -1022f64 {
                                    52f64 - exponent_4
                                } else {
                                    1074f64
                                };
                                {
                                    let source_ml_source2_s9 = if shift >= 0f64 {
                                        {
                                            let index = 0f64;
                                            {
                                                let numerator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop13(numerator.clone(), shift, index);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin10::MlSource1TemporaryConvertStringNumberRationalJoin10Next(numerator_2, denominator.clone())
                                            }
                                        }
                                    } else {
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let denominator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop14(denominator.clone(), shift, index_2);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin10::MlSource1TemporaryConvertStringNumberRationalJoin10Next(numerator.clone(), denominator_2)
                                            }
                                        }
                                    };
                                    match source_ml_source2_s9 {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin10::MlSource1TemporaryConvertStringNumberRationalJoin10Next(numerator_3, denominator_3) => {
                                            {
                                                let quotient = self::ml::Big::from_i128(0);
                                                {
                                                    let remainder = numerator_3.clone();
                                                    {
                                                        let scale_3 = denominator_3.clone();
                                                        {
                                                            let weight = self::ml::Big::from_i128(1);
                                                            {
                                                                let source_ml_source2_s10 = self::source_ml_source1_temporary_convert_string_number_rational_loop11(remainder.clone(), scale_3, weight);
                                                                match source_ml_source2_s10 {
                                                                    self::MlSource1TemporaryConvertStringNumberRationalLoop11Result::MlSource1TemporaryConvertStringNumberRationalLoop11Done(scale_4, weight_2) => {
                                                                        {
                                                                            let source_ml_source2_s11 = self::source_ml_source1_temporary_convert_string_number_rational_loop12(quotient, remainder.clone(), scale_4.clone(), weight_2.clone());
                                                                            match source_ml_source2_s11 {
                                                                                self::MlSource1TemporaryConvertStringNumberRationalLoop12Result::MlSource1TemporaryConvertStringNumberRationalLoop12Done(quotient_2, remainder_2) => {
                                                                                    {
                                                                                        let twice = remainder_2.mul(&self::ml::Big::from_i128(2));
                                                                                        {
                                                                                            let quotient_4 = if (twice > denominator_3) || ((twice == denominator_3) && ((self::ml::divide(&quotient_2, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1)))) {
                                                                                                {
                                                                                                    quotient_2.add(&self::ml::Big::from_i128(1))
                                                                                                }
                                                                                            } else {
                                                                                                quotient_2.clone()
                                                                                            };
                                                                                            (self::read_string_number_exact_mantissa(quotient_4) * self::read_string_number_binary_power(-shift))
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
                        }
                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Return(source_ml_source1_temporary_result14) => {
                            source_ml_source1_temporary_result14
                        }
                    }
                }
            }
        }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_string_number_infinity(units: &[f64], start: f64, end: f64) -> bool {
    {
let ml_library_inequality_1 = (end - start) != 8f64 ;
if ml_library_inequality_1 {
        false
    } else {
        {
            let expected = vec![73f64, 110f64, 102f64, 105f64, 110f64, 105f64, 116f64, 121f64];
            {
                let index = 0f64;
                {
                    let source_ml_source2_s12 = self::source_ml_source1_temporary_accept_string_number_infinity_loop17(units, start, &(expected), index);
                    match source_ml_source2_s12 {
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop17Result::MlSource1TemporaryAcceptStringNumberInfinityLoop17Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop17Result::MlSource1TemporaryAcceptStringNumberInfinityLoop17Return(source_ml_source1_temporary_result17) => {
                            source_ml_source1_temporary_result17
                        }
                    }
                }
            }
        }
    }
}
}
