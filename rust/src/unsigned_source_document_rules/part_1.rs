// Generated from complete js/src/unsigned-source-document-rules.js. Do not edit by hand.
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
pub fn read_unsigned_link_limb(words: &[f64], index: f64) -> f64 {
    (self::ml_array::optional_at(words, self::ml_array::number_index(index)))
        .unwrap_or_else(|| -1f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_unsigned_link_identity(words: &[f64]) -> bool {
    {
        let low = self::read_unsigned_link_limb(words, 0f64);
        {
            let high = self::read_unsigned_link_limb(words, 1f64);
            (((((((((words.len() as f64) == 2f64) && self::ml_math::is_integer(low))
                && (low >= 0f64))
                && (low < 4_294_967_296f64))
                && self::ml_math::is_integer(high))
                && (high >= 0f64))
                && (high < 4_294_967_296f64))
                && ((low != 0f64) || (high != 0f64)))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn advance_unsigned_link_identity(words: &[f64]) -> Vec<f64> {
    if (self::accept_unsigned_link_identity(words)) {
        if ((self::read_unsigned_link_limb(words, 0f64)) == 4_294_967_295f64)
            && ((self::read_unsigned_link_limb(words, 1f64)) == 4_294_967_295f64)
        {
            Vec::<f64>::new()
        } else if (self::read_unsigned_link_limb(words, 0f64)) == 4_294_967_295f64 {
            vec![0f64, (self::read_unsigned_link_limb(words, 1f64) + 1f64)]
        } else {
            vec![
                (self::read_unsigned_link_limb(words, 0f64) + 1f64),
                self::read_unsigned_link_limb(words, 1f64),
            ]
        }
    } else {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid unsigned link identity"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_unsigned_source_allocation_error(first: &[f64], count: f64, limit: f64) -> String {
    if (self::accept_unsigned_link_identity(first)) {
        if ((((!(self::ml_math::is_safe_integer(limit)) || (limit < 1f64))
            || !(self::ml_math::is_integer(count)))
            || (count < 0f64))
            || (count > 4_294_967_295f64))
            || (count > limit)
        {
            String::from("source graph allocation exceeds its identity directory limit")
        } else {
            {
                let lower = self::read_unsigned_link_limb(first, 0f64)
                    + if count > 0f64 { count - 1f64 } else { 0f64 };
                {
                    let upper = self::read_unsigned_link_limb(first, 1f64)
                        + (lower / 4_294_967_296f64).floor();
                    if upper >= 4_294_967_296f64 {
                        String::from("source graph allocation exceeds the unsigned identity space")
                    } else {
                        String::new()
                    }
                }
            }
        }
    } else {
        String::from("source graph allocation needs a positive unsigned first identity")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn allocate_unsigned_source_identities(first: &[f64], count: f64, limit: f64) -> Vec<f64> {
    {
        let ml_library_inequality_1 =
            (self::read_unsigned_source_allocation_error(first, count, limit)) != "";
        if ml_library_inequality_1 {
            Vec::<f64>::new()
        } else {
            {
                let cursor = first.to_vec();
                {
                    let identities = Vec::<f64>::new();
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s10 = self::source_ml_temporary_allocate_unsigned_source_identities_loop21(count, cursor, identities, at);
                            match source_ml_source1_s10 {
                            self::MlTemporaryAllocateUnsignedSourceIdentitiesLoop21Result::MlTemporaryAllocateUnsignedSourceIdentitiesLoop21Done(cursor_2, identities_2) => {
                                {
                                    let source_ml_source1_concat_value11 = vec![count, if (cursor_2.len() as f64) == 0f64 {
                                        -1f64
                                    } else {
                                        self::read_unsigned_link_limb(&(cursor_2), 0f64)
                                    }, if (cursor_2.len() as f64) == 0f64 {
                                        -1f64
                                    } else {
                                        self::read_unsigned_link_limb(&(cursor_2), 1f64)
                                    }];
                                    {
                                        let source_ml_source1_concat_value12 = identities_2;
                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value11), source_ml_source1_concat_value12)
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAllocateUnsignedSourceIdentitiesLoop21Result {
    MlTemporaryAllocateUnsignedSourceIdentitiesLoop21Done(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_allocate_unsigned_source_identities_loop21(
    mut count: f64,
    mut cursor: Vec<f64>,
    mut identities: Vec<f64>,
    mut at: f64,
) -> self::MlTemporaryAllocateUnsignedSourceIdentitiesLoop21Result {
    loop {
        if at < count {
            {
                let identities_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![self::read_unsigned_link_limb(&(cursor), 0f64)],
                        identities,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let identities_3 = {
                        let (ml_append_right, ml_append_left) = (
                            vec![self::read_unsigned_link_limb(&(cursor), 1f64)],
                            identities_2,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let cursor_2 = self::advance_unsigned_link_identity(&(cursor));
                        {
                            let at_2 = at + 1f64;
                            {
                                (cursor, identities, at) = (cursor_2, identities_3, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryAllocateUnsignedSourceIdentitiesLoop21Result::MlTemporaryAllocateUnsignedSourceIdentitiesLoop21Done(cursor, identities)
        };
    }
}
