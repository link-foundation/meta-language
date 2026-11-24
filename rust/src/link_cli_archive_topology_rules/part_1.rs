// Generated from complete js/src/link-cli-archive-topology-rules.js. Do not edit by hand.
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

#[derive(Clone, Debug, PartialEq)]
pub enum LinkCliArchiveTopology {
    ArchiveDoublets(Vec<f64>),
    ArchiveDirectory(f64, f64, Vec<f64>, Vec<f64>),
    ArchiveTopologyError(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_link_cli_archive_store_name() -> String {
    String::from("meta-language complete graph store 1")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_link_cli_archive_state_name() -> String {
    String::from("meta-language complete graph store state 1")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_link_cli_archive_directory(count: f64, root: f64, state: f64) -> bool {
    ((((((self::ml_math::is_safe_integer(count) && (count >= 2f64))
        && (count <= 4_294_967_293f64))
        && self::ml_math::is_safe_integer(root))
        && (root >= 3f64))
        && (root < state))
        && self::ml_math::is_safe_integer(state))
        && (state <= (count + 2f64))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_link_cli_archive_doublets(
    ends: &[f64],
    targets: &[f64],
    root: f64,
    state: f64,
    limit: f64,
) -> self::LinkCliArchiveTopology {
    {
        let count = ends.len() as f64;
        if (self::accept_link_cli_archive_directory(count, root, state)) {
            if (!(self::ml_math::is_safe_integer(limit))
                || (((count + (targets.len() as f64)) + 2f64) > limit))
                || (((count + (targets.len() as f64)) + 2f64) > 4_294_967_295f64)
            {
                self::LinkCliArchiveTopology::ArchiveTopologyError(String::from(
                    "archive store link limit exceeded",
                ))
            } else {
                {
                    let doublets = vec![1f64, root, 2f64, state];
                    {
                        let start = 0f64;
                        {
                            let next_address = count + 3f64;
                            {
                                let index = 0f64;
                                {
                                    let source_ml_source1_s1 = self::source_ml_temporary_plan_link_cli_archive_doublets_loop1(ends, targets, count, doublets, start, next_address, index);
                                    match source_ml_source1_s1 {
                                        self::MlTemporaryPlanLinkCliArchiveDoubletsLoop1Result::MlTemporaryPlanLinkCliArchiveDoubletsLoop1Done(doublets_2, start_2, next_address_2) => {
                                            {
let ml_library_inequality_1 = start_2 != (targets.len() as f64) ;
if ml_library_inequality_1 {
                                                self::LinkCliArchiveTopology::ArchiveTopologyError(String::from("archive occurrence count mismatch"))
                                            } else {
                                                {
                                                    let start_3 = 0f64;
                                                    {
                                                        let next_address_3 = count + 3f64;
                                                        {
                                                            let index_2 = 0f64;
                                                            {
                                                                let source_ml_source1_s2 = self::source_ml_temporary_plan_link_cli_archive_doublets_loop2(ends, targets, state, count, doublets_2, start_3, next_address_3, index_2);
                                                                match source_ml_source1_s2 {
                                                                    self::MlTemporaryPlanLinkCliArchiveDoubletsLoop2Result::MlTemporaryPlanLinkCliArchiveDoubletsLoop2Done(doublets_3) => {
                                                                        self::LinkCliArchiveTopology::ArchiveDoublets(doublets_3)
                                                                    }
                                                                    self::MlTemporaryPlanLinkCliArchiveDoubletsLoop2Result::MlTemporaryPlanLinkCliArchiveDoubletsLoop2Return(source_ml_temporary_result2) => {
                                                                        let source_ml_temporary_result2 = *source_ml_temporary_result2;
                                                                        source_ml_temporary_result2.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
}
                                        }
                                        self::MlTemporaryPlanLinkCliArchiveDoubletsLoop1Result::MlTemporaryPlanLinkCliArchiveDoubletsLoop1Return(source_ml_temporary_result3) => {
                                            let source_ml_temporary_result3 = *source_ml_temporary_result3;
                                            source_ml_temporary_result3.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::LinkCliArchiveTopology::ArchiveTopologyError(String::from(
                "invalid archive record directory",
            ))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_link_cli_archive_doublets(
    doublets: &[f64],
    count: f64,
    limit: f64,
) -> self::LinkCliArchiveTopology {
    {
        let length = (doublets.len() as f64) / 2f64;
        if ((!(self::ml_math::is_safe_integer(limit)) || !(self::ml_math::is_safe_integer(length)))
            || (length > limit))
            || (length > 4_294_967_295f64)
        {
            self::LinkCliArchiveTopology::ArchiveTopologyError(String::from(
                "archive store link limit exceeded",
            ))
        } else {
            {
                let root =
                    (self::ml_array::optional_at(doublets, self::ml_array::number_index(1f64)))
                        .unwrap_or_else(|| -1f64);
                {
                    let state =
                        (self::ml_array::optional_at(doublets, self::ml_array::number_index(3f64)))
                            .unwrap_or_else(|| -1f64);
                    if (({
                        let source_ml_source1_comparison_operand3 = self::ml_array::optional_at(
                            doublets,
                            self::ml_array::number_index(0f64),
                        );
                        {
                            let source_ml_source1_comparison_operand4 = 1f64;
                            !(match (
                                source_ml_source1_comparison_operand3,
                                source_ml_source1_comparison_operand4,
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            })
                        }
                    } || {
                        let source_ml_source1_comparison_operand5 = self::ml_array::optional_at(
                            doublets,
                            self::ml_array::number_index(2f64),
                        );
                        {
                            let source_ml_source1_comparison_operand6 = 2f64;
                            !(match (
                                source_ml_source1_comparison_operand5,
                                source_ml_source1_comparison_operand6,
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            })
                        }
                    }) || (length < (count + 2f64)))
                        || !(self::accept_link_cli_archive_directory(count, root, state))
                    {
                        self::LinkCliArchiveTopology::ArchiveTopologyError(String::from(
                            "invalid archive record directory",
                        ))
                    } else {
                        {
                            let ends = Vec::<f64>::new();
                            {
                                let targets = Vec::<f64>::new();
                                {
                                    let next_address = count + 3f64;
                                    {
                                        let owner = 3f64;
                                        {
                                            let source_ml_source1_s7 = self::source_ml_temporary_read_link_cli_archive_doublets_loop4(doublets, count, length, state, ends, targets, next_address, owner);
                                            match source_ml_source1_s7 {
                                                self::MlTemporaryReadLinkCliArchiveDoubletsLoop4Result::MlTemporaryReadLinkCliArchiveDoubletsLoop4Done(ends_2, targets_2, next_address_2) => {
                                                    {
let ml_library_inequality_1 = next_address_2 != (length + 1f64) ;
if ml_library_inequality_1 {
                                                        self::LinkCliArchiveTopology::ArchiveTopologyError(String::from("archive store contains unclaimed occurrences"))
                                                    } else {
                                                        self::LinkCliArchiveTopology::ArchiveDirectory(root, state, ends_2, targets_2)
                                                    }
}
                                                }
                                                self::MlTemporaryReadLinkCliArchiveDoubletsLoop4Result::MlTemporaryReadLinkCliArchiveDoubletsLoop4Return(source_ml_temporary_result4) => {
                                                    let source_ml_temporary_result4 = *source_ml_temporary_result4;
                                                    source_ml_temporary_result4.clone()
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanLinkCliArchiveDoubletsLoop2Result {
    MlTemporaryPlanLinkCliArchiveDoubletsLoop2Done(Vec<f64>),
    MlTemporaryPlanLinkCliArchiveDoubletsLoop2Return(Box<self::LinkCliArchiveTopology>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanLinkCliArchiveDoubletsLoop3Result {
    MlTemporaryPlanLinkCliArchiveDoubletsLoop3Done(Vec<f64>, f64),
    MlTemporaryPlanLinkCliArchiveDoubletsLoop3Return(Box<self::LinkCliArchiveTopology>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_link_cli_archive_doublets_loop3(
    targets: &[f64],
    mut state: f64,
    mut count: f64,
    mut doublets: Vec<f64>,
    mut next_address: f64,
    mut end: f64,
    mut owner: f64,
    mut at: f64,
) -> self::MlTemporaryPlanLinkCliArchiveDoubletsLoop3Result {
    loop {
        return if at < end {
            {
                let target =
                    (self::ml_array::optional_at(targets, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| -1f64);
                if ((!(self::ml_math::is_safe_integer(target)) || (target < 3f64))
                    || (target > (count + 2f64)))
                    || ((owner < state) != (target < state))
                {
                    self::MlTemporaryPlanLinkCliArchiveDoubletsLoop3Result::MlTemporaryPlanLinkCliArchiveDoubletsLoop3Return(Box::new(self::LinkCliArchiveTopology::ArchiveTopologyError(String::from("archive occurrence crosses its record directory"))))
                } else {
                    {
                        let doublets_2 = {
                            let (ml_append_right, ml_append_left) = (
                                vec![
                                    target,
                                    if (at + 1f64) == end {
                                        owner
                                    } else {
                                        next_address + 1f64
                                    },
                                ],
                                doublets,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let next_address_2 = next_address + 1f64;
                            {
                                let at_2 = at + 1f64;
                                {
                                    (doublets, next_address, at) =
                                        (doublets_2, next_address_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanLinkCliArchiveDoubletsLoop3Result::MlTemporaryPlanLinkCliArchiveDoubletsLoop3Done(doublets, next_address)
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
pub fn source_ml_temporary_plan_link_cli_archive_doublets_loop2(
    ends: &[f64],
    targets: &[f64],
    mut state: f64,
    mut count: f64,
    mut doublets: Vec<f64>,
    mut start: f64,
    mut next_address: f64,
    mut index: f64,
) -> self::MlTemporaryPlanLinkCliArchiveDoubletsLoop2Result {
    loop {
        return if index < count {
            {
                let end = (self::ml_array::optional_at(ends, self::ml_array::number_index(index)))
                    .unwrap_or_else(|| -1f64);
                {
                    let owner = index + 3f64;
                    {
                        let at = start;
                        {
                            let source_ml_source1_s8 =
                                self::source_ml_temporary_plan_link_cli_archive_doublets_loop3(
                                    targets,
                                    state,
                                    count,
                                    doublets,
                                    next_address,
                                    end,
                                    owner,
                                    at,
                                );
                            match source_ml_source1_s8 {
                                self::MlTemporaryPlanLinkCliArchiveDoubletsLoop3Result::MlTemporaryPlanLinkCliArchiveDoubletsLoop3Done(doublets_2, next_address_2) => {
                                    {
                                        let start_2 = end;
                                        {
                                            let index_2 = index + 1f64;
                                            {
                                                (doublets, start, next_address, index) = (doublets_2, start_2, next_address_2, index_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryPlanLinkCliArchiveDoubletsLoop3Result::MlTemporaryPlanLinkCliArchiveDoubletsLoop3Return(source_ml_temporary_result3) => {
                                    let source_ml_temporary_result3 = *source_ml_temporary_result3;
                                    self::MlTemporaryPlanLinkCliArchiveDoubletsLoop2Result::MlTemporaryPlanLinkCliArchiveDoubletsLoop2Return(Box::new(source_ml_temporary_result3.clone()))
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanLinkCliArchiveDoubletsLoop2Result::MlTemporaryPlanLinkCliArchiveDoubletsLoop2Done(doublets)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanLinkCliArchiveDoubletsLoop1Result {
    MlTemporaryPlanLinkCliArchiveDoubletsLoop1Done(Vec<f64>, f64, f64),
    MlTemporaryPlanLinkCliArchiveDoubletsLoop1Return(Box<self::LinkCliArchiveTopology>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_link_cli_archive_doublets_loop1(
    ends: &[f64],
    targets: &[f64],
    mut count: f64,
    mut doublets: Vec<f64>,
    mut start: f64,
    mut next_address: f64,
    mut index: f64,
) -> self::MlTemporaryPlanLinkCliArchiveDoubletsLoop1Result {
    loop {
        return if index < count {
            {
                let end = (self::ml_array::optional_at(ends, self::ml_array::number_index(index)))
                    .unwrap_or_else(|| -1f64);
                {
                    let owner = index + 3f64;
                    if (!(self::ml_math::is_safe_integer(end)) || (end < start))
                        || (end > (targets.len() as f64))
                    {
                        self::MlTemporaryPlanLinkCliArchiveDoubletsLoop1Result::MlTemporaryPlanLinkCliArchiveDoubletsLoop1Return(Box::new(self::LinkCliArchiveTopology::ArchiveTopologyError(String::from("invalid cumulative occurrence count"))))
                    } else {
                        {
                            let doublets_2 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![if start < end { next_address } else { owner }, owner],
                                    doublets,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let next_address_2 = next_address + (end - start);
                                {
                                    let start_2 = end;
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            (doublets, start, next_address, index) =
                                                (doublets_2, start_2, next_address_2, index_2);
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
            self::MlTemporaryPlanLinkCliArchiveDoubletsLoop1Result::MlTemporaryPlanLinkCliArchiveDoubletsLoop1Done(doublets, start, next_address)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadLinkCliArchiveDoubletsLoop4Result {
    MlTemporaryReadLinkCliArchiveDoubletsLoop4Done(Vec<f64>, Vec<f64>, f64),
    MlTemporaryReadLinkCliArchiveDoubletsLoop4Return(Box<self::LinkCliArchiveTopology>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadLinkCliArchiveDoubletsLoop5Result {
    MlTemporaryReadLinkCliArchiveDoubletsLoop5Done(Vec<f64>, f64),
    MlTemporaryReadLinkCliArchiveDoubletsLoop5Return(Box<self::LinkCliArchiveTopology>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_read_link_cli_archive_doublets_loop5(
    doublets: &[f64],
    mut count: f64,
    mut length: f64,
    mut state: f64,
    mut targets: Vec<f64>,
    mut next_address: f64,
    mut owner: f64,
    mut address: f64,
) -> self::MlTemporaryReadLinkCliArchiveDoubletsLoop5Result {
    loop {
        return if address == owner {
            self::MlTemporaryReadLinkCliArchiveDoubletsLoop5Result::MlTemporaryReadLinkCliArchiveDoubletsLoop5Done(targets, next_address)
        } else {
            if (address != next_address) || (address > length) {
                self::MlTemporaryReadLinkCliArchiveDoubletsLoop5Result::MlTemporaryReadLinkCliArchiveDoubletsLoop5Return(Box::new(self::LinkCliArchiveTopology::ArchiveTopologyError(String::from("noncanonical archive occurrence chain"))))
            } else {
                {
                    let position = (address - 1f64) * 2f64;
                    {
                        let target = (self::ml_array::optional_at(
                            doublets,
                            self::ml_array::number_index(position),
                        ))
                        .unwrap_or_else(|| -1f64);
                        if ((!(self::ml_math::is_safe_integer(target)) || (target < 3f64))
                            || (target > (count + 2f64)))
                            || ((owner < state) != (target < state))
                        {
                            self::MlTemporaryReadLinkCliArchiveDoubletsLoop5Result::MlTemporaryReadLinkCliArchiveDoubletsLoop5Return(Box::new(self::LinkCliArchiveTopology::ArchiveTopologyError(String::from("archive occurrence crosses its record directory"))))
                        } else {
                            {
                                let targets_2 = {
                                    let (ml_append_right, ml_append_left) = (vec![target], targets);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let address_2 = (self::ml_array::optional_at(
                                        doublets,
                                        self::ml_array::number_index(position + 1f64),
                                    ))
                                    .unwrap_or_else(|| -1f64);
                                    {
                                        let next_address_2 = next_address + 1f64;
                                        {
                                            (targets, next_address, address) =
                                                (targets_2, next_address_2, address_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
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
pub fn source_ml_temporary_read_link_cli_archive_doublets_loop4(
    doublets: &[f64],
    mut count: f64,
    mut length: f64,
    mut state: f64,
    mut ends: Vec<f64>,
    mut targets: Vec<f64>,
    mut next_address: f64,
    mut owner: f64,
) -> self::MlTemporaryReadLinkCliArchiveDoubletsLoop4Result {
    loop {
        return if owner <= (count + 2f64) {
            {
                let offset = (owner - 1f64) * 2f64;
                {
                    let address = (self::ml_array::optional_at(
                        doublets,
                        self::ml_array::number_index(offset),
                    ))
                    .unwrap_or_else(|| -1f64);
                    if {
                        let source_ml_source1_comparison_operand9 = self::ml_array::optional_at(
                            doublets,
                            self::ml_array::number_index(offset + 1f64),
                        );
                        {
                            let source_ml_source1_comparison_operand10 = owner;
                            !(match (
                                source_ml_source1_comparison_operand9,
                                source_ml_source1_comparison_operand10,
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            })
                        }
                    } {
                        self::MlTemporaryReadLinkCliArchiveDoubletsLoop4Result::MlTemporaryReadLinkCliArchiveDoubletsLoop4Return(Box::new(self::LinkCliArchiveTopology::ArchiveTopologyError(String::from("invalid archive record owner"))))
                    } else {
                        {
                            let source_ml_source1_s11 =
                                self::source_ml_temporary_read_link_cli_archive_doublets_loop5(
                                    doublets,
                                    count,
                                    length,
                                    state,
                                    targets,
                                    next_address,
                                    owner,
                                    address,
                                );
                            match source_ml_source1_s11 {
                                self::MlTemporaryReadLinkCliArchiveDoubletsLoop5Result::MlTemporaryReadLinkCliArchiveDoubletsLoop5Done(targets_2, next_address_2) => {
                                    {
                                        let ends_2 = { let (ml_append_right, ml_append_left) = (vec![(targets_2.len() as f64)], ends); self::ml_array::append(ml_append_left, ml_append_right) };
                                        {
                                            let owner_2 = owner + 1f64;
                                            {
                                                (ends, targets, next_address, owner) = (ends_2, targets_2, next_address_2, owner_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryReadLinkCliArchiveDoubletsLoop5Result::MlTemporaryReadLinkCliArchiveDoubletsLoop5Return(source_ml_temporary_result5) => {
                                    let source_ml_temporary_result5 = *source_ml_temporary_result5;
                                    self::MlTemporaryReadLinkCliArchiveDoubletsLoop4Result::MlTemporaryReadLinkCliArchiveDoubletsLoop4Return(Box::new(source_ml_temporary_result5.clone()))
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadLinkCliArchiveDoubletsLoop4Result::MlTemporaryReadLinkCliArchiveDoubletsLoop4Done(ends, targets, next_address)
        };
    }
}
