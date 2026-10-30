// Generated from complete js/src/translation/rust-nominal-copy-rules.js. Do not edit by hand.
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
pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required Boolean fact column value"))
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
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required Boolean fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_boolean_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_nominal_copy_owners(
    owners: &[String],
    copies: &[bool],
    clones: &[bool],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    leaves: &[bool],
) -> Vec<f64> {
    {
        let size = owners.len() as f64;
        if (((((copies.len() as f64) != size) || ((clones.len() as f64) != size))
            || ((starts.len() as f64) != size))
            || ((counts.len() as f64) != size))
            || ((targets.len() as f64) != (leaves.len() as f64))
        {
            vec![-1f64]
        } else {
            {
                let accepted = Vec::<f64>::new();
                {
                    let expected_start = 0f64;
                    {
                        let owner = 0f64;
                        {
                            let source_ml_source1_s1 =
                                self::source_ml_temporary_plan_rust_nominal_copy_owners_loop1(
                                    owners,
                                    copies,
                                    clones,
                                    starts,
                                    counts,
                                    targets,
                                    size,
                                    expected_start,
                                    owner,
                                );
                            match source_ml_source1_s1 {
                                self::MlTemporaryPlanRustNominalCopyOwnersLoop1Result::MlTemporaryPlanRustNominalCopyOwnersLoop1Done(expected_start_2) => {
                                    {
let ml_library_inequality_1 = expected_start_2 != (targets.len() as f64) ;
if ml_library_inequality_1 {
                                        vec![-1f64]
                                    } else {
                                        {
                                            let field = 0f64;
                                            {
                                                let source_ml_source1_s2 = self::source_ml_temporary_plan_rust_nominal_copy_owners_loop2(targets, size, field);
                                                match source_ml_source1_s2 {
                                                    self::MlTemporaryPlanRustNominalCopyOwnersLoop2Result::MlTemporaryPlanRustNominalCopyOwnersLoop2Done => {
                                                        {
                                                            let pass = 0f64;
                                                            {
                                                                let accepted_2 = self::source_ml_temporary_plan_rust_nominal_copy_owners_loop3(copies, starts, counts, targets, leaves, size, accepted, pass);
                                                                {
                                                                    let owner_2 = 0f64;
                                                                    {
                                                                        let source_ml_source1_s3 = self::source_ml_temporary_plan_rust_nominal_copy_owners_loop6(copies, size, &(accepted_2), owner_2);
                                                                        match source_ml_source1_s3 {
                                                                            self::MlTemporaryPlanRustNominalCopyOwnersLoop6Result::MlTemporaryPlanRustNominalCopyOwnersLoop6Done => {
                                                                                {
                                                                                    let result = vec![0f64];
                                                                                    {
                                                                                        let owner_3 = 0f64;
                                                                                        {
                                                                                            self::source_ml_temporary_plan_rust_nominal_copy_owners_loop7(size, &(accepted_2), result, owner_3)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlTemporaryPlanRustNominalCopyOwnersLoop6Result::MlTemporaryPlanRustNominalCopyOwnersLoop6Return(source_ml_temporary_result7) => {
                                                                                source_ml_temporary_result7
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryPlanRustNominalCopyOwnersLoop2Result::MlTemporaryPlanRustNominalCopyOwnersLoop2Return(source_ml_temporary_result7_2) => {
                                                        source_ml_temporary_result7_2
                                                    }
                                                }
                                            }
                                        }
                                    }
}
                                }
                                self::MlTemporaryPlanRustNominalCopyOwnersLoop1Result::MlTemporaryPlanRustNominalCopyOwnersLoop1Return(source_ml_temporary_result7_3) => {
                                    source_ml_temporary_result7_3
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
pub fn accept_rust_nominal_copy_field_leaf(
    kind: &str,
    wrappers: &[String],
    primitive_copy: bool,
) -> bool {
    if (wrappers.len() as f64) > 0f64 {
        (self::read_required_text_fact(wrappers, 0f64)) == "reference"
    } else {
        primitive_copy
            && self::ml_array::includes(
                &vec![
                    String::from("bool"),
                    String::from("float"),
                    String::from("fixed"),
                    String::from("unit"),
                    String::from("function"),
                ],
                &kind.to_owned(),
                0f64,
                false,
                |a, b| a == b,
            )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_nominal_copy_owners_loop5(
    targets: &[f64],
    leaves: &[bool],
    accepted: &[f64],
    mut ready: bool,
    mut start: f64,
    mut count: f64,
    mut at: f64,
) -> bool {
    loop {
        if at < (start + count) {
            {
                let target = self::read_required_number_fact(targets, at);
                if !((self::ml_array::optional_at(leaves, self::ml_array::number_index(at)))
                    .unwrap_or_default())
                    && ((target < 0f64)
                        || !(self::ml_array::includes(
                            accepted,
                            &target,
                            0f64,
                            false,
                            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                        )))
                {
                    {
                        let ready_2 = false;
                        {
                            let at_2 = at + 1f64;
                            {
                                (ready, at) = (ready_2, at_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let at_3 = at + 1f64;
                        {
                            at = at_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { ready };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_nominal_copy_owners_loop4(
    copies: &[bool],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    leaves: &[bool],
    mut size: f64,
    mut accepted: Vec<f64>,
    mut owner: f64,
) -> Vec<f64> {
    loop {
        if owner < size {
            if !((self::ml_array::optional_at(copies, self::ml_array::number_index(owner)))
                .unwrap_or_default())
                || self::ml_array::includes(&accepted, &owner, 0f64, false, |a: &f64, b: &f64| {
                    a == b || (a.is_nan() && b.is_nan())
                })
            {
                {
                    let owner_2 = owner + 1f64;
                    {
                        owner = owner_2;
                        continue;
                    }
                }
            }
            {
                {
                    let ready = true;
                    {
                        let start = self::read_required_number_fact(starts, owner);
                        {
                            let count = self::read_required_number_fact(counts, owner);
                            {
                                let at = start;
                                {
                                    let ready_2 = self::source_ml_temporary_plan_rust_nominal_copy_owners_loop5(targets, leaves, &(accepted), ready, start, count, at);
                                    if ready_2 {
                                        {
                                            let accepted_2 = {
                                                let (ml_append_right, ml_append_left) =
                                                    (vec![owner], accepted);
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            };
                                            {
                                                let owner_3 = owner + 1f64;
                                                {
                                                    (accepted, owner) = (accepted_2, owner_3);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    {
                                        {
                                            let owner_4 = owner + 1f64;
                                            {
                                                owner = owner_4;
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
        return { accepted };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_nominal_copy_owners_loop3(
    copies: &[bool],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    leaves: &[bool],
    mut size: f64,
    mut accepted: Vec<f64>,
    mut pass: f64,
) -> Vec<f64> {
    loop {
        return if pass < size {
            {
                let before = accepted.len() as f64;
                {
                    let owner = 0f64;
                    {
                        let accepted_2 =
                            self::source_ml_temporary_plan_rust_nominal_copy_owners_loop4(
                                copies, starts, counts, targets, leaves, size, accepted, owner,
                            );
                        if (accepted_2.len() as f64) == before {
                            accepted_2
                        } else {
                            {
                                let pass_2 = pass + 1f64;
                                {
                                    (accepted, pass) = (accepted_2, pass_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            accepted
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_nominal_copy_owners_loop7(
    mut size: f64,
    accepted: &[f64],
    mut result: Vec<f64>,
    mut owner: f64,
) -> Vec<f64> {
    loop {
        if owner < size {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![if self::ml_array::includes(
                            accepted,
                            &owner,
                            0f64,
                            false,
                            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                        ) {
                            1f64
                        } else {
                            0f64
                        }],
                        result,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let owner_2 = owner + 1f64;
                    {
                        (result, owner) = (result_2, owner_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustNominalCopyOwnersLoop6Result {
    MlTemporaryPlanRustNominalCopyOwnersLoop6Done,
    MlTemporaryPlanRustNominalCopyOwnersLoop6Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_nominal_copy_owners_loop6(
    copies: &[bool],
    mut size: f64,
    accepted: &[f64],
    mut owner: f64,
) -> self::MlTemporaryPlanRustNominalCopyOwnersLoop6Result {
    loop {
        return if owner < size {
            if match {
                let source_ml_source1_logical4 =
                    self::ml_array::optional_at(copies, self::ml_array::number_index(owner));
                if (source_ml_source1_logical4.clone()).unwrap_or_default() {
                    Some(
                        !(self::ml_array::includes(
                            accepted,
                            &owner,
                            0f64,
                            false,
                            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                        )),
                    )
                } else {
                    source_ml_source1_logical4.clone()
                }
            } {
                Some(ml_present) => ml_present,
                None => false,
            } {
                self::MlTemporaryPlanRustNominalCopyOwnersLoop6Result::MlTemporaryPlanRustNominalCopyOwnersLoop6Return(vec![-3f64, owner])
            } else {
                {
                    let owner_2 = owner + 1f64;
                    {
                        owner = owner_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalCopyOwnersLoop6Result::MlTemporaryPlanRustNominalCopyOwnersLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustNominalCopyOwnersLoop2Result {
    MlTemporaryPlanRustNominalCopyOwnersLoop2Done,
    MlTemporaryPlanRustNominalCopyOwnersLoop2Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_nominal_copy_owners_loop2(
    targets: &[f64],
    mut size: f64,
    mut field: f64,
) -> self::MlTemporaryPlanRustNominalCopyOwnersLoop2Result {
    loop {
        return if field < (targets.len() as f64) {
            {
                let target = self::read_required_number_fact(targets, field);
                if (!(self::ml_math::is_integer(target)) || (target < -1f64)) || (target >= size) {
                    self::MlTemporaryPlanRustNominalCopyOwnersLoop2Result::MlTemporaryPlanRustNominalCopyOwnersLoop2Return(vec![-1f64])
                } else {
                    {
                        let field_2 = field + 1f64;
                        {
                            field = field_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNominalCopyOwnersLoop2Result::MlTemporaryPlanRustNominalCopyOwnersLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustNominalCopyOwnersLoop1Result {
    MlTemporaryPlanRustNominalCopyOwnersLoop1Done(f64),
    MlTemporaryPlanRustNominalCopyOwnersLoop1Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_nominal_copy_owners_loop1(
    owners: &[String],
    copies: &[bool],
    clones: &[bool],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut size: f64,
    mut expected_start: f64,
    mut owner: f64,
) -> self::MlTemporaryPlanRustNominalCopyOwnersLoop1Result {
    loop {
        return if owner < size {
            {
                let name = self::read_required_text_fact(owners, owner);
                {
                    let start = self::read_required_number_fact(starts, owner);
                    {
                        let count = self::read_required_number_fact(counts, owner);
                        if (((((((name).is_empty())
                            || ((self::ml_array::index_of(
                                owners,
                                &name.clone(),
                                0f64,
                                false,
                                |a, b| a == b,
                            )) != owner))
                            || !(self::ml_math::is_integer(start)))
                            || (start != expected_start))
                            || !(self::ml_math::is_integer(count)))
                            || (count < 0f64))
                            || ((start + count) > (targets.len() as f64))
                        {
                            self::MlTemporaryPlanRustNominalCopyOwnersLoop1Result::MlTemporaryPlanRustNominalCopyOwnersLoop1Return(vec![-1f64, owner])
                        } else {
                            if match {
                                let source_ml_source1_logical5 = self::ml_array::optional_at(
                                    copies,
                                    self::ml_array::number_index(owner),
                                );
                                if (source_ml_source1_logical5.clone()).unwrap_or_default() {
                                    Some(
                                        !((self::ml_array::optional_at(
                                            clones,
                                            self::ml_array::number_index(owner),
                                        ))
                                        .unwrap_or_default()),
                                    )
                                } else {
                                    source_ml_source1_logical5.clone()
                                }
                            } {
                                Some(ml_present) => ml_present,
                                None => false,
                            } {
                                self::MlTemporaryPlanRustNominalCopyOwnersLoop1Result::MlTemporaryPlanRustNominalCopyOwnersLoop1Return(vec![-2f64, owner])
                            } else {
                                {
                                    let expected_start_2 = expected_start + count;
                                    {
                                        let owner_2 = owner + 1f64;
                                        {
                                            (expected_start, owner) = (expected_start_2, owner_2);
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
            self::MlTemporaryPlanRustNominalCopyOwnersLoop1Result::MlTemporaryPlanRustNominalCopyOwnersLoop1Done(expected_start)
        };
    }
}
