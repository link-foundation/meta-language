// Generated from resource-description-rules.js by the checked translator.
// Do not edit by hand.
#![allow(
    unused,
    unreachable_patterns,
    non_snake_case,
    non_camel_case_types,
    invalid_nan_comparisons,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
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
        a.len()
            .cmp(&b.len())
            .then_with(|| a.iter().rev().cmp(b.iter().rev()))
    }

    fn add_magnitude(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut out = Vec::with_capacity(a.len().max(b.len()) + 1);
        let mut carry = 0u64;
        for index in 0..a.len().max(b.len()) {
            let sum = u64::from(*a.get(index).unwrap_or(&0))
                + u64::from(*b.get(index).unwrap_or(&0))
                + carry;
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
            Self {
                negative: negative && !magnitude.is_empty(),
                magnitude,
            }
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
            let value = self
                .magnitude
                .iter()
                .rev()
                .fold(0u64, |acc, &digit| (acc << 32) | u64::from(digit));
            usize::try_from(value).ok()
        }

        pub fn neg(&self) -> Self {
            Self::from_parts(!self.negative, self.magnitude.clone())
        }

        pub fn add(&self, other: &Self) -> Self {
            if self.negative == other.negative {
                return Self::from_parts(
                    self.negative,
                    add_magnitude(&self.magnitude, &other.magnitude),
                );
            }
            match compare_magnitude(&self.magnitude, &other.magnitude) {
                Ordering::Less => Self::from_parts(
                    other.negative,
                    sub_magnitude(&other.magnitude, &self.magnitude),
                ),
                _ => Self::from_parts(
                    self.negative,
                    sub_magnitude(&self.magnitude, &other.magnitude),
                ),
            }
        }

        pub fn sub(&self, other: &Self) -> Self {
            self.add(&other.neg())
        }

        pub fn mul(&self, other: &Self) -> Self {
            Self::from_parts(
                self.negative != other.negative,
                mul_magnitude(&self.magnitude, &other.magnitude),
            )
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
                quotient = if b.is_negative() {
                    quotient.add(&one)
                } else {
                    quotient.sub(&one)
                };
            }
        }
        if remainder {
            a.sub(&quotient.mul(b))
        } else {
            quotient
        }
    }

    pub fn to_nat_checked(value: Big, message: &str) -> Big {
        if value.is_negative() {
            panic!("{message}");
        }
        value
    }

    pub fn clamp_nat(value: Big) -> Big {
        if value.is_negative() {
            Big::zero()
        } else {
            value
        }
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
        if x - floor >= 0.5 { floor + 1.0 } else { floor }
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
            if a.is_sign_negative() { b } else { a }
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
            if a.is_sign_negative() { a } else { b }
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
        is_integer(x) && x.abs() <= 9_007_199_254_740_991.0
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
        if index >= 0.0 && index.fract() == 0.0 && index < 9_007_199_254_740_992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    fn integer(value: f64) -> f64 {
        if value.is_nan() { 0.0 } else { value.trunc() }
    }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 {
            (length as f64 + value).max(0.0) as usize
        } else {
            value.min(length as f64) as usize
        }
    }
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len());
        let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> f64 {
        if values.is_empty() {
            return -1.0;
        }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) {
                return -1.0;
            }
            let start = if from < 0.0 {
                (values.len() as f64 + from) as usize
            } else {
                from.min((values.len() - 1) as f64) as usize
            };
            (0..=start)
                .rev()
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 {
                (values.len() as f64 + from).max(0.0) as usize
            } else {
                from.min(values.len() as f64) as usize
            };
            (start..values.len())
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        }
    }
    pub fn includes<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values {
            if !out.iter().any(|previous| equal(previous, &value)) {
                out.push(value);
            }
        }
        out
    }
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) {
                out[index].1 = value;
            } else {
                out.push((key, value));
            }
        }
        out
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn resource_description_unit(units: &[f64], index: f64) -> f64 {
    if (!(super::ml_math::is_safe_integer(index)) || (index < (0f64))) {
        (-1f64)
    } else {
        (super::ml_array::optional_at(units, super::ml_array::number_index(index)))
            .clone()
            .unwrap_or_else(|| (-1f64))
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn resource_description_letter(unit: f64) -> bool {
    (((unit >= (65f64)) && (unit <= (90f64))) || ((unit >= (97f64)) && (unit <= (122f64))))
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn resource_description_digit(unit: f64) -> bool {
    ((unit >= (48f64)) && (unit <= (57f64)))
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn resource_description_hexadecimal(unit: f64) -> bool {
    ((super::resource_description_digit(unit) || ((unit >= (65f64)) && (unit <= (70f64))))
        || ((unit >= (97f64)) && (unit <= (102f64))))
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn accept_resource_description_unicode(units: &[f64]) -> bool {
    {
        let index = 0f64;
        {
            let ml_s1 = super::ml_accept_resource_description_unicode_loop1(units, index);
            match ml_s1.clone() {
                super::MlAcceptResourceDescriptionUnicodeLoop1Result::MlAcceptResourceDescriptionUnicodeLoop1Done => {
                    true
                }
                super::MlAcceptResourceDescriptionUnicodeLoop1Result::MlAcceptResourceDescriptionUnicodeLoop1Return(ml_result1) => {
                    ml_result1
                }
            }
        }
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
/// Execute the checked shared resource-description algorithm.
#[must_use]
// The checked continuation explicitly carries its lexical state.
#[allow(clippy::too_many_lines)]
pub fn accept_resource_description_identifier_character(code: f64, query: bool) -> bool {
    if (super::resource_description_letter(code) || super::resource_description_digit(code)) {
        true
    } else {
        if ((((code == (45f64)) || (code == (46f64))) || (code == (95f64))) || (code == (126f64))) {
            true
        } else {
            if (((((((((((code == (33f64)) || (code == (36f64))) || (code == (38f64)))
                || (code == (39f64)))
                || (code == (40f64)))
                || (code == (41f64)))
                || (code == (42f64)))
                || (code == (43f64)))
                || (code == (44f64)))
                || (code == (59f64)))
                || (code == (61f64)))
            {
                true
            } else {
                if ((((code >= (160f64)) && (code <= (55295f64)))
                    || ((code >= (63744f64)) && (code <= (64975f64))))
                    || ((code >= (65008f64)) && (code <= (65519f64))))
                {
                    true
                } else {
                    if (((code >= (65536f64)) && (code <= (983_037f64)))
                        && ((code % 65536f64) <= (65533f64)))
                    {
                        true
                    } else {
                        (query
                            && (((code >= (57344f64)) && (code <= (63743f64)))
                                || (((code >= (983_040f64)) && (code <= (1_114_109f64)))
                                    && ((code % 65536f64) <= (65533f64)))))
                    }
                }
            }
        }
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
// The checked continuation explicitly carries its lexical state.
#[allow(clippy::too_many_lines)]
pub fn accept_resource_description_identifier(units: &[f64]) -> bool {
    if ((!(super::accept_resource_description_unicode(units)) || ((units.len() as f64) < (2f64)))
        || !(super::resource_description_letter(super::resource_description_unit(units, 0f64))))
    {
        false
    } else {
        {
            let start = 1f64;
            {
                let ml_s2 = super::ml_accept_resource_description_identifier_loop2(units, start);
                match ml_s2.clone() {
                    super::MlAcceptResourceDescriptionIdentifierLoop2Result::MlAcceptResourceDescriptionIdentifierLoop2Done(start_2) => {
                        if (start_2 == ((units.len() as f64))) {
                            false
                        } else {
                            {
                                let start_3 = (start_2 + 1f64);
                                {
                                    let authority_start = (-1f64);
                                    {
                                        let authority_end = (-1f64);
                                        {
                                            let ml_s5 = if (((((start_3 + 1f64)) < ((units.len() as f64))) && ((super::resource_description_unit(units, start_3)) == (47f64))) && ((super::resource_description_unit(units, (start_3 + 1f64))) == (47f64))) {
                                                {
                                                    let authority_start_2 = (start_3 + 2f64);
                                                    {
                                                        let authority_end_2 = authority_start_2;
                                                        {
                                                            let authority_end_3 = super::ml_accept_resource_description_identifier_loop6(units, authority_end_2);
                                                            {
                                                                let host = authority_start_2;
                                                                {
                                                                    let found_user = false;
                                                                    {
                                                                        let index = authority_start_2;
                                                                        {
                                                                            let ml_s6 = super::ml_accept_resource_description_identifier_loop7(units, authority_end_3, host, found_user, index);
                                                                            match ml_s6.clone() {
                                                                                super::MlAcceptResourceDescriptionIdentifierLoop7Result::MlAcceptResourceDescriptionIdentifierLoop7Done(host_2) => {
                                                                                    {
                                                                                        let found_port = false;
                                                                                        {
                                                                                            let index_2 = host_2;
                                                                                            {
                                                                                                let ml_s7 = super::ml_accept_resource_description_identifier_loop8(units, authority_end_3, found_port, index_2);
                                                                                                match ml_s7.clone() {
                                                                                                    super::MlAcceptResourceDescriptionIdentifierLoop8Result::MlAcceptResourceDescriptionIdentifierLoop8Done => {
                                                                                                        super::MlAcceptResourceDescriptionIdentifierJoin3::MlAcceptResourceDescriptionIdentifierJoin3Next
                                                                                                    }
                                                                                                    super::MlAcceptResourceDescriptionIdentifierLoop8Result::MlAcceptResourceDescriptionIdentifierLoop8Return(ml_result8) => {
                                                                                                        super::MlAcceptResourceDescriptionIdentifierJoin3::MlAcceptResourceDescriptionIdentifierJoin3Return(ml_result8)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                super::MlAcceptResourceDescriptionIdentifierLoop7Result::MlAcceptResourceDescriptionIdentifierLoop7Return(ml_result8_2) => {
                                                                                    super::MlAcceptResourceDescriptionIdentifierJoin3::MlAcceptResourceDescriptionIdentifierJoin3Return(ml_result8_2)
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
                                                super::MlAcceptResourceDescriptionIdentifierJoin3::MlAcceptResourceDescriptionIdentifierJoin3Next
                                            };
                                            match ml_s5.clone() {
                                                super::MlAcceptResourceDescriptionIdentifierJoin3::MlAcceptResourceDescriptionIdentifierJoin3Next => {
                                                    {
                                                        let query = false;
                                                        {
                                                            let fragment = false;
                                                            {
                                                                let index_3 = start_3;
                                                                {
                                                                    let ml_s8 = super::ml_accept_resource_description_identifier_loop4(units, query, fragment, index_3);
                                                                    match ml_s8.clone() {
                                                                        super::MlAcceptResourceDescriptionIdentifierLoop4Result::MlAcceptResourceDescriptionIdentifierLoop4Done => {
                                                                            true
                                                                        }
                                                                        super::MlAcceptResourceDescriptionIdentifierLoop4Result::MlAcceptResourceDescriptionIdentifierLoop4Return(ml_result4) => {
                                                                            ml_result4
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                super::MlAcceptResourceDescriptionIdentifierJoin3::MlAcceptResourceDescriptionIdentifierJoin3Return(ml_result5) => {
                                                    ml_result5
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    super::MlAcceptResourceDescriptionIdentifierLoop2Result::MlAcceptResourceDescriptionIdentifierLoop2Return(ml_result8_3) => {
                        ml_result8_3
                    }
                }
            }
        }
    }
}
