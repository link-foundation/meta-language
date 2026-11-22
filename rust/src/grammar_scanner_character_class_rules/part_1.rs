// Generated from complete js/src/grammar-scanner-character-class-rules.js. Do not edit by hand.
use super::MlTemporaryRenderGrammarScannerCharacterClassDefinitionLoop8Result;
use super::source_ml_temporary_render_grammar_scanner_character_class_definition_loop8;
use super::source_ml_temporary_render_grammar_scanner_character_class_members_loop4;
use super::source_ml_temporary_render_grammar_scanner_character_class_members_loop6;

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
            let value = self
                .magnitude
                .iter()
                .rev()
                .fold(0u64, |acc, &digit| (acc << 32) | u64::from(digit));
            usize::try_from(value).ok()
        }

        #[must_use]
        pub fn neg(&self) -> Self {
            Self::from_parts(!self.negative, self.magnitude.clone())
        }

        #[must_use]
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

        #[must_use]
        pub fn sub(&self, other: &Self) -> Self {
            self.add(&other.neg())
        }

        #[must_use]
        pub fn mul(&self, other: &Self) -> Self {
            Self::from_parts(
                self.negative != other.negative,
                mul_magnitude(&self.magnitude, &other.magnitude),
            )
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
                std::fmt::Write::write_fmt(&mut text, format_args!("{chunk:09}"))
                    .expect("writing to a String cannot fail");
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

    #[must_use]
    pub fn to_nat_checked(value: Big, message: &str) -> Big {
        assert!(!value.is_negative(), "{message}");
        value
    }

    #[must_use]
    pub fn clamp_nat(value: Big) -> Big {
        if value.is_negative() {
            Big::zero()
        } else {
            value
        }
    }

    /// Bounded domains for executable theorem checks.
    #[must_use]
    pub fn range(low: i128, high: i128) -> Vec<Big> {
        (low..=high).map(Big::from_i128).collect()
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
        index
            .and_then(|index| values.get(index))
            .map_or_else(|| panic!("array index out of range"), Clone::clone)
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
    const fn integer(value: f64) -> f64 {
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
    #[must_use]
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len());
        let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    #[must_use]
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
    #[must_use]
    pub fn includes<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    #[must_use]
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values {
            if !out.iter().any(|previous| equal(previous, &value)) {
                out.push(value);
            }
        }
        out
    }
    #[must_use]
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

pub mod ml_source_exception {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SourceException {
        pub class_name: String,
        pub message_units: Vec<u16>,
        pub message_present: bool,
    }
    impl SourceException {
        #[must_use]
        pub fn raise(self) -> ! {
            std::panic::panic_any((self.class_name, self.message_units, self.message_present))
        }
        #[must_use]
        pub fn from_text(class_name: &str, message: &str) -> Self {
            Self {
                class_name: class_name.to_owned(),
                message_units: message.encode_utf16().collect(),
                message_present: true,
            }
        }
    }
    impl std::fmt::Display for SourceException {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.message_units))
        }
    }
    impl std::error::Error for SourceException {}
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required Number fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required text fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required Number fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_number_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required text fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_text_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_grammar_scanner_character_classes(
    modes: &[String],
    spellings: &[String],
    tokens: &[String],
    next_modes: &[String],
    code_points: &[f64],
    plan: &[f64],
) -> Vec<String> {
    {
        let grouped = Vec::<f64>::new();
        {
            let matchers = Vec::<String>::new();
            {
                let position = 0f64;
                {
                    self::source_ml_temporary_plan_grammar_scanner_character_classes_loop1(
                        modes,
                        spellings,
                        tokens,
                        next_modes,
                        code_points,
                        plan,
                        grouped,
                        matchers,
                        position,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_grammar_scanner_character_class_members(
    members: &[f64],
    spellings: &[String],
    code_points: &[f64],
) -> String {
    {
        let ordered = Vec::<f64>::new();
        {
            let previous = -1f64;
            {
                let position = 0f64;
                {
                    let ordered_2 = self::source_ml_temporary_render_grammar_scanner_character_class_members_loop4(members, code_points, ordered, previous, position);
                    {
                        let text = String::new();
                        {
                            let first = 0f64;
                            {
                                self::source_ml_temporary_render_grammar_scanner_character_class_members_loop6(spellings, code_points, &(ordered_2), text, first)
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
pub fn render_grammar_scanner_character_class_definition(
    modes: &[String],
    tokens: &[String],
    next_modes: &[String],
    code_points: &[f64],
    plan: &[f64],
    matchers: &[String],
    scanner: &str,
) -> String {
    {
        let declared_modes = vec![String::from("default")];
        {
            let declared_tokens = Vec::<String>::new();
            {
                let declarations = String::new();
                {
                    let literals = String::new();
                    {
                        let characters = String::new();
                        {
                            let position = 0f64;
                            {
                                let source_ml_source1_s1 = self::source_ml_temporary_render_grammar_scanner_character_class_definition_loop8(modes, tokens, next_modes, code_points, plan, matchers, declared_modes, declared_tokens, declarations, literals, characters, position);
                                match source_ml_source1_s1 {
                                    self::MlTemporaryRenderGrammarScannerCharacterClassDefinitionLoop8Result::MlTemporaryRenderGrammarScannerCharacterClassDefinitionLoop8Done(declared_tokens_2, declarations_2, literals_2, characters_2) => {
                                        format!("{declarations_2}(scanner {} (tokens {}) (operations {literals_2}{characters_2}fail))\n", scanner.to_owned(), declared_tokens_2.join((String::from(" ")).as_str()))
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanGrammarScannerCharacterClassesLoop2Result {
    MlTemporaryPlanGrammarScannerCharacterClassesLoop2Done(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_grammar_scanner_character_classes_loop2(
    modes: &[String],
    tokens: &[String],
    next_modes: &[String],
    code_points: &[f64],
    plan: &[f64],
    mut grouped: Vec<f64>,
    mut position: f64,
    mode: &str,
    token: &str,
    next_mode: &str,
    mut members: Vec<f64>,
    mut candidate: f64,
) -> self::MlTemporaryPlanGrammarScannerCharacterClassesLoop2Result {
    loop {
        if candidate < (plan.len() as f64) {
            {
                let index = self::read_required_number_fact(plan, candidate);
                if ((((self::read_required_number_fact(code_points, index)) >= 0f64)
                    && (mode == (self::read_required_text_fact(modes, index))))
                    && (token == (self::read_required_text_fact(tokens, index))))
                    && (next_mode == (self::read_required_text_fact(next_modes, index)))
                {
                    {
                        let members_2 = {
                            let (ml_append_right, ml_append_left) = (vec![index], members);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        if candidate != position {
                            {
                                let grouped_2 = {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![candidate], grouped);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let candidate_2 = candidate + 1f64;
                                    {
                                        (grouped, members, candidate) =
                                            (grouped_2, members_2, candidate_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            {
                                let candidate_3 = candidate + 1f64;
                                {
                                    (members, candidate) = (members_2, candidate_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
                {
                    {
                        let candidate_4 = candidate + 1f64;
                        {
                            candidate = candidate_4;
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryPlanGrammarScannerCharacterClassesLoop2Result::MlTemporaryPlanGrammarScannerCharacterClassesLoop2Done(grouped, members)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_grammar_scanner_character_classes_loop1(
    modes: &[String],
    spellings: &[String],
    tokens: &[String],
    next_modes: &[String],
    code_points: &[f64],
    plan: &[f64],
    mut grouped: Vec<f64>,
    mut matchers: Vec<String>,
    mut position: f64,
) -> Vec<String> {
    loop {
        return if position < (plan.len() as f64) {
            {
                let observation = self::read_required_number_fact(plan, position);
                if self::ml_array::includes(&grouped, &position, 0f64, false, |a: &f64, b: &f64| {
                    a == b || (a.is_nan() && b.is_nan())
                }) {
                    {
                        let matchers_2 = {
                            let (ml_append_right, ml_append_left) = (vec![String::new()], matchers);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let position_2 = position + 1f64;
                            {
                                (matchers, position) = (matchers_2, position_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let spelling = self::read_required_text_fact(spellings, observation);
                        if (self::read_required_number_fact(code_points, observation)) < 0f64 {
                            {
                                let matchers_3 = {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![format!("(literal {spelling})")], matchers);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let position_3 = position + 1f64;
                                    {
                                        (matchers, position) = (matchers_3, position_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            {
                                let mode = self::read_required_text_fact(modes, observation);
                                {
                                    let token = self::read_required_text_fact(tokens, observation);
                                    {
                                        let next_mode =
                                            self::read_required_text_fact(next_modes, observation);
                                        {
                                            let members = Vec::<f64>::new();
                                            {
                                                let candidate = position;
                                                {
                                                    let source_ml_source1_s2 = self::source_ml_temporary_plan_grammar_scanner_character_classes_loop2(modes, tokens, next_modes, code_points, plan, grouped, position, &(mode), &(token), &(next_mode), members, candidate);
                                                    match source_ml_source1_s2 {
                                                        self::MlTemporaryPlanGrammarScannerCharacterClassesLoop2Result::MlTemporaryPlanGrammarScannerCharacterClassesLoop2Done(grouped_2, members_2) => {
                                                            {
                                                                let matchers_4 = { let (ml_append_right, ml_append_left) = (vec![if (members_2.len() as f64) == 1f64 {
                                                                    format!("(literal {spelling})")
                                                                } else {
                                                                    format!("(class plain{})", self::render_grammar_scanner_character_class_members(&(members_2), spellings, code_points))
                                                                }], matchers); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    let position_4 = position + 1f64;
                                                                    {
                                                                        (grouped, matchers, position) = (grouped_2, matchers_4, position_4);
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
        } else {
            matchers
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryRenderGrammarScannerCharacterClassMembersLoop5Result {
    MlTemporaryRenderGrammarScannerCharacterClassMembersLoop5Done(f64, f64),
}
