// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlAcceptDeclarationDependencyFactsLoop4Result;
use super::MlAcceptDeclarationDependencyFactsLoop5Result;
use super::MlAcceptDeclarationModuleScopeLoop20Result;
use super::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop26Result;
use super::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop27Result;
use super::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop28Result;
use super::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop29Result;
use super::MlContainsDeclarationDependencyLoop2Result;
use super::MlReadDeclarationDependencyComponentsLoop12Result;
use super::MlReadRecursiveDeclarationSourceOwnersLoop21Result;
use super::MlReadRecursiveDeclarationSourceOwnersLoop22Result;
use super::ml_accept_declaration_dependency_facts_loop4;
use super::ml_accept_declaration_dependency_facts_loop5;
use super::ml_accept_declaration_module_scope_loop20;
use super::ml_accept_recursive_declaration_auxiliary_closure_loop26;
use super::ml_accept_recursive_declaration_auxiliary_closure_loop27;
use super::ml_accept_recursive_declaration_auxiliary_closure_loop28;
use super::ml_accept_recursive_declaration_auxiliary_closure_loop29;
use super::ml_contains_declaration_dependency_loop2;
use super::ml_read_declaration_dependency_components_loop7;
use super::ml_read_declaration_dependency_components_loop8;
use super::ml_read_declaration_dependency_components_loop11;
use super::ml_read_declaration_dependency_components_loop12;
use super::ml_read_declaration_dependency_components_loop13;
use super::ml_read_recursive_declaration_source_owners_loop21;
use super::ml_read_recursive_declaration_source_owners_loop22;

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
pub fn refuse_required_number_fact() -> f64 {
    panic!("{}", "missing required Number fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    panic!("{}", "missing required text fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    panic!("{}", "missing required Boolean fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))),
            "{}",
            "invalid required Number fact column position"
        );

        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_number_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))),
            "{}",
            "invalid required text fact column position"
        );

        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_text_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))),
            "{}",
            "invalid required Boolean fact column position"
        );

        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_boolean_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn contains_declaration_dependency(values: &[f64], value: f64) -> bool {
    {
        let ml_values1 = values.to_vec();
        {
            let ml_index1 = self::ml::Big::from_i128(0);
            {
                let ml_s1 =
                    self::ml_contains_declaration_dependency_loop2(value, &(ml_values1), ml_index1);
                match ml_s1 {
                    self::MlContainsDeclarationDependencyLoop2Result::MlContainsDeclarationDependencyLoop2Done => {
                        false
                    }
                    self::MlContainsDeclarationDependencyLoop2Result::MlContainsDeclarationDependencyLoop2Return(ml_result2) => {
                        ml_result2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_declaration_dependency_facts(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
) -> bool {
    {
        let ml_library_inequality_2 = (starts.len() as f64) != (counts.len() as f64);
        if ml_library_inequality_2 {
            false
        } else {
            {
                let offset = 0f64;
                {
                    let node = 0f64;
                    {
                        let ml_s2 = self::ml_accept_declaration_dependency_facts_loop4(
                            starts, counts, targets, offset, node,
                        );
                        match ml_s2 {
                        self::MlAcceptDeclarationDependencyFactsLoop4Result::MlAcceptDeclarationDependencyFactsLoop4Done(offset_2) => {
                            {
let ml_library_inequality_1 = offset_2 != (targets.len() as f64) ;
if ml_library_inequality_1 {
                                false
                            } else {
                                {
                                    let ml_values3 = targets.to_vec();
                                    {
                                        let ml_index3 = self::ml::Big::from_i128(0);
                                        {
                                            let ml_s3 = self::ml_accept_declaration_dependency_facts_loop5(starts, &(ml_values3), ml_index3);
                                            match ml_s3 {
                                                self::MlAcceptDeclarationDependencyFactsLoop5Result::MlAcceptDeclarationDependencyFactsLoop5Done => {
                                                    true
                                                }
                                                self::MlAcceptDeclarationDependencyFactsLoop5Result::MlAcceptDeclarationDependencyFactsLoop5Return(ml_result5) => {
                                                    ml_result5
                                                }
                                            }
                                        }
                                    }
                                }
                            }
}
                        }
                        self::MlAcceptDeclarationDependencyFactsLoop4Result::MlAcceptDeclarationDependencyFactsLoop4Return(ml_result5_2) => {
                            ml_result5_2
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
pub fn read_declaration_dependency_components(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
) -> Vec<f64> {
    if (self::accept_declaration_dependency_facts(starts, counts, targets)) {
        {
            let visited = Vec::<f64>::new();
            {
                let node = 0f64;
                {
                    let visited_2 = self::ml_read_declaration_dependency_components_loop7(
                        starts, visited, node,
                    );
                    {
                        let finished = Vec::<f64>::new();
                        {
                            let root = 0f64;
                            {
                                let finished_2 =
                                    self::ml_read_declaration_dependency_components_loop8(
                                        starts, counts, targets, visited_2, finished, root,
                                    );
                                {
                                    let assigned = Vec::<f64>::new();
                                    {
                                        let node_2 = 0f64;
                                        {
                                            let assigned_2 = self::ml_read_declaration_dependency_components_loop11(starts, assigned, node_2);
                                            {
                                                let vertices = Vec::<f64>::new();
                                                {
                                                    let leaders = Vec::<f64>::new();
                                                    {
                                                        let position = finished_2.len() as f64;
                                                        {
                                                            let ml_s4 = self::ml_read_declaration_dependency_components_loop12(starts, counts, targets, &(finished_2), assigned_2, vertices, leaders, position);
                                                            match ml_s4 {
                                                                self::MlReadDeclarationDependencyComponentsLoop12Result::MlReadDeclarationDependencyComponentsLoop12Done(vertices_2, leaders_2) => {
                                                                    {
                                                                        let result = Vec::<f64>::new();
                                                                        {
                                                                            let node_3 = 0f64;
                                                                            {
                                                                                self::ml_read_declaration_dependency_components_loop13(starts, &(vertices_2), &(leaders_2), result, node_3)
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
        }
    } else {
        panic!("{}", "invalid declaration dependency facts")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_recursive_declaration_group(count: f64, ordinary: bool, unique: bool) -> bool {
    (((count > 1f64) && ((count % 1f64) == 0f64)) && ordinary) && unique
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_recursive_declaration_publication(
    members: f64,
    declarations: f64,
    signatures: f64,
    flat: bool,
    emitted: bool,
    matching: bool,
) -> bool {
    ((((((members > 1f64) && ((members % 1f64) == 0f64))
        && ((declarations).partial_cmp(&(members)) == Some(std::cmp::Ordering::Equal)))
        && ((signatures).partial_cmp(&(members)) == Some(std::cmp::Ordering::Equal)))
        && flat)
        && emitted)
        && matching
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_declaration_module_scope(forms: &[String], complete_checked: bool) -> bool {
    if complete_checked {
        true
    } else {
        {
            let ml_values19 = forms.to_vec();
            {
                let ml_index19 = self::ml::Big::from_i128(0);
                {
                    let ml_s5 =
                        self::ml_accept_declaration_module_scope_loop20(&(ml_values19), ml_index19);
                    match ml_s5 {
                        self::MlAcceptDeclarationModuleScopeLoop20Result::MlAcceptDeclarationModuleScopeLoop20Done => {
                            true
                        }
                        self::MlAcceptDeclarationModuleScopeLoop20Result::MlAcceptDeclarationModuleScopeLoop20Return(ml_result20) => {
                            ml_result20
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_declaration_signature_binding(scope_known: bool, owner_count: f64) -> bool {
    scope_known && (owner_count == 1f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_recursive_declaration_source_owners(
    root_starts: &[f64],
    root_ends: &[f64],
    declaration_starts: &[f64],
    declaration_ends: &[f64],
    roles: &[f64],
) -> Vec<f64> {
    if ((((root_starts.len() as f64) < 1f64)
        || ((root_starts.len() as f64) != (root_ends.len() as f64)))
        || ((declaration_starts.len() as f64) != (declaration_ends.len() as f64)))
        || ((declaration_starts.len() as f64) != (roles.len() as f64))
    {
        Vec::<f64>::new()
    } else {
        {
            let previous_end = -1f64;
            {
                let root = 0f64;
                {
                    let ml_s6 = self::ml_read_recursive_declaration_source_owners_loop21(
                        root_starts,
                        root_ends,
                        previous_end,
                        root,
                    );
                    match ml_s6 {
                        self::MlReadRecursiveDeclarationSourceOwnersLoop21Result::MlReadRecursiveDeclarationSourceOwnersLoop21Done => {
                            {
                                let owners = Vec::<f64>::new();
                                {
                                    let roots = Vec::<f64>::new();
                                    {
                                        let declaration = 0f64;
                                        {
                                            let ml_s7 = self::ml_read_recursive_declaration_source_owners_loop22(root_starts, root_ends, declaration_starts, declaration_ends, roles, owners, roots, declaration);
                                            match ml_s7 {
                                                self::MlReadRecursiveDeclarationSourceOwnersLoop22Result::MlReadRecursiveDeclarationSourceOwnersLoop22Done(owners_2, roots_2) => {
                                                    if (roots_2.len() as f64) == (root_starts.len() as f64) {
                                                        owners_2
                                                    } else {
                                                        Vec::<f64>::new()
                                                    }
                                                }
                                                self::MlReadRecursiveDeclarationSourceOwnersLoop22Result::MlReadRecursiveDeclarationSourceOwnersLoop22Return(ml_result22) => {
                                                    ml_result22
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::MlReadRecursiveDeclarationSourceOwnersLoop21Result::MlReadRecursiveDeclarationSourceOwnersLoop21Return(ml_result24) => {
                            ml_result24
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_recursive_declaration_auxiliary_closure(
    owners: &[f64],
    roles: &[f64],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
) -> bool {
    if (((owners.len() as f64) != (roles.len() as f64))
        || ((owners.len() as f64) != (starts.len() as f64)))
        || !(self::accept_declaration_dependency_facts(starts, counts, targets))
    {
        false
    } else {
        {
            let root_owners = Vec::<f64>::new();
            {
                let declaration = 0f64;
                {
                    let ml_s8 = self::ml_accept_recursive_declaration_auxiliary_closure_loop26(
                        owners,
                        roles,
                        root_owners,
                        declaration,
                    );
                    match ml_s8 {
                        self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop26Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop26Done(root_owners_2) => {
                            if (root_owners_2.len() as f64) < 1f64 {
                                false
                            } else {
                                {
                                    let ml_values25 = owners.to_vec();
                                    {
                                        let ml_index25 = self::ml::Big::from_i128(0);
                                        {
                                            let ml_s9 = self::ml_accept_recursive_declaration_auxiliary_closure_loop27(&(root_owners_2), &(ml_values25), ml_index25);
                                            match ml_s9 {
                                                self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop27Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop27Done => {
                                                    {
                                                        let source = 0f64;
                                                        {
                                                            let ml_s10 = self::ml_accept_recursive_declaration_auxiliary_closure_loop28(owners, roles, starts, counts, targets, source);
                                                            match ml_s10 {
                                                                self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop28Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop28Done => {
                                                                    {
                                                                        let root = 0f64;
                                                                        {
                                                                            let ml_s11 = self::ml_accept_recursive_declaration_auxiliary_closure_loop29(owners, roles, starts, counts, targets, root);
                                                                            match ml_s11 {
                                                                                self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop29Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop29Done => {
                                                                                    true
                                                                                }
                                                                                self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop29Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop29Return(ml_result29) => {
                                                                                    ml_result29
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop28Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop28Return(ml_result32) => {
                                                                    ml_result32
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop27Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop27Return(ml_result33) => {
                                                    ml_result33
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop26Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop26Return(ml_result33_2) => {
                            ml_result33_2
                        }
                    }
                }
            }
        }
    }
}
