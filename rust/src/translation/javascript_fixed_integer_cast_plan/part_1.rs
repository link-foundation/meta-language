// Generated from complete js/src/translation/javascript-fixed-integer-cast-plan.js. Do not edit by hand.
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
            Self { negative: negative && !magnitude.is_empty(), magnitude }
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
            let value = self.magnitude.iter().rev().fold(0u64, |acc, &digit| (acc << 32) | u64::from(digit));
            usize::try_from(value).ok()
        }

        pub fn neg(&self) -> Self {
            Self::from_parts(!self.negative, self.magnitude.clone())
        }

        pub fn add(&self, other: &Self) -> Self {
            if self.negative == other.negative {
                return Self::from_parts(self.negative, add_magnitude(&self.magnitude, &other.magnitude));
            }
            match compare_magnitude(&self.magnitude, &other.magnitude) {
                Ordering::Less => Self::from_parts(other.negative, sub_magnitude(&other.magnitude, &self.magnitude)),
                _ => Self::from_parts(self.negative, sub_magnitude(&self.magnitude, &other.magnitude)),
            }
        }

        pub fn sub(&self, other: &Self) -> Self {
            self.add(&other.neg())
        }

        pub fn mul(&self, other: &Self) -> Self {
            Self::from_parts(self.negative != other.negative, mul_magnitude(&self.magnitude, &other.magnitude))
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
                quotient = if b.is_negative() { quotient.add(&one) } else { quotient.sub(&one) };
            }
        }
        if remainder { a.sub(&quotient.mul(b)) } else { quotient }
    }

    pub fn to_nat_checked(value: Big, message: &str) -> Big {
        if value.is_negative() {
            panic!("{message}");
        }
        value
    }

    pub fn clamp_nat(value: Big) -> Big {
        if value.is_negative() { Big::zero() } else { value }
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
        if x - floor >= 0.5 {
            floor + 1.0
        } else {
            floor
        }
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
        is_integer(x) && x.abs() <= 9007199254740991.0
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
        if index >= 0.0 && index.fract() == 0.0 && index < 9007199254740992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    fn integer(value: f64) -> f64 { if value.is_nan() { 0.0 } else { value.trunc() } }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 { (length as f64 + value).max(0.0) as usize } else { value.min(length as f64) as usize }
    }
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len()); let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
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
    pub fn includes<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values { if !out.iter().any(|previous| equal(previous, &value)) { out.push(value); } }
        out
    }
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
pub fn read_fixed_integer_cast_modulus(bits: f64) -> String {
    if (bits == (8f64)) {
        String::from("256")
    } else {
        if (bits == (16f64)) {
            String::from("65536")
        } else {
            if (bits == (32f64)) {
                String::from("4294967296")
            } else {
                if (bits == (64f64)) {
                    String::from("18446744073709551616")
                } else {
                    if (bits == (128f64)) {
                        String::from("340282366920938463463374607431768211456")
                    } else {
                        String::from("")
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_fixed_integer_cast_signed_limit(bits: f64) -> String {
    if (bits == (8f64)) {
        String::from("128")
    } else {
        if (bits == (16f64)) {
            String::from("32768")
        } else {
            if (bits == (32f64)) {
                String::from("2147483648")
            } else {
                if (bits == (64f64)) {
                    String::from("9223372036854775808")
                } else {
                    if (bits == (128f64)) {
                        String::from("170141183460469231731687303715884105728")
                    } else {
                        String::from("")
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_fixed_integer_cast_expression(bits: f64, signed: bool) -> Vec<String> {
    {
        let modulus = self::read_fixed_integer_cast_modulus(bits);
        if (modulus == "") {
            Vec::<String>::new()
        } else {
            vec![String::from("input"), String::from(""), String::from(""), String::from(""), String::from(""), String::from("integer"), modulus.clone(), String::from(""), String::from(""), String::from(""), String::from("integer"), String::from("0"), String::from(""), String::from(""), String::from(""), String::from("reference"), String::from("0"), String::from(""), String::from(""), String::from(""), String::from("binary"), String::from("rem"), String::from("3"), String::from("1"), String::from(""), String::from("reference"), String::from("1"), String::from(""), String::from(""), String::from(""), String::from("binary"), String::from("lt"), String::from("5"), String::from("2"), String::from(""), String::from("binary"), String::from("add"), String::from("5"), String::from("1"), String::from(""), String::from("conditional"), String::from(""), String::from("6"), String::from("7"), String::from("5"), String::from("reference"), String::from("2"), String::from(""), String::from(""), String::from(""), String::from("integer"), self::read_fixed_integer_cast_signed_limit(bits), String::from(""), String::from(""), String::from(""), String::from("binary"), String::from("ge"), String::from("9"), String::from("10"), String::from(""), String::from("binary"), String::from("sub"), String::from("9"), String::from("1"), String::from(""), String::from("conditional"), String::from(""), String::from("11"), String::from("12"), String::from("9"), String::from("let"), String::from("2"), String::from("8"), if signed {
                String::from("13")
            } else {
                String::from("9")
            }, String::from(""), String::from("let"), String::from("1"), String::from("4"), String::from("14"), String::from(""), String::from("let"), String::from("0"), String::from("0"), String::from("15"), String::from("")]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_fixed_integer_cast_expression(bits: f64, signed: bool) -> Vec<String> {
    {
        let width = if bits.is_nan() {
            0f64
        } else {
            bits.trunc()
        };
        if ((!(width.is_finite()) || (width < (0f64))) || (width > (4096f64))) {
            Vec::<String>::new()
        } else {
            {
                let modulus = self::ml::Big::from_i128(1);
                {
                    let index = 0f64;
                    {
                        let modulus_2 = self::source_ml_source1_temporary_plan_java_script_fixed_integer_cast_expression_loop2(width, modulus.clone(), index);
                        {
                            let modulus_text = {
                                let source_ml_source2_string_argument2 = modulus_2.clone();
                                source_ml_source2_string_argument2.to_string()
                            };
                            {
                                let limit_text = {
                                    let source_ml_source2_string_argument3 = self::ml::divide(&modulus_2, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                                    source_ml_source2_string_argument3.to_string()
                                };
                                {
                                    let plan = self::plan_fixed_integer_cast_expression(8f64, (signed && (width != (0f64))));
                                    { let ml_values1 = plan.clone(); let ml_callback1 = { let plan = plan.clone(); let modulus_text = modulus_text.clone(); let limit_text = limit_text.clone(); std::rc::Rc::new(move |cell: String, index_2: f64| -> String { if ((((index_2 % 5f64)) == (1f64)) && {
                                        let source_ml_source2_comparison_operand4 = self::ml_array::optional_at(&plan, self::ml_array::number_index((index_2 - 1f64)));
                                        {
                                            let source_ml_source2_comparison_operand5 = String::from("integer");
                                            match (source_ml_source2_comparison_operand4.clone(), source_ml_source2_comparison_operand5.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                        }
                                    }) {
                                        if (cell == "256") {
                                            modulus_text.clone()
                                        } else {
                                            if (cell == "128") {
                                                limit_text.clone()
                                            } else {
                                                cell.clone()
                                            }
                                        }
                                    } else {
                                        cell.clone()
                                    } }) }; ml_values1.iter().cloned().enumerate().map(|(ml_index1, ml_value1)| ml_callback1(ml_value1, (ml_index1 as f64))).collect::<Vec<_>>() }
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
pub fn plan_java_script_fixed_integer_arguments(count: f64) -> Vec<f64> {
    if ((!(self::ml_math::is_integer(count)) || (count < (3f64))) || (count > (4096f64))) {
        Vec::<f64>::new()
    } else {
        {
            let coordinates = Vec::<f64>::new();
            {
                let index = 1f64;
                {
                    let coordinates_2 = self::source_ml_source1_temporary_plan_java_script_fixed_integer_arguments_loop3(count, coordinates, index);
                    coordinates_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_fixed_integer_cast_expression_loop2(mut width: f64, mut modulus: self::ml::Big, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < width) {
            {
                let modulus_2 = modulus.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = (index + 1f64);
                    {
                        (modulus, index) = (modulus_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            modulus.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_fixed_integer_arguments_loop3(mut count: f64, mut coordinates: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < count) {
            {
                let coordinates_2 = { let (ml_append_right, ml_append_left) = (vec![index], coordinates); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = (index + 1f64);
                    {
                        (coordinates, index) = (coordinates_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            coordinates
        };
    }
}
