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
    use std::fmt::Write as _;

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
            let (negative, digits) = text
                .strip_prefix('-')
                .map_or((false, text), |rest| (true, rest));
            let ten = Self::from_i128(10);
            let mut value = Self::zero();
            for digit in digits.chars() {
                let digit = digit.to_digit(10).expect("decimal literal");
                value = value.mul(&ten).add(&Self::from_i128(i128::from(digit)));
            }
            if negative { value.neg() } else { value }
        }

        pub const fn is_zero(&self) -> bool {
            self.magnitude.is_empty()
        }

        pub const fn is_negative(&self) -> bool {
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
                write!(text, "{chunk:09}").expect("formatting a String succeeds");
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
        assert!(!value.is_negative(), "{message}");
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
        if (-0.5..0.0).contains(&x) {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 { floor + 1.0 } else { floor }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where `f64::signum` does not.
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

pub use crate::translation::frontend_rules::ml_array;

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn resource_description_unit(units: &[f64], index: f64) -> f64 {
    if (!(super::ml_math::is_safe_integer(index)) || (index < (0f64))) {
        (-1f64)
    } else {
        (super::ml_array::optional_at(units, super::ml_array::number_index(index))).unwrap_or(-1f64)
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn resource_description_letter(unit: f64) -> bool {
    (((65f64..=90f64).contains(&unit)) || ((97f64..=122f64).contains(&unit)))
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn resource_description_digit(unit: f64) -> bool {
    ((48f64..=57f64).contains(&unit))
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn resource_description_hexadecimal(unit: f64) -> bool {
    ((super::resource_description_digit(unit) || ((65f64..=70f64).contains(&unit)))
        || ((97f64..=102f64).contains(&unit)))
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn accept_resource_description_unicode(units: &[f64]) -> bool {
    {
        let index = 0f64;
        {
            let ml_s1 = super::ml_accept_resource_description_unicode_loop1(units, index);
            match ml_s1 {
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
pub fn accept_resource_description_identifier_character(code: f64, query: bool) -> bool {
    if (super::resource_description_letter(code) || super::resource_description_digit(code))
        || (((((code == (45f64)) || (code == (46f64))) || (code == (95f64))) || (code == (126f64)))
            || ((((((((((((code == (33f64)) || (code == (36f64))) || (code == (38f64)))
                || (code == (39f64)))
                || (code == (40f64)))
                || (code == (41f64)))
                || (code == (42f64)))
                || (code == (43f64)))
                || (code == (44f64)))
                || (code == (59f64)))
                || (code == (61f64)))
                || (((((160f64..=55295f64).contains(&code))
                    || ((63744f64..=64975f64).contains(&code)))
                    || ((65008f64..=65519f64).contains(&code)))
                    || (((65536f64..=983_037f64).contains(&code))
                        && ((code % 65536f64) <= (65533f64))))))
    {
        true
    } else {
        (query
            && (((57344f64..=63743f64).contains(&code))
                || (((983_040f64..=1_114_109f64).contains(&code))
                    && ((code % 65536f64) <= (65533f64)))))
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
                match ml_s2 {
                    super::MlAcceptResourceDescriptionIdentifierLoop2Result::MlAcceptResourceDescriptionIdentifierLoop2Done(start_2) => {
                        if (start_2 == (units.len() as f64)) {
                            false
                        } else {
                            {
                                let start_3 = (start_2 + 1f64);
                                {
                                    let authority_start = (-1f64);
                                    {
                                        let authority_end = (-1f64);
                                        {
                                            let ml_s5 = if ((((start_3 + 1f64) < (units.len() as f64)) && ((super::resource_description_unit(units, start_3)) == (47f64))) && ((super::resource_description_unit(units, start_3 + 1f64)) == (47f64))) {
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
                                                                            match ml_s6 {
                                                                                super::MlAcceptResourceDescriptionIdentifierLoop7Result::MlAcceptResourceDescriptionIdentifierLoop7Done(host_2) => {
                                                                                    {
                                                                                        let found_port = false;
                                                                                        {
                                                                                            let index_2 = host_2;
                                                                                            {
                                                                                                let ml_s7 = super::ml_accept_resource_description_identifier_loop8(units, authority_end_3, found_port, index_2);
                                                                                                match ml_s7 {
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
                                            match ml_s5 {
                                                super::MlAcceptResourceDescriptionIdentifierJoin3::MlAcceptResourceDescriptionIdentifierJoin3Next => {
                                                    {
                                                        let query = false;
                                                        {
                                                            let fragment = false;
                                                            {
                                                                let index_3 = start_3;
                                                                {
                                                                    let ml_s8 = super::ml_accept_resource_description_identifier_loop4(units, query, fragment, index_3);
                                                                    match ml_s8 {
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

/// Execute the checked shared resource-description algorithm.
#[must_use]
// The checked continuation explicitly carries its lexical state.
#[allow(clippy::too_many_lines)]
pub fn accept_resource_description_language(units: &[f64], grandfathered: bool) -> bool {
    if grandfathered {
        true
    } else {
        {
            let starts = vec![0f64];
            {
                let lengths = Vec::<f64>::new();
                {
                    let index = 0f64;
                    {
                        let ml_s9 = super::ml_accept_resource_description_language_loop11(
                            units, starts, lengths, index,
                        );
                        match ml_s9 {
                            super::MlAcceptResourceDescriptionLanguageLoop11Result::MlAcceptResourceDescriptionLanguageLoop11Done(starts_2, lengths_2) => {
                                {
                                    let part = 0f64;
                                    {
                                        let ml_s13 = if (((super::resource_description_unit(&lengths_2, 0f64)) == (1f64)) && (((super::resource_description_unit(units, 0f64)) == (88f64)) || ((super::resource_description_unit(units, 0f64)) == (120f64)))) {
                                            {
                                                let part_2 = 1f64;
                                                super::MlAcceptResourceDescriptionLanguageJoin12::MlAcceptResourceDescriptionLanguageJoin12Next(part_2)
                                            }
                                        } else if ((super::resource_description_unit(&lengths_2, 0f64)) < (2f64)) {
                                                super::MlAcceptResourceDescriptionLanguageJoin12::MlAcceptResourceDescriptionLanguageJoin12Return(false)
                                            } else {
                                                {
                                                    let index_2 = 0f64;
                                                    {
                                                        let ml_s14 = super::ml_accept_resource_description_language_loop13(units, lengths_2.clone(), index_2);
                                                        match ml_s14 {
                                                            super::MlAcceptResourceDescriptionLanguageLoop13Result::MlAcceptResourceDescriptionLanguageLoop13Done => {
                                                                {
                                                                    let part_3 = 1f64;
                                                                    {
                                                                        let part_5 = if ((super::resource_description_unit(&lengths_2, 0f64)) <= (3f64)) {
                                                                            {
                                                                                let count = 0f64;
                                                                                { super::ml_accept_resource_description_language_loop14(units, starts_2.clone(), lengths_2.clone(), part_3, count) }
                                                                            }
                                                                        } else {
                                                                            part_3
                                                                        };
                                                                        {
                                                                            let part_7 = if ((part_5 < (starts_2.len() as f64)) && ((super::resource_description_unit(&lengths_2, part_5)) == (4f64))) {
                                                                                {
                                                                                    let letters = true;
                                                                                    {
                                                                                        let index_3 = super::resource_description_unit(&starts_2, part_5);
                                                                                        {
                                                                                            let letters_2 = super::ml_accept_resource_description_language_loop16(units, starts_2.clone(), part_5, letters, index_3);
                                                                                            if letters_2 {
                                                                                                { (part_5 + 1f64) }
                                                                                            } else {
                                                                                                part_5
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                part_5
                                                                            };
                                                                            {
                                                                                let part_9 = if ((part_7 < (starts_2.len() as f64)) && (((((super::resource_description_unit(&lengths_2, part_7)) == (2f64)) && super::resource_description_letter(super::resource_description_unit(units, super::resource_description_unit(&starts_2, part_7)))) && super::resource_description_letter(super::resource_description_unit(units, super::resource_description_unit(&starts_2, part_7) + 1f64))) || (((((super::resource_description_unit(&lengths_2, part_7)) == (3f64)) && super::resource_description_digit(super::resource_description_unit(units, super::resource_description_unit(&starts_2, part_7)))) && super::resource_description_digit(super::resource_description_unit(units, super::resource_description_unit(&starts_2, part_7) + 1f64))) && super::resource_description_digit(super::resource_description_unit(units, super::resource_description_unit(&starts_2, part_7) + 2f64))))) {
                                                                                    { (part_7 + 1f64) }
                                                                                } else {
                                                                                    part_7
                                                                                };
                                                                                {
                                                                                    let variants = Vec::<f64>::new();
                                                                                    {
                                                                                        let ml_s15 = super::ml_accept_resource_description_language_loop17(units, starts_2.clone(), lengths_2.clone(), part_9, variants);
                                                                                        match ml_s15 {
                                                                                            super::MlAcceptResourceDescriptionLanguageLoop17Result::MlAcceptResourceDescriptionLanguageLoop17Done(part_10) => {
                                                                                                {
                                                                                                    let singletons = Vec::<f64>::new();
                                                                                                    {
                                                                                                        let ml_s16 = super::ml_accept_resource_description_language_loop18(units, starts_2.clone(), lengths_2.clone(), part_10, singletons);
                                                                                                        match ml_s16 {
                                                                                                            super::MlAcceptResourceDescriptionLanguageLoop18Result::MlAcceptResourceDescriptionLanguageLoop18Done(part_11) => {
                                                                                                                if (part_11 == (starts_2.len() as f64)) {
                                                                                                                    super::MlAcceptResourceDescriptionLanguageJoin12::MlAcceptResourceDescriptionLanguageJoin12Return(true)
                                                                                                                } else if (((super::resource_description_unit(&lengths_2, part_11)) != (1f64)) || (((super::resource_description_unit(units, super::resource_description_unit(&starts_2, part_11))) != (88f64)) && ((super::resource_description_unit(units, super::resource_description_unit(&starts_2, part_11))) != (120f64)))) {
                                                                                                                        super::MlAcceptResourceDescriptionLanguageJoin12::MlAcceptResourceDescriptionLanguageJoin12Return(false)
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let part_12 = (part_11 + 1f64);
                                                                                                                            super::MlAcceptResourceDescriptionLanguageJoin12::MlAcceptResourceDescriptionLanguageJoin12Next(part_12)
                                                                                                                        }
                                                                                                                    }
                                                                                                            }
                                                                                                            super::MlAcceptResourceDescriptionLanguageLoop18Result::MlAcceptResourceDescriptionLanguageLoop18Return(ml_result18) => {
                                                                                                                super::MlAcceptResourceDescriptionLanguageJoin12::MlAcceptResourceDescriptionLanguageJoin12Return(ml_result18)
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            super::MlAcceptResourceDescriptionLanguageLoop17Result::MlAcceptResourceDescriptionLanguageLoop17Return(ml_result20) => {
                                                                                                super::MlAcceptResourceDescriptionLanguageJoin12::MlAcceptResourceDescriptionLanguageJoin12Return(ml_result20)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            super::MlAcceptResourceDescriptionLanguageLoop13Result::MlAcceptResourceDescriptionLanguageLoop13Return(ml_result22) => {
                                                                super::MlAcceptResourceDescriptionLanguageJoin12::MlAcceptResourceDescriptionLanguageJoin12Return(ml_result22)
                                                            }
                                                        }
                                                    }
                                                }
                                            };
                                        match ml_s13 {
                                            super::MlAcceptResourceDescriptionLanguageJoin12::MlAcceptResourceDescriptionLanguageJoin12Next(next_part_position) => {
                                                (next_part_position < (starts_2.len() as f64))
                                            }
                                            super::MlAcceptResourceDescriptionLanguageJoin12::MlAcceptResourceDescriptionLanguageJoin12Return(ml_result12) => {
                                                ml_result12
                                            }
                                        }
                                    }
                                }
                            }
                            super::MlAcceptResourceDescriptionLanguageLoop11Result::MlAcceptResourceDescriptionLanguageLoop11Return(ml_result22_2) => {
                                ml_result22_2
                            }
                        }
                    }
                }
            }
        }
    }
}
