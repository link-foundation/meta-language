// Generated from complete js/src/translation/source-library-binding-rules.js. Do not edit by hand.
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn dependency_function1(kind: &str, name: &str, duplicate: bool, reserved: &[String]) -> bool {
    if ((kind != "identifier") || ((name).is_empty())) || duplicate {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s60 =
                    self::source_ml_source1_temporary_dependency_function1_loop3(
                        name, reserved, index,
                    );
                match source_ml_source2_s60 {
                    self::MlSource1TemporaryDependencyFunction1Loop3Result::MlSource1TemporaryDependencyFunction1Loop3Done => {
                        true
                    }
                    self::MlSource1TemporaryDependencyFunction1Loop3Result::MlSource1TemporaryDependencyFunction1Loop3Return(source_ml_source1_temporary_result3) => {
                        source_ml_source1_temporary_result3
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_library_declaration_binding_kind(
    keyword: &str,
    kind: &str,
    name: &str,
    callable: bool,
) -> String {
    if (self::dependency_function1(kind, name, false, &(Vec::<String>::new()))) {
        if keyword == "function" {
            String::from("function")
        } else if (keyword == "const") && callable {
            String::from("const")
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_source_function_head_name(kind: &str) -> bool {
    kind == "identifier"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_library_function_declaration_offset(
    async_: &str,
    second: &str,
    line_break: bool,
) -> f64 {
    if async_ == "function" {
        0f64
    } else if ((async_ == "async") && (second == "function")) && !line_break {
        1f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_source_function_expression_term(term: &str) -> bool {
    (term == "function_expression") || (term == "generator_function")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn test_java_script_source_keyword_role(identifier: bool, keyword: bool) -> bool {
    !identifier && keyword
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_function_scope_form(term: &str) -> String {
    if self::test_java_script_source_function_expression_term(term) {
        String::from("function expression")
    } else if (term == "function_declaration") || (term == "generator_function_declaration") {
        String::from("function declaration")
    } else if term == "arrow_function" {
        String::from("arrow function")
    } else if term == "method_definition" {
        String::from("method definition")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_source_simple_parameter_list(terms: &[String]) -> bool {
    {
        let source_ml_source1_temporary_values4 = terms.to_vec();
        {
            let source_ml_source1_temporary_index4 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s61 = self::source_ml_source1_temporary_accept_java_script_source_simple_parameter_list_loop5(&(source_ml_source1_temporary_values4), source_ml_source1_temporary_index4);
                match source_ml_source2_s61 {
                    self::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Done => {
                        true
                    }
                    self::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Return(source_ml_source1_temporary_result5) => {
                        source_ml_source1_temporary_result5
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_source_parameter_variable_dependency(
    first: &str,
    second: &str,
    body_parent: bool,
) -> bool {
    body_parent
        && (((first == "parameter") && (second == "var"))
            || ((first == "var") && (second == "parameter")))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_source_variable_redeclaration(kind: &str, existing: bool) -> bool {
    (kind == "var") && existing
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_source_loop_scope_term(term: &str) -> bool {
    ((term == "for_statement") || (term == "for_in_statement")) || (term == "for_of_statement")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_source_catch_scope_term(term: &str) -> bool {
    term == "catch_clause"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_source_class_expression_term(term: &str) -> bool {
    term == "class"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_source_variable_environment_term(term: &str) -> bool {
    (term == "class_static_block")
        || (!(self::read_java_script_source_function_scope_form(term)).is_empty())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDependencyFunction1Loop3Result {
    MlSource1TemporaryDependencyFunction1Loop3Done,
    MlSource1TemporaryDependencyFunction1Loop3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_dependency_function1_loop3(
    name: &str,
    reserved: &[String],
    mut index: f64,
) -> self::MlSource1TemporaryDependencyFunction1Loop3Result {
    loop {
        return if index < (reserved.len() as f64) {
            {
                let word =
                    self::ml_array::optional_at(reserved, self::ml_array::number_index(index));
                if {
                    let source_ml_source2_comparison_operand71 = word.clone();
                    {
                        let source_ml_source2_comparison_operand72 = None::<()>;
                        match (
                            source_ml_source2_comparison_operand71,
                            source_ml_source2_comparison_operand72,
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }
                    }
                } {
                    panic!(
                        "{}",
                        "source binding reserved names need a complete text column"
                    )
                } else if (word.clone().expect("a proven present value")) == name {
                    self::MlSource1TemporaryDependencyFunction1Loop3Result::MlSource1TemporaryDependencyFunction1Loop3Return(false)
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDependencyFunction1Loop3Result::MlSource1TemporaryDependencyFunction1Loop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result {
    MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Done,
    MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_java_script_source_simple_parameter_list_loop5(
    source_ml_source1_temporary_values4: &[String],
    mut source_ml_source1_temporary_index4: self::ml::Big,
) -> self::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result {
    loop {
        return if source_ml_source1_temporary_index4
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values4.len() as u128))
        {
            {
                let term = self::ml_array::at(
                    source_ml_source1_temporary_values4,
                    source_ml_source1_temporary_index4.to_index(),
                );
                if self::ml_array::includes(
                    &vec![
                        String::from("assignment_pattern"),
                        String::from("object_assignment_pattern"),
                        String::from("rest_pattern"),
                        String::from("object_pattern"),
                        String::from("array_pattern"),
                    ],
                    &term,
                    0f64,
                    false,
                    |a, b| a == b,
                ) {
                    self::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Return(false)
                } else {
                    {
                        let source_ml_source1_temporary_index4_2 =
                            source_ml_source1_temporary_index4.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index4 =
                                source_ml_source1_temporary_index4_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Result::MlSource1TemporaryAcceptJavaScriptSourceSimpleParameterListLoop5Done
        };
    }
}
