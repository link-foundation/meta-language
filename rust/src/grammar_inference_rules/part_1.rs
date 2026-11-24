// Generated from complete js/src/grammar-inference-rules.js. Do not edit by hand.
use super::MlAcceptInferenceDistributionsLoop9Result;
use super::MlAcceptInferenceTableShapeLoop25Result;
use super::MlAcceptInferenceTableShapeLoop26Result;
use super::MlAcceptInferredSymbolSequenceLoop23Result;
use super::MlApproximateInferenceLogarithmLoop6Result;
use super::MlApproximateInferenceLogarithmLoop7Result;
use super::MlFoldInferenceStatesLoop11Result;
use super::ml_accept_inference_distributions_loop9;
use super::ml_accept_inference_table_shape_loop25;
use super::ml_accept_inference_table_shape_loop26;
use super::ml_accept_inferred_symbol_sequence_loop23;
use super::ml_approximate_inference_logarithm_loop6;
use super::ml_approximate_inference_logarithm_loop7;
use super::ml_approximate_inference_logarithm_loop8;
use super::ml_build_inference_prefix_tree_loop3;
use super::ml_fold_inference_states_loop11;
use super::ml_fold_inference_states_loop12;
use super::ml_infer_regular_grammar_table_loop17;
use super::ml_read_inference_blue_states_loop15;
use super::ml_read_inference_blue_states_loop16;

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
pub fn read_inference_cell(table: &[f64], at: f64) -> f64 {
    (self::ml_array::optional_at(table, self::ml_array::number_index(at))).unwrap_or_else(|| 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn replace_inference_cell(table: &[f64], at: f64, value: f64) -> Vec<f64> {
    { let ml_values1 = table.to_vec(); let ml_callback1 = { let at = at; let value = value; std::rc::Rc::new(move |cell: f64, index: f64| -> f64 { if index == at {
        value
    } else {
        cell
    } }) }; ml_values1.iter().cloned().enumerate().map(|(ml_index1, ml_value1)| ml_callback1(ml_value1, (ml_index1 as f64))).collect::<Vec<_>>() }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_inference_representative(table: &[f64], state: f64) -> f64 {
    {
        let current = state;
        {
            let count = 0f64;
            {
                let ml_s1 = self::ml_find_inference_representative_loop1(table, current, count);
                match ml_s1 {
                    self::MlFindInferenceRepresentativeLoop1Result::MlFindInferenceRepresentativeLoop1Done => {
                        panic!("{}", "grammar inference parent cycle")
                    }
                    self::MlFindInferenceRepresentativeLoop1Result::MlFindInferenceRepresentativeLoop1Return(ml_result1) => {
                        ml_result1
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_inference_transition(table: &[f64], state: f64, symbol: f64) -> f64 {
    {
        let at = 0f64;
        {
            let ml_s2 = self::ml_find_inference_transition_loop2(table, state, symbol, at);
            match ml_s2 {
                self::MlFindInferenceTransitionLoop2Result::MlFindInferenceTransitionLoop2Done => {
                    -1f64
                }
                self::MlFindInferenceTransitionLoop2Result::MlFindInferenceTransitionLoop2Return(ml_result2) => {
                    ml_result2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn build_inference_prefix_tree(samples: &[f64], alphabet_size: f64, state_limit: f64) -> Vec<f64> {
    {
        let table = vec![0f64, 0f64, 0f64, 0f64, 0f64, 0f64, 1f64, 0f64];
        {
            let position = 0f64;
            {
                let states = 1f64;
                {
                    let ml_s3 = self::ml_build_inference_prefix_tree_loop3(samples, alphabet_size, state_limit, table, position, states);
                    match ml_s3 {
                        self::MlBuildInferencePrefixTreeLoop3Result::MlBuildInferencePrefixTreeLoop3Done(table_2) => {
                            table_2
                        }
                        self::MlBuildInferencePrefixTreeLoop3Result::MlBuildInferencePrefixTreeLoop3Return(ml_result3) => {
                            ml_result3
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn approximate_inference_logarithm(value: f64) -> f64 {
    {
assert!(!(!(value > 0f64) || !(value.is_finite())), "{}", "grammar inference logarithm domain");

        {
            let scaled = value;
            {
                let exponent = 0f64;
                {
                    let ml_s4 = self::ml_approximate_inference_logarithm_loop6(scaled, exponent);
                    match ml_s4 {
                        self::MlApproximateInferenceLogarithmLoop6Result::MlApproximateInferenceLogarithmLoop6Done(scaled_2, exponent_2) => {
                            {
                                let ml_s5 = self::ml_approximate_inference_logarithm_loop7(scaled_2, exponent_2);
                                match ml_s5 {
                                    self::MlApproximateInferenceLogarithmLoop7Result::MlApproximateInferenceLogarithmLoop7Done(scaled_3, exponent_3) => {
                                        {
                                            let ratio = (scaled_3 - 1f64) / (scaled_3 + 1f64);
                                            {
                                                let square = ratio * ratio;
                                                {
                                                    let term = ratio;
                                                    {
                                                        let total = 0f64;
                                                        {
                                                            let index = 0f64;
                                                            {
                                                                let total_2 = self::ml_approximate_inference_logarithm_loop8(square, term, total, index);
                                                                ((2f64 * total_2) + (exponent_3 * 0.6931471805599453f64))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_inference_proportion(first: f64, first_count: f64, second: f64, second_count: f64, bound: f64) -> bool {
    if (first_count == 0f64) || (second_count == 0f64) {
        true
    } else {
        (((first / first_count) - (second / second_count)).abs()) <= (bound * ((1f64 / first_count.sqrt()) + (1f64 / second_count.sqrt())))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_inference_distributions(table: &[f64], first: f64, second: f64, bound: f64, pair_limit: f64) -> f64 {
    {
        let pending = vec![first, second];
        {
            let seen = Vec::<f64>::new();
            {
                let position = 0f64;
                {
                    let ml_s6 = self::ml_accept_inference_distributions_loop9(table, bound, pair_limit, pending, seen, position);
                    match ml_s6 {
                        self::MlAcceptInferenceDistributionsLoop9Result::MlAcceptInferenceDistributionsLoop9Done => {
                            1f64
                        }
                        self::MlAcceptInferenceDistributionsLoop9Result::MlAcceptInferenceDistributionsLoop9Return(ml_result9) => {
                            ml_result9
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn fold_inference_states(original: &[f64], target: f64, source: f64, pair_limit: f64) -> Vec<f64> {
    {
        let table = original.to_vec();
        {
            let evidence = 0f64;
            {
                let position = 0f64;
                {
                    let pending = vec![target, source];
                    {
                        let ml_s7 = self::ml_fold_inference_states_loop11(pair_limit, table, evidence, position, pending);
                        match ml_s7 {
                            self::MlFoldInferenceStatesLoop11Result::MlFoldInferenceStatesLoop11Done(table_2, evidence_2) => {
                                {
                                    let edge = 0f64;
                                    {
                                        let table_3 = self::ml_fold_inference_states_loop12(table_2, edge);
                                        { let (ml_append_right, ml_append_left) = (vec![2f64, target, source, evidence_2, 0f64, 0f64, 0f64, 0f64], table_3); self::ml_array::append(ml_append_left, ml_append_right) }
                                    }
                                }
                            }
                            self::MlFoldInferenceStatesLoop11Result::MlFoldInferenceStatesLoop11Return(ml_result12) => {
                                ml_result12
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
pub fn read_inference_blue_states(table: &[f64]) -> Vec<f64> {
    {
        let next = Vec::<f64>::new();
        {
            let edge = 0f64;
            {
                let next_2 = self::ml_read_inference_blue_states_loop15(table, next, edge);
                {
                    let result = Vec::<f64>::new();
                    {
                        let state = 0f64;
                        {
                            self::ml_read_inference_blue_states_loop16(table, &(next_2), result, state)
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn infer_regular_grammar_table(samples: &[f64], alphabet_size: f64, strategy: f64, alpha: f64, state_limit: f64, pair_limit: f64, attempt_limit: f64) -> Vec<f64> {
    if ((strategy != 0f64) && (strategy != 1f64)) && (strategy != 2f64) {
        vec![-7f64]
    } else if ((alpha > 0f64) && (alpha < 1f64)) {
            {
                let table = self::build_inference_prefix_tree(samples, alphabet_size, state_limit);
                {
                    let attempts = 0f64;
                    if (self::read_inference_cell(&(table), 0f64)) < 0f64 {
                        table
                    } else {
                        {
                            let bound = (0.5f64 * self::approximate_inference_logarithm(2f64 / (1f64 - alpha))).sqrt();
                            self::ml_infer_regular_grammar_table_loop17(strategy, pair_limit, attempt_limit, table, attempts, bound)
                        }
                    }
                }
            }
        } else {
            vec![-8f64]
        }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_inference_failure_reason(code: f64) -> String {
    if code == -1f64 {
        String::from("invalid labelled grammar inference sample")
    } else if code == -2f64 {
            String::from("invalid grammar inference symbol")
        } else if code == -3f64 {
                String::from("grammar inference state limit exceeded")
            } else if code == -4f64 {
                    String::from("contradictory positive and negative grammar inference samples")
                } else if code == -5f64 {
                        String::from("grammar inference pair limit exceeded")
                    } else if code == -6f64 {
                            String::from("grammar inference merge attempt limit exceeded")
                        } else if code == -7f64 {
                                String::from("unknown regular grammar inference strategy")
                            } else if code == -8f64 {
                                    String::from("grammar inference confidence must be between zero and one")
                                } else {
                                    String::from("invalid grammar inference result")
                                }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_inferred_symbol_sequence(table: &[f64], symbols: &[f64]) -> bool {
    {
        let state = self::find_inference_representative(table, 0f64);
        {
            let ml_values22 = symbols.to_vec();
            {
                let ml_index22 = self::ml::Big::from_i128(0);
                {
                    let ml_s8 = self::ml_accept_inferred_symbol_sequence_loop23(table, state, &(ml_values22), ml_index22);
                    match ml_s8 {
                        self::MlAcceptInferredSymbolSequenceLoop23Result::MlAcceptInferredSymbolSequenceLoop23Done(state_2) => {
                            ((self::read_inference_cell(table, (state_2 + 2f64))) == 1f64) && ((self::read_inference_cell(table, (state_2 + 3f64))) == 0f64)
                        }
                        self::MlAcceptInferredSymbolSequenceLoop23Result::MlAcceptInferredSymbolSequenceLoop23Return(ml_result23) => {
                            ml_result23
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_inference_table_shape(table: &[f64], alphabet_size: f64) -> bool {
    if ((((table.len() as f64) < 8f64) || ((table.len() as f64) > 524_288f64)) || (((table.len() as f64) % 8f64) != 0f64)) || ((self::read_inference_cell(table, 0f64)) != 0f64) {
        false
    } else {
        {
            let ml_values24 = table.to_vec();
            {
                let ml_index24 = self::ml::Big::from_i128(0);
                {
                    let ml_s9 = self::ml_accept_inference_table_shape_loop25(&(ml_values24), ml_index24);
                    match ml_s9 {
                        self::MlAcceptInferenceTableShapeLoop25Result::MlAcceptInferenceTableShapeLoop25Done => {
                            {
                                let at = 0f64;
                                {
                                    let ml_s10 = self::ml_accept_inference_table_shape_loop26(table, alphabet_size, at);
                                    match ml_s10 {
                                        self::MlAcceptInferenceTableShapeLoop26Result::MlAcceptInferenceTableShapeLoop26Done => {
                                            true
                                        }
                                        self::MlAcceptInferenceTableShapeLoop26Result::MlAcceptInferenceTableShapeLoop26Return(ml_result26) => {
                                            ml_result26
                                        }
                                    }
                                }
                            }
                        }
                        self::MlAcceptInferenceTableShapeLoop25Result::MlAcceptInferenceTableShapeLoop25Return(ml_result29) => {
                            ml_result29
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindInferenceRepresentativeLoop1Result {
    MlFindInferenceRepresentativeLoop1Done,
    MlFindInferenceRepresentativeLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_find_inference_representative_loop1(table: &[f64], mut current: f64, mut count: f64) -> self::MlFindInferenceRepresentativeLoop1Result {
    loop {
        return if count < (table.len() as f64) {
            {
                let parent = self::read_inference_cell(table, (current + 1f64));
                if (parent).partial_cmp(&(current)) == Some(std::cmp::Ordering::Equal) {
                    self::MlFindInferenceRepresentativeLoop1Result::MlFindInferenceRepresentativeLoop1Return(current)
                } else {
                    {
                        let current_2 = parent;
                        {
                            let count_2 = count + 8f64;
                            {
                                (current, count) = (current_2, count_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlFindInferenceRepresentativeLoop1Result::MlFindInferenceRepresentativeLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindInferenceTransitionLoop2Result {
    MlFindInferenceTransitionLoop2Done,
    MlFindInferenceTransitionLoop2Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_find_inference_transition_loop2(table: &[f64], mut state: f64, mut symbol: f64, mut at: f64) -> self::MlFindInferenceTransitionLoop2Result {
    loop {
        return if at < (table.len() as f64) {
            if (((self::read_inference_cell(table, at)) == 1f64) && ((self::read_inference_cell(table, (at + 1f64))) == state)) && ((self::read_inference_cell(table, (at + 2f64))) == symbol) {
                self::MlFindInferenceTransitionLoop2Result::MlFindInferenceTransitionLoop2Return(at)
            } else {
                {
                    let at_2 = at + 8f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlFindInferenceTransitionLoop2Result::MlFindInferenceTransitionLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlBuildInferencePrefixTreeLoop3Result {
    MlBuildInferencePrefixTreeLoop3Done(Vec<f64>),
    MlBuildInferencePrefixTreeLoop3Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlBuildInferencePrefixTreeLoop4Result {
    MlBuildInferencePrefixTreeLoop4Done(Vec<f64>, f64, f64),
    MlBuildInferencePrefixTreeLoop4Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlBuildInferencePrefixTreeJoin5 {
    MlBuildInferencePrefixTreeJoin5Next(Vec<f64>, f64, f64),
    MlBuildInferencePrefixTreeJoin5Return(Vec<f64>),
}
