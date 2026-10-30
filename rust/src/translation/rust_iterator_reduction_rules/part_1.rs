// Generated from complete js/src/translation/rust-iterator-reduction-rules.js. Do not edit by hand.
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
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_iterator_arithmetic_consumer(consumer: &str) -> bool {
    (consumer == "sum") || (consumer == "product")
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_iterator_arithmetic_error(
    kind: &str,
    known: bool,
    owner: bool,
    layers: f64,
    result_known: bool,
    result_layers: f64,
    same_type: bool,
    same_owner: bool,
) -> String {
    if ((((((((kind == "fixed") || (kind == "float")) && known) && owner)
        && ((layers == 0f64) || (layers == 1f64)))
        && result_known)
        && (result_layers == 0f64))
        && same_type)
        && same_owner
    {
        String::new()
    } else {
        String::from(
            "sum and product need one standard numeric item owner and the matching direct result type",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_iterator_arithmetic_seed(consumer: &str, kind: &str) -> Vec<String> {
    if consumer == "sum" {
        if kind == "float" {
            vec![String::from("0"), String::from("negative")]
        } else {
            vec![String::from("0"), String::from("positive")]
        }
    } else if consumer == "product" {
        vec![String::from("1"), String::from("positive")]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_iterator_arithmetic_terminal(consumer: &str) -> Vec<String> {
    if consumer == "sum" {
        vec![
            String::from("accumulator"),
            String::from("item"),
            String::from("add"),
        ]
    } else if consumer == "product" {
        vec![
            String::from("accumulator"),
            String::from("item"),
            String::from("multiply"),
        ]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_iterator_reduction_start(method: &str, count: f64, ordinary: bool) -> bool {
    (ordinary && ((method == "iter") || (method == "into_iter"))) && (count == 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_iterator_copy_adapter(method: &str) -> bool {
    (method == "copied") || (method == "cloned")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_reduction_operation(method: &str) -> String {
    if self::accept_rust_iterator_arithmetic_consumer(method) {
        method.to_owned()
    } else if method == "collect" {
        String::from("collect")
    } else if method == "count" {
        String::from("count")
    } else if method == "all" {
        String::from("every")
    } else if method == "any" {
        String::from("some")
    } else if (method == "fold") || (method == "rfold") {
        String::from("reduce")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_reduction_callback_arity(method: &str) -> f64 {
    if (method == "fold") || (method == "rfold") {
        2f64
    } else if (method == "all") || (method == "any") {
        1f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_reduction_capture_roles(method: &str) -> Vec<String> {
    if (method == "fold") || (method == "rfold") {
        vec![
            String::from("receiver"),
            String::from("initial"),
            String::from("callback"),
        ]
    } else if (method == "all") || (method == "any") {
        vec![String::from("receiver"), String::from("callback")]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_reduction_receiver_error(receiver: bool, owner: bool) -> String {
    if receiver && owner {
        String::new()
    } else {
        String::from(
            "a reduction needs an actual standard immutable scalar array and its complete primitive element owner",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_reduction_accumulator_error(
    known: bool,
    copy: bool,
    layers: f64,
) -> String {
    if (known && copy) && (layers == 0f64) {
        String::new()
    } else {
        String::from("fold needs the actual direct immutable Copy accumulator type")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_reduction_callback_error(
    consumer: &str,
    kind: &str,
    count: f64,
    layers: f64,
    parameters: &[bool],
    result: bool,
) -> String {
    {
        let expected = self::read_rust_iterator_reduction_callback_arity(consumer);
        if (((((expected < 0f64) || (kind != "function")) || (count != expected))
            || (layers != 0f64))
            || ((parameters.len() as f64) != expected))
            || !result
        {
            String::from("a reduction needs its complete direct scalar callable source signature")
        } else {
            {
                let source_ml_temporary_values1 = parameters.to_vec();
                {
                    let source_ml_temporary_index1 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_s1 = self::source_ml_temporary_read_rust_iterator_reduction_callback_error_loop2(&(source_ml_temporary_values1), source_ml_temporary_index1);
                        match source_ml_source1_s1 {
                            self::MlTemporaryReadRustIteratorReductionCallbackErrorLoop2Result::MlTemporaryReadRustIteratorReductionCallbackErrorLoop2Done => {
                                String::new()
                            }
                            self::MlTemporaryReadRustIteratorReductionCallbackErrorLoop2Result::MlTemporaryReadRustIteratorReductionCallbackErrorLoop2Return(source_ml_temporary_result2) => {
                                source_ml_temporary_result2
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
pub fn accept_rust_iterator_reduction_adapter(method: &str) -> bool {
    (method == "rev") || self::accept_rust_iterator_copy_adapter(method)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_reduction_adapter_state(
    method: &str,
    state: &str,
    direction: &str,
) -> Vec<String> {
    if (((state != "borrowed") && (state != "copied")) && (state != "owned"))
        || ((direction != "forward") && (direction != "reverse"))
    {
        Vec::<String>::new()
    } else if method == "rev" {
        vec![
            state.to_owned(),
            if direction == "forward" {
                String::from("reverse")
            } else {
                String::from("forward")
            },
        ]
    } else if self::accept_rust_iterator_copy_adapter(method) && (state == "borrowed") {
        vec![String::from("copied"), direction.to_owned()]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_iterator_reduction_receiver(method: &str, direction: &str) -> Vec<String> {
    if (method == "rfold") == (direction == "reverse") {
        Vec::<String>::new()
    } else {
        vec![String::from("map reversed indices")]
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_iterator_reduction_reverse_index() -> Vec<String> {
    vec![
        String::from("receiver"),
        String::from("receiver"),
        String::from("length"),
        String::from("index"),
        String::from("sub"),
        String::from("one"),
        String::from("sub"),
        String::from("natural"),
        String::from("lookup"),
    ]
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_reduction_count_result_owner() -> String {
    String::from("usize")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_iterator_reduction_count() -> Vec<String> {
    vec![
        String::from("receiver"),
        String::from("integer length"),
        String::from("result cast"),
    ]
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_iterator_collection_inferred_type(token: &str, known: bool) -> bool {
    self::accept_rust_iterator_item_inferred_type(token, known)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_iterator_item_inferred_type(token: &str, known: bool) -> bool {
    (token == "_") && known
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_iterator_collection_error(
    copied: bool,
    known: bool,
    kind: &str,
    model: &str,
    owner_kind: &str,
    owner_path: &str,
    children: f64,
    same_element: bool,
    layers: f64,
) -> String {
    if (((((((copied && known) && (kind == "array")) && (model == "owned vector"))
        && (owner_kind == "library"))
        && (owner_path == "std::vec::Vec"))
        && (children == 1f64))
        && same_element)
        && (layers == 0f64)
    {
        String::new()
    } else {
        String::from(
            "collection needs copied scalar items and an actual standard Vec of their complete source type",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_count_receiver_error(
    kind: &str,
    model: &str,
    known: bool,
    layers: bool,
    adapters: bool,
) -> String {
    if ((((kind == "array") && ((model == "owned vector") || (model == "immutable slice")))
        && known)
        && layers)
        && adapters
    {
        String::new()
    } else {
        String::from("count needs an actual standard array receiver and valid item adapters")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_reduction_initial_state(method: &str) -> String {
    if method == "into_iter" {
        String::from("owned")
    } else if method == "iter" {
        String::from("borrowed")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_iterator_owned_receiver_error(
    kind: &str,
    model: &str,
    known: bool,
    layers: f64,
    fresh: bool,
) -> String {
    if ((((kind == "array") && (model == "owned vector")) && known) && (layers == 0f64)) && fresh {
        String::new()
    } else {
        String::from(
            "owned iteration needs an actual fresh standard vector and its consumption proof",
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustIteratorReductionCallbackErrorLoop2Result {
    MlTemporaryReadRustIteratorReductionCallbackErrorLoop2Done,
    MlTemporaryReadRustIteratorReductionCallbackErrorLoop2Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_iterator_reduction_callback_error_loop2(
    source_ml_temporary_values1: &[bool],
    mut source_ml_temporary_index1: self::ml::Big,
) -> self::MlTemporaryReadRustIteratorReductionCallbackErrorLoop2Result {
    loop {
        return if source_ml_temporary_index1
            < (self::ml::Big::from_u128(source_ml_temporary_values1.len() as u128))
        {
            {
                let parameter = self::ml_array::at(
                    source_ml_temporary_values1,
                    source_ml_temporary_index1.to_index(),
                );
                if parameter {
                    {
                        let source_ml_temporary_index1_2 =
                            source_ml_temporary_index1.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index1 = source_ml_temporary_index1_2;
                            continue;
                        }
                    }
                }
                {
                    self::MlTemporaryReadRustIteratorReductionCallbackErrorLoop2Result::MlTemporaryReadRustIteratorReductionCallbackErrorLoop2Return(String::from("a reduction needs its complete direct scalar callable source signature"))
                }
            }
        } else {
            self::MlTemporaryReadRustIteratorReductionCallbackErrorLoop2Result::MlTemporaryReadRustIteratorReductionCallbackErrorLoop2Done
        };
    }
}
