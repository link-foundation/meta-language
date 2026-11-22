// Translated from JavaScript by meta-language: portable core, Rust target.
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]

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

pub mod ml_source_exception {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SourceException {
        pub class_name: String,
        pub message_units: Vec<u16>,
        pub message_present: bool,
    }
    impl SourceException {
        pub fn raise(self) -> ! {
            std::panic::panic_any((self.class_name, self.message_units, self.message_present))
        }
        pub fn from_text(class_name: &str, message: &str) -> Self {
            Self { class_name: class_name.to_owned(), message_units: message.encode_utf16().collect(), message_present: true }
        }
    }
    impl std::fmt::Display for SourceException {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.message_units))
        }
    }
    impl std::error::Error for SourceException {}
}

pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required Number fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

pub fn refuse_required_text_fact() -> String {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required text fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("missing required Boolean fact column value")).encode_utf16().collect(), message_present: true }).raise()
}

pub fn read_required_number_fact(values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, index: f64) -> f64 {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ({ let ml_reference_array1 = values.clone(); let ml_reference_projection1 = ml_reference_array1.borrow().len() as f64; ml_reference_projection1 }))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required Number fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        ({
            let source_ml_source1_array_read_value1 = values.clone();
            {
                let source_ml_source1_array_read_index2 = index;
                { let ml_reference_array2 = source_ml_source1_array_read_value1.clone(); let ml_reference_position2 = source_ml_source1_array_read_index2; let ml_reference_projection2 = self::ml_array::optional_at(&*ml_reference_array2.borrow(), self::ml_array::number_index(ml_reference_position2)); ml_reference_projection2 }
            }
        }).clone().unwrap_or_else(|| self::refuse_required_number_fact())
    }
}

pub fn read_required_text_fact(values: std::rc::Rc<std::cell::RefCell<Vec<String>>>, index: f64) -> String {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ({ let ml_reference_array3 = values.clone(); let ml_reference_projection3 = ml_reference_array3.borrow().len() as f64; ml_reference_projection3 }))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required text fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        ({
            let source_ml_source1_array_read_value3 = values.clone();
            {
                let source_ml_source1_array_read_index4 = index;
                { let ml_reference_array4 = source_ml_source1_array_read_value3.clone(); let ml_reference_position4 = source_ml_source1_array_read_index4; let ml_reference_projection4 = self::ml_array::optional_at(&*ml_reference_array4.borrow(), self::ml_array::number_index(ml_reference_position4)); ml_reference_projection4 }
            }
        }).clone().unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

pub fn read_required_boolean_fact(values: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, index: f64) -> bool {
    if (((index < (0f64)) || (((index % 1f64)) != (0f64))) || (index >= ({ let ml_reference_array5 = values.clone(); let ml_reference_projection5 = ml_reference_array5.borrow().len() as f64; ml_reference_projection5 }))) {
        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (String::from("invalid required Boolean fact column position")).encode_utf16().collect(), message_present: true }).raise()
    } else {
        ({
            let source_ml_source1_array_read_value5 = values.clone();
            {
                let source_ml_source1_array_read_index6 = index;
                { let ml_reference_array6 = source_ml_source1_array_read_value5.clone(); let ml_reference_position6 = source_ml_source1_array_read_index6; let ml_reference_projection6 = self::ml_array::optional_at(&*ml_reference_array6.borrow(), self::ml_array::number_index(ml_reference_position6)); ml_reference_projection6 }
            }
        }).clone().unwrap_or_else(|| self::refuse_required_boolean_fact())
    }
}

pub fn plan_datatype_dependency_groups(count: f64, ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if (((((!(self::ml_math::is_integer(count)) || {
        let source_ml_source1_same_value_operand7 = count;
        {
            let source_ml_source1_same_value_operand8 = (-0f64);
            (((source_ml_source1_same_value_operand7 == source_ml_source1_same_value_operand8) && ((source_ml_source1_same_value_operand7 != (0f64)) || (((1f64 / source_ml_source1_same_value_operand7)) == ((1f64 / source_ml_source1_same_value_operand8))))) || ((source_ml_source1_same_value_operand7 != source_ml_source1_same_value_operand7) && (source_ml_source1_same_value_operand8 != source_ml_source1_same_value_operand8)))
        }
    }) || (count < (0f64))) || (count > (8192f64))) || (({ let ml_reference_array7 = ends.clone(); let ml_reference_projection7 = ml_reference_array7.borrow().len() as f64; ml_reference_projection7 }) != count)) || (({ let ml_reference_array8 = edges.clone(); let ml_reference_projection8 = ml_reference_array8.borrow().len() as f64; ml_reference_projection8 }) > (131072f64))) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
    } else {
        {
            let starts = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
            {
                let heads = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                {
                    let next = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let sources = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                            {
                                let labels = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                {
                                    let order = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                    {
                                        let node = 0f64;
                                        {
                                            let source_ml_source1_s9 = self::source_ml_temporary_plan_datatype_dependency_groups_loop2(count, heads.clone(), seen.clone(), labels.clone(), node);
                                            match source_ml_source1_s9.clone() {
                                                self::MlTemporaryPlanDatatypeDependencyGroupsLoop2Result::MlTemporaryPlanDatatypeDependencyGroupsLoop2Done(heads_2, seen_2, labels_2) => {
                                                    {
                                                        let first = 0f64;
                                                        {
                                                            let node_2 = 0f64;
                                                            {
                                                                let source_ml_source1_s10 = self::source_ml_temporary_plan_datatype_dependency_groups_loop3(count, ends.clone(), edges.clone(), starts.clone(), heads_2.clone(), next.clone(), sources.clone(), first, node_2);
                                                                match source_ml_source1_s10.clone() {
                                                                    self::MlTemporaryPlanDatatypeDependencyGroupsLoop3Result::MlTemporaryPlanDatatypeDependencyGroupsLoop3Done(starts_2, next_2, sources_2, first_2) => {
                                                                        if (first_2 != ({ let ml_reference_array9 = edges.clone(); let ml_reference_projection9 = ml_reference_array9.borrow().len() as f64; ml_reference_projection9 })) {
                                                                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                                                        } else {
                                                                            {
                                                                                let root = 0f64;
                                                                                {
                                                                                    let order_2 = self::source_ml_temporary_plan_datatype_dependency_groups_loop4(count, ends.clone(), edges.clone(), starts_2.clone(), seen_2.clone(), order.clone(), root);
                                                                                    {
                                                                                        let at = ({ let ml_reference_array10 = order_2.clone(); let ml_reference_projection10 = ml_reference_array10.borrow().len() as f64; ml_reference_projection10 } - 1f64);
                                                                                        {
                                                                                            let source_ml_source1_s11 = self::source_ml_temporary_plan_datatype_dependency_groups_loop11(heads_2.clone(), next_2.clone(), sources_2.clone(), labels_2.clone(), order_2.clone(), at);
                                                                                            match source_ml_source1_s11.clone() {
                                                                                                self::MlTemporaryPlanDatatypeDependencyGroupsLoop11Result::MlTemporaryPlanDatatypeDependencyGroupsLoop11Done => {
                                                                                                    labels_2.clone()
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlTemporaryPlanDatatypeDependencyGroupsLoop3Result::MlTemporaryPlanDatatypeDependencyGroupsLoop3Return(source_ml_temporary_result17) => {
                                                                        source_ml_temporary_result17.clone()
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

pub fn source_ml_temporary_plan_datatype_dependency_groups_loop6(mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut seen: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut order: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut stack: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut positions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if { let ml_value = { let ml_reference_array12 = stack.clone(); let ml_reference_projection12 = ml_reference_array12.borrow().len() as f64; ml_reference_projection12 }; ml_value != 0f64 && !ml_value.is_nan() } {
            {
                let top = ({ let ml_reference_array13 = stack.clone(); let ml_reference_projection13 = ml_reference_array13.borrow().len() as f64; ml_reference_projection13 } - 1f64);
                {
                    let node = self::read_required_number_fact(stack.clone(), top);
                    {
                        let position = self::read_required_number_fact(positions.clone(), top);
                        if (position < (self::read_required_number_fact(ends.clone(), node))) {
                            {
                                let source_ml_temporary_plan_datatype_dependency_groups_ignored7 = { let ml_array_owner14 = positions.clone(); let ml_array_argument14_0: f64 = top; let ml_array_argument14_1: f64 = 1f64; let ml_array_argument14_2: f64 = (position + 1f64); let ml_array_length = ml_array_owner14.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument14_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument14_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument14_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner14.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                                {
                                    let target = self::read_required_number_fact(edges.clone(), position);
                                    if ((self::read_required_number_fact(seen.clone(), target)) == (0f64)) {
                                        {
                                            let source_ml_temporary_plan_datatype_dependency_groups_ignored8 = { let ml_array_owner15 = seen.clone(); let ml_array_argument15_0: f64 = target; let ml_array_argument15_1: f64 = 1f64; let ml_array_argument15_2: f64 = 1f64; let ml_array_length = ml_array_owner15.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument15_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument15_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument15_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner15.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                                            {
                                                let stack_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value16 = stack.borrow().clone(); ml_reference_value16 }, vec![target])));
                                                {
                                                    let source_ml_temporary_plan_datatype_dependency_groups_ignored9 = { let ml_method_owner17 = positions.clone(); let ml_method_argument17_0 = self::read_required_number_fact(starts.clone(), target); let ml_inserted = vec![ml_method_argument17_0]; let mut ml_method_values17 = ml_method_owner17.borrow_mut(); let ml_length = ml_method_values17.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values17.extend(ml_inserted); ml_length as f64 };
                                                    {
                                                        stack = stack_2.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let stack_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value18 = stack.borrow().clone(); ml_reference_value18 }, 0f64, (-1f64))));
                                {
                                    let source_ml_temporary_plan_datatype_dependency_groups_ignored10 = { let ml_method_owner19 = positions.clone(); let ml_result = ml_method_owner19.borrow_mut().pop(); ml_result };
                                    {
                                        let order_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value20 = order.borrow().clone(); ml_reference_value20 }, vec![node])));
                                        {
                                            (order, stack) = (order_2.clone(), stack_3.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            order.clone()
        };
    }
}

pub fn source_ml_temporary_plan_datatype_dependency_groups_loop4(mut count: f64, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut seen: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut order: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut root: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (root < count) {
            if ((self::read_required_number_fact(seen.clone(), root)) != (0f64)) {
                {
                    let root_2 = (root + 1f64);
                    {
                        root = root_2;
                        continue;
                    }
                }
            } else {
                {
                    let stack = std::rc::Rc::new(std::cell::RefCell::new(vec![root]));
                    {
                        let positions = std::rc::Rc::new(std::cell::RefCell::new(vec![self::read_required_number_fact(starts.clone(), root)]));
                        {
                            let source_ml_temporary_plan_datatype_dependency_groups_ignored5 = { let ml_array_owner21 = seen.clone(); let ml_array_argument21_0: f64 = root; let ml_array_argument21_1: f64 = 1f64; let ml_array_argument21_2: f64 = 1f64; let ml_array_length = ml_array_owner21.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument21_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument21_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument21_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner21.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                            {
                                let order_2 = self::source_ml_temporary_plan_datatype_dependency_groups_loop6(ends.clone(), edges.clone(), starts.clone(), seen.clone(), order.clone(), stack.clone(), positions.clone());
                                {
                                    let root_3 = (root + 1f64);
                                    {
                                        (order, root) = (order_2.clone(), root_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            order.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanDatatypeDependencyGroupsLoop11Result {
    MlTemporaryPlanDatatypeDependencyGroupsLoop11Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanDatatypeDependencyGroupsLoop14Result {
    MlTemporaryPlanDatatypeDependencyGroupsLoop14Done,
}

pub fn source_ml_temporary_plan_datatype_dependency_groups_loop14(mut labels: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut representative: f64, mut source_ml_temporary_values1: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_temporary_index1: self::ml::Big) -> self::MlTemporaryPlanDatatypeDependencyGroupsLoop14Result {
    loop {
        return if (source_ml_temporary_index1 < ({ let ml_reference_array22 = source_ml_temporary_values1.clone(); let ml_reference_projection22 = self::ml::Big::from_u128(ml_reference_array22.borrow().len() as u128); ml_reference_projection22 })) {
            {
                let member = {
                    let source_ml_source1_array_read_value12 = source_ml_temporary_values1.clone();
                    {
                        let source_ml_source1_array_read_index13 = source_ml_temporary_index1.clone();
                        { let ml_reference_array23 = source_ml_source1_array_read_value12.clone(); let ml_reference_position23 = source_ml_source1_array_read_index13.clone(); let ml_reference_projection23 = self::ml_array::at(&*ml_reference_array23.borrow(), ml_reference_position23.to_index()); ml_reference_projection23 }
                    }
                };
                {
                    let source_ml_temporary_plan_datatype_dependency_groups_ignored15 = { let ml_array_owner24 = labels.clone(); let ml_array_argument24_0: f64 = member; let ml_array_argument24_1: f64 = 1f64; let ml_array_argument24_2: f64 = representative; let ml_array_length = ml_array_owner24.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument24_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument24_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument24_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner24.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                    {
                        let source_ml_temporary_index1_2 = source_ml_temporary_index1.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index1 = source_ml_temporary_index1_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanDatatypeDependencyGroupsLoop14Result::MlTemporaryPlanDatatypeDependencyGroupsLoop14Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanDatatypeDependencyGroupsLoop13Result {
    MlTemporaryPlanDatatypeDependencyGroupsLoop13Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, f64),
}

pub fn source_ml_temporary_plan_datatype_dependency_groups_loop16(mut next: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut sources: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut root: f64, mut stack: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edge: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (edge >= (0f64)) {
            {
                let source = self::read_required_number_fact(sources.clone(), edge);
                {
                    let stack_3 = if ((self::read_required_number_fact(labels.clone(), source)) < (0f64)) {
                        {
                            let source_ml_temporary_plan_datatype_dependency_groups_ignored17 = { let ml_array_owner25 = labels.clone(); let ml_array_argument25_0: f64 = source; let ml_array_argument25_1: f64 = 1f64; let ml_array_argument25_2: f64 = root; let ml_array_length = ml_array_owner25.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument25_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument25_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument25_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner25.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                            {
                                let stack_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value26 = stack.borrow().clone(); ml_reference_value26 }, vec![source])));
                                stack_2.clone()
                            }
                        }
                    } else {
                        stack.clone()
                    };
                    {
                        let edge_2 = self::read_required_number_fact(next.clone(), edge);
                        {
                            (stack, edge) = (stack_3.clone(), edge_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            stack.clone()
        };
    }
}

pub fn source_ml_temporary_plan_datatype_dependency_groups_loop13(mut heads: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut next: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut sources: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut root: f64, mut stack: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut members: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut representative: f64) -> self::MlTemporaryPlanDatatypeDependencyGroupsLoop13Result {
    loop {
        return if { let ml_value = { let ml_reference_array28 = stack.clone(); let ml_reference_projection28 = ml_reference_array28.borrow().len() as f64; ml_reference_projection28 }; ml_value != 0f64 && !ml_value.is_nan() } {
            {
                let node = self::read_required_number_fact(stack.clone(), ({ let ml_reference_array29 = stack.clone(); let ml_reference_projection29 = ml_reference_array29.borrow().len() as f64; ml_reference_projection29 } - 1f64));
                {
                    let stack_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value30 = stack.borrow().clone(); ml_reference_value30 }, 0f64, (-1f64))));
                    {
                        let members_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value31 = members.borrow().clone(); ml_reference_value31 }, vec![node])));
                        {
                            let representative_2 = self::ml_math::min(representative, node);
                            {
                                let edge = self::read_required_number_fact(heads.clone(), node);
                                {
                                    let stack_3 = self::source_ml_temporary_plan_datatype_dependency_groups_loop16(next.clone(), sources.clone(), labels.clone(), root, stack_2.clone(), edge);
                                    {
                                        (stack, members, representative) = (stack_3.clone(), members_2.clone(), representative_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanDatatypeDependencyGroupsLoop13Result::MlTemporaryPlanDatatypeDependencyGroupsLoop13Done(members.clone(), representative)
        };
    }
}

pub fn source_ml_temporary_plan_datatype_dependency_groups_loop11(mut heads: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut next: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut sources: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut order: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlTemporaryPlanDatatypeDependencyGroupsLoop11Result {
    loop {
        return if (at >= (0f64)) {
            {
                let root = self::read_required_number_fact(order.clone(), at);
                if ((self::read_required_number_fact(labels.clone(), root)) >= (0f64)) {
                    {
                        let at_2 = (at - 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                } else {
                    {
                        let stack = std::rc::Rc::new(std::cell::RefCell::new(vec![root]));
                        {
                            let members = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                            {
                                let source_ml_temporary_plan_datatype_dependency_groups_ignored12 = { let ml_array_owner32 = labels.clone(); let ml_array_argument32_0: f64 = root; let ml_array_argument32_1: f64 = 1f64; let ml_array_argument32_2: f64 = root; let ml_array_length = ml_array_owner32.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument32_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument32_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument32_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner32.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                                {
                                    let representative = root;
                                    {
                                        let source_ml_source1_s14 = self::source_ml_temporary_plan_datatype_dependency_groups_loop13(heads.clone(), next.clone(), sources.clone(), labels.clone(), root, stack.clone(), members.clone(), representative);
                                        match source_ml_source1_s14.clone() {
                                            self::MlTemporaryPlanDatatypeDependencyGroupsLoop13Result::MlTemporaryPlanDatatypeDependencyGroupsLoop13Done(members_2, representative_2) => {
                                                {
                                                    let source_ml_temporary_values1 = members_2.clone();
                                                    {
                                                        let source_ml_temporary_index1 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source1_s15 = self::source_ml_temporary_plan_datatype_dependency_groups_loop14(labels.clone(), representative_2, source_ml_temporary_values1.clone(), source_ml_temporary_index1.clone());
                                                            match source_ml_source1_s15.clone() {
                                                                self::MlTemporaryPlanDatatypeDependencyGroupsLoop14Result::MlTemporaryPlanDatatypeDependencyGroupsLoop14Done => {
                                                                    {
                                                                        let at_3 = (at - 1f64);
                                                                        {
                                                                            at = at_3;
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
            }
        } else {
            self::MlTemporaryPlanDatatypeDependencyGroupsLoop11Result::MlTemporaryPlanDatatypeDependencyGroupsLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanDatatypeDependencyGroupsLoop3Result {
    MlTemporaryPlanDatatypeDependencyGroupsLoop3Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>, f64),
    MlTemporaryPlanDatatypeDependencyGroupsLoop3Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanDatatypeDependencyGroupsLoop18Result {
    MlTemporaryPlanDatatypeDependencyGroupsLoop18Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    MlTemporaryPlanDatatypeDependencyGroupsLoop18Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_temporary_plan_datatype_dependency_groups_loop18(mut count: f64, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut heads: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut next: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut sources: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64, mut last: f64, mut edge: f64) -> self::MlTemporaryPlanDatatypeDependencyGroupsLoop18Result {
    loop {
        return if (edge < last) {
            {
                let target = self::read_required_number_fact(edges.clone(), edge);
                if (((!(self::ml_math::is_integer(target)) || {
                    let source_ml_source1_same_value_operand16 = target;
                    {
                        let source_ml_source1_same_value_operand17 = (-0f64);
                        (((source_ml_source1_same_value_operand16 == source_ml_source1_same_value_operand17) && ((source_ml_source1_same_value_operand16 != (0f64)) || (((1f64 / source_ml_source1_same_value_operand16)) == ((1f64 / source_ml_source1_same_value_operand17))))) || ((source_ml_source1_same_value_operand16 != source_ml_source1_same_value_operand16) && (source_ml_source1_same_value_operand17 != source_ml_source1_same_value_operand17)))
                    }
                }) || (target < (0f64))) || (target >= count)) {
                    self::MlTemporaryPlanDatatypeDependencyGroupsLoop18Result::MlTemporaryPlanDatatypeDependencyGroupsLoop18Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                } else {
                    {
                        let next_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value33 = next.borrow().clone(); ml_reference_value33 }, vec![self::read_required_number_fact(heads.clone(), target)])));
                        {
                            let sources_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value34 = sources.borrow().clone(); ml_reference_value34 }, vec![node])));
                            {
                                let source_ml_temporary_plan_datatype_dependency_groups_ignored19 = { let ml_array_owner35 = heads.clone(); let ml_array_argument35_0: f64 = target; let ml_array_argument35_1: f64 = 1f64; let ml_array_argument35_2: f64 = edge; let ml_array_length = ml_array_owner35.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument35_0; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_count = { let ml_delete_value: f64 = ml_array_argument35_1; ml_delete_value.trunc().max(0f64).min((ml_array_length - ml_array_start) as f64) as usize }; let ml_array_inserted = vec![ml_array_argument35_2]; let _ = (ml_array_length - ml_array_count).checked_add(ml_array_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); let ml_removed: Vec<_> = ml_array_owner35.borrow_mut().splice(ml_array_start..ml_array_start + ml_array_count, ml_array_inserted).collect(); std::rc::Rc::new(std::cell::RefCell::new(ml_removed)) };
                                {
                                    let edge_2 = (edge + 1f64);
                                    {
                                        (next, sources, edge) = (next_2.clone(), sources_2.clone(), edge_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanDatatypeDependencyGroupsLoop18Result::MlTemporaryPlanDatatypeDependencyGroupsLoop18Done(next.clone(), sources.clone())
        };
    }
}

pub fn source_ml_temporary_plan_datatype_dependency_groups_loop3(mut count: f64, mut ends: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut edges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut starts: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut heads: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut next: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut sources: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut first: f64, mut node: f64) -> self::MlTemporaryPlanDatatypeDependencyGroupsLoop3Result {
    loop {
        return if (node < count) {
            {
                let last = self::read_required_number_fact(ends.clone(), node);
                if (((!(self::ml_math::is_integer(last)) || {
                    let source_ml_source1_same_value_operand18 = last;
                    {
                        let source_ml_source1_same_value_operand19 = (-0f64);
                        (((source_ml_source1_same_value_operand18 == source_ml_source1_same_value_operand19) && ((source_ml_source1_same_value_operand18 != (0f64)) || (((1f64 / source_ml_source1_same_value_operand18)) == ((1f64 / source_ml_source1_same_value_operand19))))) || ((source_ml_source1_same_value_operand18 != source_ml_source1_same_value_operand18) && (source_ml_source1_same_value_operand19 != source_ml_source1_same_value_operand19)))
                    }
                }) || (last < first)) || (last > ({ let ml_reference_array36 = edges.clone(); let ml_reference_projection36 = ml_reference_array36.borrow().len() as f64; ml_reference_projection36 }))) {
                    self::MlTemporaryPlanDatatypeDependencyGroupsLoop3Result::MlTemporaryPlanDatatypeDependencyGroupsLoop3Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                } else {
                    {
                        let starts_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value37 = starts.borrow().clone(); ml_reference_value37 }, vec![first])));
                        {
                            let edge = first;
                            {
                                let source_ml_source1_s20 = self::source_ml_temporary_plan_datatype_dependency_groups_loop18(count, edges.clone(), heads.clone(), next.clone(), sources.clone(), node, last, edge);
                                match source_ml_source1_s20.clone() {
                                    self::MlTemporaryPlanDatatypeDependencyGroupsLoop18Result::MlTemporaryPlanDatatypeDependencyGroupsLoop18Done(next_2, sources_2) => {
                                        {
                                            let first_2 = last;
                                            {
                                                let node_2 = (node + 1f64);
                                                {
                                                    (starts, next, sources, first, node) = (starts_2.clone(), next_2.clone(), sources_2.clone(), first_2, node_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanDatatypeDependencyGroupsLoop18Result::MlTemporaryPlanDatatypeDependencyGroupsLoop18Return(source_ml_temporary_result18) => {
                                        self::MlTemporaryPlanDatatypeDependencyGroupsLoop3Result::MlTemporaryPlanDatatypeDependencyGroupsLoop3Return(source_ml_temporary_result18.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanDatatypeDependencyGroupsLoop3Result::MlTemporaryPlanDatatypeDependencyGroupsLoop3Done(starts.clone(), next.clone(), sources.clone(), first)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanDatatypeDependencyGroupsLoop2Result {
    MlTemporaryPlanDatatypeDependencyGroupsLoop2Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_temporary_plan_datatype_dependency_groups_loop2(mut count: f64, mut heads: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut seen: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut labels: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut node: f64) -> self::MlTemporaryPlanDatatypeDependencyGroupsLoop2Result {
    loop {
        return if (node < count) {
            {
                let heads_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value38 = heads.borrow().clone(); ml_reference_value38 }, vec![(-1f64)])));
                {
                    let seen_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value39 = seen.borrow().clone(); ml_reference_value39 }, vec![0f64])));
                    {
                        let labels_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value40 = labels.borrow().clone(); ml_reference_value40 }, vec![(-1f64)])));
                        {
                            let node_2 = (node + 1f64);
                            {
                                (heads, seen, labels, node) = (heads_2.clone(), seen_2.clone(), labels_2.clone(), node_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanDatatypeDependencyGroupsLoop2Result::MlTemporaryPlanDatatypeDependencyGroupsLoop2Done(heads.clone(), seen.clone(), labels.clone())
        };
    }
}

fn ml_main() {
    assert!(((self::plan_datatype_dependency_groups(0f64, std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 1");
    assert!(((self::plan_datatype_dependency_groups(1f64, std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])))), "assertion 2");
    assert!(((self::plan_datatype_dependency_groups(2f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64])))), "assertion 3");
    assert!(((self::plan_datatype_dependency_groups(3f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 2f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64, 2f64])))), "assertion 4");
    assert!(((self::plan_datatype_dependency_groups(3f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64, 0f64])))), "assertion 5");
    assert!(((self::plan_datatype_dependency_groups(4f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 3f64, 4f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 0f64, 3f64, 2f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64, 2f64, 2f64])))), "assertion 6");
    assert!(((self::plan_datatype_dependency_groups(4f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 3f64, 3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 3f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64, 3f64])))), "assertion 7");
    assert!(((self::plan_datatype_dependency_groups(5f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 3f64, 4f64, 4f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 0f64, 3f64, 2f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64, 2f64, 2f64, 4f64])))), "assertion 8");
    assert!(((self::plan_datatype_dependency_groups(4f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 3f64, 4f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64, 3f64, 0f64, 1f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 0f64, 1f64])))), "assertion 9");
    assert!(((self::plan_datatype_dependency_groups(2f64, std::rc::Rc::new(std::cell::RefCell::new(vec![2f64, 3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 1f64, 0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64])))), "assertion 10");
    assert!(((self::plan_datatype_dependency_groups(3f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 3f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])))), "assertion 11");
    assert!(((self::plan_datatype_dependency_groups(2f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 0f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 12");
    assert!(((self::plan_datatype_dependency_groups(1f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 13");
    assert!(((self::plan_datatype_dependency_groups(1f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 14");
    assert!(((self::plan_datatype_dependency_groups(1f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0.5f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 15");
    assert!(((self::plan_datatype_dependency_groups((-1f64), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 16");
    assert!(((self::plan_datatype_dependency_groups(8193f64, std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 17");
    assert!(((self::plan_datatype_dependency_groups(2f64, std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 18");
    assert!(((self::plan_datatype_dependency_groups(1f64, std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 19");
    assert!(((self::plan_datatype_dependency_groups(1f64, std::rc::Rc::new(std::cell::RefCell::new(vec![(-0f64)])), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 20");
    assert!(((self::plan_datatype_dependency_groups(1f64, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])), std::rc::Rc::new(std::cell::RefCell::new(vec![(-0f64)])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 21");
}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(move || { ml_main() })
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}

#[test]
fn all_literal_datatype_dependency_group_controls() { main(); }
