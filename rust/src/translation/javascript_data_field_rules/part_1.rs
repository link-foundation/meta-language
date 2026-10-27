// Generated from complete js/src/translation/javascript-data-field-rules.js. Do not edit by hand.
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
        if x - floor >= 0.5 { floor + 1.0 } else { floor }
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
            if a.is_sign_negative() { b } else { a }
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
            if a.is_sign_negative() { a } else { b }
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_data_field_plan(
    language: &str,
    field: &str,
    inherited: bool,
    positions: &[f64],
    types: &[String],
) -> (String, String, bool) {
    {
        let ml_library_inequality_1 = language != "JavaScript";
        if ml_library_inequality_1 {
            {
                let source_ml_source1_field1 = String::from("unchanged");
                {
                    let source_ml_source1_field2 = String::new();
                    {
                        let source_ml_source1_field3 = false;
                        (
                            source_ml_source1_field2,
                            source_ml_source1_field1,
                            source_ml_source1_field3,
                        )
                    }
                }
            }
        } else if (((positions.len() as f64) == 0f64) || ((positions.len() as f64) > 65_536f64))
            || ((positions.len() as f64) != (types.len() as f64))
        {
            {
                let source_ml_source1_field4 = String::from("refuse");
                {
                    let source_ml_source1_field5 = String::from(
                        "a datatype property needs its complete bounded constructor directory",
                    );
                    {
                        let source_ml_source1_field6 = false;
                        (
                            source_ml_source1_field5,
                            source_ml_source1_field4,
                            source_ml_source1_field6,
                        )
                    }
                }
            }
        } else if field == "$" {
            {
                let source_ml_source1_field7 = String::from("tag");
                {
                    let source_ml_source1_field8 = String::new();
                    {
                        let source_ml_source1_field9 = false;
                        (
                            source_ml_source1_field8,
                            source_ml_source1_field7,
                            source_ml_source1_field9,
                        )
                    }
                }
            }
        } else {
            {
                let missing = false;
                {
                    let selected = String::new();
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s10 =
                                self::source_ml_temporary_read_java_script_data_field_plan_loop1(
                                    positions, types, missing, selected, at,
                                );
                            match source_ml_source1_s10 {
                                    self::MlTemporaryReadJavaScriptDataFieldPlanLoop1Result::MlTemporaryReadJavaScriptDataFieldPlanLoop1Done(missing_2, selected_2) => {
                                        if missing_2 && inherited {
                                            {
                                                let source_ml_source1_field11 = String::from("refuse");
                                                {
                                                    let source_ml_source1_field12 = String::from("an inherited datatype property needs its complete prototype operation");
                                                    {
                                                        let source_ml_source1_field13 = false;
                                                        (source_ml_source1_field12, source_ml_source1_field11, source_ml_source1_field13)
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                let source_ml_source1_field14 = if selected_2 == "" {
                                                    String::from("absent")
                                                } else {
                                                    String::from("field")
                                                };
                                                {
                                                    let source_ml_source1_field15 = String::new();
                                                    {
                                                        let source_ml_source1_field16 = missing_2;
                                                        (source_ml_source1_field15, source_ml_source1_field14, source_ml_source1_field16)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryReadJavaScriptDataFieldPlanLoop1Result::MlTemporaryReadJavaScriptDataFieldPlanLoop1Return(source_ml_temporary_result1) => {
                                        source_ml_temporary_result1.clone()
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
pub fn read_java_script_data_field_inference_form(field: &str, positions: &[f64]) -> String {
    if ((positions.len() as f64) == 0f64) || ((positions.len() as f64) > 65_536f64) {
        String::from("refuse")
    } else if field == "$" {
        String::from("tag")
    } else {
        {
            let present = false;
            {
                let missing = false;
                {
                    let source_ml_temporary_values2 = positions.to_vec();
                    {
                        let source_ml_temporary_index2 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source1_s17 = self::source_ml_temporary_read_java_script_data_field_inference_form_loop3(present, missing, &(source_ml_temporary_values2), source_ml_temporary_index2);
                            match source_ml_source1_s17 {
                                    self::MlTemporaryReadJavaScriptDataFieldInferenceFormLoop3Result::MlTemporaryReadJavaScriptDataFieldInferenceFormLoop3Done(present_2, missing_2) => {
                                        if present_2 {if missing_2 {
                                                String::from("optional")
                                            } else {
                                                String::from("present")
                                            }} else {
                                            String::from("absent")
                                        }
                                    }
                                    self::MlTemporaryReadJavaScriptDataFieldInferenceFormLoop3Result::MlTemporaryReadJavaScriptDataFieldInferenceFormLoop3Return(source_ml_temporary_result3) => {
                                        source_ml_temporary_result3
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
pub fn accept_java_script_data_field_owner_fallback(
    kind: &str,
    owners: f64,
    settled: bool,
) -> bool {
    (settled && (kind == "var")) && (owners == 1f64)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptDataFieldPlanLoop1Result {
    MlTemporaryReadJavaScriptDataFieldPlanLoop1Done(bool, String),
    MlTemporaryReadJavaScriptDataFieldPlanLoop1Return((String, String, bool)),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_data_field_plan_loop1(
    positions: &[f64],
    types: &[String],
    mut missing: bool,
    mut selected: String,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptDataFieldPlanLoop1Result {
    loop {
        return if at < (positions.len() as f64) {
            {
                let position =
                    (self::ml_array::optional_at(positions, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| -2f64);
                {
                    let type_ =
                        (self::ml_array::optional_at(types, self::ml_array::number_index(at)))
                            .unwrap_or_else(|| String::new());
                    if (!(self::ml_math::is_safe_integer(position)) || (position < -1f64))
                        || if position < 0f64 {
                            type_ != ""
                        } else {
                            type_ == ""
                        }
                    {
                        self::MlTemporaryReadJavaScriptDataFieldPlanLoop1Result::MlTemporaryReadJavaScriptDataFieldPlanLoop1Return({
                            let source_ml_source1_field18 = String::from("refuse");
                            {
                                let source_ml_source1_field19 = String::from("a datatype property needs actual matching field coordinates and types");
                                {
                                    let source_ml_source1_field20 = false;
                                    (source_ml_source1_field19, source_ml_source1_field18, source_ml_source1_field20)
                                }
                            }
                        })
                    } else {
                        if position < 0f64 {
                            {
                                let missing_2 = true;
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        (missing, at) = (missing_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if (selected).is_empty() {
                                {
                                    let selected_2 = type_.clone();
                                    {
                                        let at_3 = at + 1f64;
                                        {
                                            (selected, at) = (selected_2, at_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                {
                                    let ml_library_inequality_1 = selected != type_;
                                    if ml_library_inequality_1 {
                                        self::MlTemporaryReadJavaScriptDataFieldPlanLoop1Result::MlTemporaryReadJavaScriptDataFieldPlanLoop1Return({
                                        let source_ml_source1_field21 = String::from("refuse");
                                        {
                                            let source_ml_source1_field22 = String::from("different constructor field types need their explicit union property model");
                                            {
                                                let source_ml_source1_field23 = false;
                                                (source_ml_source1_field22, source_ml_source1_field21, source_ml_source1_field23)
                                            }
                                        }
                                    })
                                    } else {
                                        {
                                            let at_4 = at + 1f64;
                                            {
                                                at = at_4;
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
        } else {
            self::MlTemporaryReadJavaScriptDataFieldPlanLoop1Result::MlTemporaryReadJavaScriptDataFieldPlanLoop1Done(missing, selected.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptDataFieldInferenceFormLoop3Result {
    MlTemporaryReadJavaScriptDataFieldInferenceFormLoop3Done(bool, bool),
    MlTemporaryReadJavaScriptDataFieldInferenceFormLoop3Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_data_field_inference_form_loop3(
    mut present: bool,
    mut missing: bool,
    source_ml_temporary_values2: &[f64],
    mut source_ml_temporary_index2: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptDataFieldInferenceFormLoop3Result {
    loop {
        return if source_ml_temporary_index2
            < (self::ml::Big::from_u128(source_ml_temporary_values2.len() as u128))
        {
            {
                let position = self::ml_array::at(
                    source_ml_temporary_values2,
                    source_ml_temporary_index2.to_index(),
                );
                if !(self::ml_math::is_safe_integer(position)) || (position < -1f64) {
                    self::MlTemporaryReadJavaScriptDataFieldInferenceFormLoop3Result::MlTemporaryReadJavaScriptDataFieldInferenceFormLoop3Return(String::from("refuse"))
                } else {
                    if position < 0f64 {
                        {
                            let missing_2 = true;
                            {
                                let source_ml_temporary_index2_2 =
                                    source_ml_temporary_index2.add(&self::ml::Big::from_i128(1));
                                {
                                    (missing, source_ml_temporary_index2) =
                                        (missing_2, source_ml_temporary_index2_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let present_2 = true;
                            {
                                let source_ml_temporary_index2_3 =
                                    source_ml_temporary_index2.add(&self::ml::Big::from_i128(1));
                                {
                                    (present, source_ml_temporary_index2) =
                                        (present_2, source_ml_temporary_index2_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptDataFieldInferenceFormLoop3Result::MlTemporaryReadJavaScriptDataFieldInferenceFormLoop3Done(present, missing)
        };
    }
}
