// Generated from complete js/src/resource-description-address-rules.js. Do not edit by hand.
use super::MlAcceptResourceAddressFutureVersionLoop9Result;
use super::MlAcceptResourceAddressVersionSixJoin3;
use super::MlProjectResourceAddressAuthorityLoop11Result;
use super::ml_accept_resource_address_future_version_loop8;
use super::ml_accept_resource_address_future_version_loop9;
use super::ml_accept_resource_address_version_six_loop4;
use super::ml_project_resource_address_authority_loop11;
use super::ml_project_resource_address_authority_loop12;
use super::ml_project_resource_address_authority_loop13;
use super::ml_project_resource_address_authority_loop14;
use super::ml_project_resource_address_authority_loop15;

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
pub fn read_resource_address_unit(units: &[f64], at: f64) -> f64 {
    (self::ml_array::optional_at(units, self::ml_array::number_index(at))).unwrap_or_else(|| -1f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_resource_address_hexadecimal(value: f64) -> bool {
    (((48f64..=57f64).contains(&value)) || ((65f64..=70f64).contains(&value))) || ((97f64..=102f64).contains(&value))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_resource_address_decimal_tail(units: &[f64], start: f64, end: f64) -> bool {
    {
        let parts = 0f64;
        {
            let position = start;
            {
                let ml_s1 = self::ml_accept_resource_address_decimal_tail_loop1(units, end, parts, position);
                match ml_s1 {
                    self::MlAcceptResourceAddressDecimalTailLoop1Result::MlAcceptResourceAddressDecimalTailLoop1Done(parts_2) => {
                        parts_2 == 4f64
                    }
                    self::MlAcceptResourceAddressDecimalTailLoop1Result::MlAcceptResourceAddressDecimalTailLoop1Return(ml_result1) => {
                        ml_result1
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_resource_address_version_six(units: &[f64], start: f64, end: f64) -> bool {
    {
        let position = start;
        {
            let groups = 0f64;
            {
                let compressed = false;
                {
                    let ml_s2 = if (self::read_resource_address_unit(units, position)) == 58f64 {
                        if ((position + 1f64) >= end) || ((self::read_resource_address_unit(units, (position + 1f64))) != 58f64) {
                            self::MlAcceptResourceAddressVersionSixJoin3::MlAcceptResourceAddressVersionSixJoin3Return(false)
                        } else {
                            {
                                let compressed_2 = true;
                                {
                                    let position_2 = position + 2f64;
                                    self::MlAcceptResourceAddressVersionSixJoin3::MlAcceptResourceAddressVersionSixJoin3Next(position_2, compressed_2)
                                }
                            }
                        }
                    } else {
                        self::MlAcceptResourceAddressVersionSixJoin3::MlAcceptResourceAddressVersionSixJoin3Next(position, compressed)
                    };
                    match ml_s2 {
                        self::MlAcceptResourceAddressVersionSixJoin3::MlAcceptResourceAddressVersionSixJoin3Next(position_3, compressed_3) => {
                            {
                                let ml_s3 = self::ml_accept_resource_address_version_six_loop4(units, end, position_3, groups, compressed_3);
                                match ml_s3 {
                                    self::MlAcceptResourceAddressVersionSixLoop4Result::MlAcceptResourceAddressVersionSixLoop4Done(groups_2, compressed_4) => {
                                        if compressed_4 {
                                            groups_2 < 8f64
                                        } else {
                                            groups_2 == 8f64
                                        }
                                    }
                                    self::MlAcceptResourceAddressVersionSixLoop4Result::MlAcceptResourceAddressVersionSixLoop4Return(ml_result4) => {
                                        ml_result4
                                    }
                                }
                            }
                        }
                        self::MlAcceptResourceAddressVersionSixJoin3::MlAcceptResourceAddressVersionSixJoin3Return(ml_result7) => {
                            ml_result7
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_resource_address_future_version(units: &[f64], start: f64, end: f64) -> bool {
    {
        let first = self::read_resource_address_unit(units, start);
        if (first != 86f64) && (first != 118f64) {
            false
        } else {
            {
                let position = start + 1f64;
                {
                    let position_2 = self::ml_accept_resource_address_future_version_loop8(units, end, position);
                    if ((position_2 == (start + 1f64)) || (position_2 >= end)) || ((self::read_resource_address_unit(units, position_2)) != 46f64) {
                        false
                    } else {
                        {
                            let position_3 = position_2 + 1f64;
                            if position_3 == end {
                                false
                            } else {
                                {
                                    let ml_s4 = self::ml_accept_resource_address_future_version_loop9(units, end, position_3);
                                    match ml_s4 {
                                        self::MlAcceptResourceAddressFutureVersionLoop9Result::MlAcceptResourceAddressFutureVersionLoop9Done => {
                                            true
                                        }
                                        self::MlAcceptResourceAddressFutureVersionLoop9Result::MlAcceptResourceAddressFutureVersionLoop9Return(ml_result9) => {
                                            ml_result9
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
pub fn project_resource_address_authority(units: &[f64]) -> Vec<f64> {
    if (units.len() as f64) > 65_536f64 {
        Vec::<f64>::new()
    } else {
        {
            let ml_values10 = units.to_vec();
            {
                let ml_index10 = self::ml::Big::from_i128(0);
                {
                    let ml_s5 = self::ml_project_resource_address_authority_loop11(&(ml_values10), ml_index10);
                    match ml_s5 {
                        self::MlProjectResourceAddressAuthorityLoop11Result::MlProjectResourceAddressAuthorityLoop11Done => {
                            {
                                let colon = 0f64;
                                {
                                    let colon_2 = self::ml_project_resource_address_authority_loop12(units, colon);
                                    if (((colon_2 + 2f64) >= (units.len() as f64)) || ((self::read_resource_address_unit(units, (colon_2 + 1f64))) != 47f64)) || ((self::read_resource_address_unit(units, (colon_2 + 2f64))) != 47f64) {
                                        Vec::<f64>::new()
                                    } else {
                                        {
                                            let end = colon_2 + 3f64;
                                            {
                                                let end_2 = self::ml_project_resource_address_authority_loop13(units, end);
                                                {
                                                    let host = colon_2 + 3f64;
                                                    {
                                                        let at = host;
                                                        {
                                                            let host_2 = self::ml_project_resource_address_authority_loop14(units, end_2, host, at);
                                                            {
let ml_library_inequality_1 = (self::read_resource_address_unit(units, host_2)) != 91f64 ;
if ml_library_inequality_1 {
                                                                Vec::<f64>::new()
                                                            } else {
                                                                {
                                                                    let close = host_2 + 1f64;
                                                                    {
                                                                        let close_2 = self::ml_project_resource_address_authority_loop15(units, end_2, close);
                                                                        if (
 ((close_2 == end_2) || (close_2 == (host_2 + 1f64))) || (((close_2 + 1f64) < end_2) && ((self::read_resource_address_unit(units, (close_2 + 1f64))) != 58f64)) 
) || (
 !(self::accept_resource_address_version_six(units, (host_2 + 1f64), close_2)) && !(self::accept_resource_address_future_version(units, (host_2 + 1f64), close_2)) 
) {
                                                                                Vec::<f64>::new()
                                                                            } else {
                                                                                self::ml_array::append(self::ml_array::append(self::ml_array::slice(units, 0f64, host_2), vec![104f64]), self::ml_array::slice(units, (close_2 + 1f64), f64::INFINITY))
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
                        self::MlProjectResourceAddressAuthorityLoop11Result::MlProjectResourceAddressAuthorityLoop11Return(ml_result15) => {
                            ml_result15
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_resource_address_identifier(ordinary: bool, address: bool) -> bool {
    ordinary || address
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptResourceAddressDecimalTailLoop1Result {
    MlAcceptResourceAddressDecimalTailLoop1Done(f64),
    MlAcceptResourceAddressDecimalTailLoop1Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptResourceAddressDecimalTailLoop2Result {
    MlAcceptResourceAddressDecimalTailLoop2Done(f64, f64),
    MlAcceptResourceAddressDecimalTailLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_resource_address_decimal_tail_loop2(units: &[f64], mut end: f64, mut position: f64, mut beginning: f64, mut value: f64) -> self::MlAcceptResourceAddressDecimalTailLoop2Result {
    loop {
        return if (position < end) && ((self::read_resource_address_unit(units, position)) != 46f64) {
            {
                let digit = self::read_resource_address_unit(units, position) - 48f64;
                if ((digit < 0f64) || (digit > 9f64)) || ((position - beginning) >= 3f64) {
                    self::MlAcceptResourceAddressDecimalTailLoop2Result::MlAcceptResourceAddressDecimalTailLoop2Return(false)
                } else {
                    {
                        let value_2 = (value * 10f64) + digit;
                        {
                            let position_2 = position + 1f64;
                            {
                                (position, value) = (position_2, value_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptResourceAddressDecimalTailLoop2Result::MlAcceptResourceAddressDecimalTailLoop2Done(position, value)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_resource_address_decimal_tail_loop1(units: &[f64], mut end: f64, mut parts: f64, mut position: f64) -> self::MlAcceptResourceAddressDecimalTailLoop1Result {
    loop {
        return if position < end {
            {
                let beginning = position;
                {
                    let value = 0f64;
                    {
                        let ml_s6 = self::ml_accept_resource_address_decimal_tail_loop2(units, end, position, beginning, value);
                        match ml_s6 {
                            self::MlAcceptResourceAddressDecimalTailLoop2Result::MlAcceptResourceAddressDecimalTailLoop2Done(position_2, value_2) => {
                                if ((position_2 == beginning) || (value_2 > 255f64)) || (((position_2 - beginning) > 1f64) && ((self::read_resource_address_unit(units, beginning)) == 48f64)) {
                                    self::MlAcceptResourceAddressDecimalTailLoop1Result::MlAcceptResourceAddressDecimalTailLoop1Return(false)
                                } else {
                                    {
                                        let parts_2 = parts + 1f64;
                                        if position_2 == end {
                                            self::MlAcceptResourceAddressDecimalTailLoop1Result::MlAcceptResourceAddressDecimalTailLoop1Done(parts_2)
                                        } else {
                                            {
                                                let position_3 = position_2 + 1f64;
                                                if position_3 == end {
                                                    self::MlAcceptResourceAddressDecimalTailLoop1Result::MlAcceptResourceAddressDecimalTailLoop1Return(false)
                                                } else {
                                                    {
                                                        (parts, position) = (parts_2, position_3);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlAcceptResourceAddressDecimalTailLoop2Result::MlAcceptResourceAddressDecimalTailLoop2Return(ml_result2) => {
                                self::MlAcceptResourceAddressDecimalTailLoop1Result::MlAcceptResourceAddressDecimalTailLoop1Return(ml_result2)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptResourceAddressDecimalTailLoop1Result::MlAcceptResourceAddressDecimalTailLoop1Done(parts)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptResourceAddressVersionSixLoop4Result {
    MlAcceptResourceAddressVersionSixLoop4Done(f64, bool),
    MlAcceptResourceAddressVersionSixLoop4Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptResourceAddressVersionSixJoin6 {
    MlAcceptResourceAddressVersionSixJoin6Next(f64),
    MlAcceptResourceAddressVersionSixJoin6Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptResourceAddressVersionSixLoop7Result {
    MlAcceptResourceAddressVersionSixLoop7Done,
    MlAcceptResourceAddressVersionSixLoop7Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_resource_address_version_six_loop7(units: &[f64], mut position: f64, mut at: f64) -> self::MlAcceptResourceAddressVersionSixLoop7Result {
    loop {
        return if at < position {
            if (self::accept_resource_address_hexadecimal(self::read_resource_address_unit(units, at))) {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
{
                self::MlAcceptResourceAddressVersionSixLoop7Result::MlAcceptResourceAddressVersionSixLoop7Return(false)
            }
        } else {
            self::MlAcceptResourceAddressVersionSixLoop7Result::MlAcceptResourceAddressVersionSixLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptResourceAddressVersionSixLoop5Result {
    MlAcceptResourceAddressVersionSixLoop5Done(f64, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_resource_address_version_six_loop5(units: &[f64], mut end: f64, mut position: f64, mut decimal: bool) -> self::MlAcceptResourceAddressVersionSixLoop5Result {
    loop {
        if (position < end) && ((self::read_resource_address_unit(units, position)) != 58f64) {
            {
                let decimal_3 = if (self::read_resource_address_unit(units, position)) == 46f64 {
                    {
                        true
                    }
                } else {
                    decimal
                };
                {
                    let position_2 = position + 1f64;
                    {
                        (position, decimal) = (position_2, decimal_3);
                        continue;
                    }
                }
            }
        }
return {
            self::MlAcceptResourceAddressVersionSixLoop5Result::MlAcceptResourceAddressVersionSixLoop5Done(position, decimal)
        };
    }
}
