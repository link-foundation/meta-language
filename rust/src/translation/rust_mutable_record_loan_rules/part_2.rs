// Generated from complete js/src/translation/rust-mutable-record-loan-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::source_ml_source1_temporary_plan_rust_owned_record_loop_iterations_loop1;
use super::source_ml_source1_temporary_read_rust_owned_record_loop_progress_error_loop2;

/// JavaScript strings retain every UTF-16 code unit, including isolated surrogates.
pub mod ml_string {
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Text(pub Vec<u16>);

    impl std::fmt::Display for Text {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.0))
        }
    }

    impl Text {
        #[must_use]
        pub fn from_text(text: &str) -> Self {
            Self(text.encode_utf16().collect())
        }
        #[must_use]
        pub const fn from_units(units: Vec<u16>) -> Self {
            Self(units)
        }
        #[must_use]
        pub const fn length(&self) -> f64 {
            self.0.len() as f64
        }
        #[must_use]
        pub fn concat(&self, other: &Self) -> Self {
            let mut units = self.0.clone();
            units.extend_from_slice(&other.0);
            Self(units)
        }
        #[must_use]
        pub fn starts_with(&self, search: &Self) -> bool {
            self.0.starts_with(&search.0)
        }
        #[must_use]
        pub fn ends_with(&self, search: &Self) -> bool {
            self.0.ends_with(&search.0)
        }
        #[must_use]
        pub fn contains(&self, search: &Self) -> bool {
            self.index_of(search, 0.0) >= 0.0
        }
        #[must_use]
        pub fn slice(&self, start: f64, end: f64) -> Self {
            let first = relative(start, self.0.len());
            let last = relative(end, self.0.len());
            Self(self.0[first..last.max(first)].to_vec())
        }
        #[must_use]
        pub fn index_of(&self, search: &Self, position: f64) -> f64 {
            let start = absolute(position, self.0.len());
            if search.0.is_empty() {
                return start as f64;
            }
            if search.0.len() > self.0.len() {
                return -1.0;
            }
            (start..=self.0.len() - search.0.len())
                .find(|&index| self.0[index..].starts_with(&search.0))
                .map_or(-1.0, |index| index as f64)
        }
        #[must_use]
        pub fn last_index_of(&self, search: &Self, position: f64) -> f64 {
            let position = if position.is_nan() {
                f64::INFINITY
            } else {
                position
            };
            let start = absolute(position, self.0.len());
            if search.0.is_empty() {
                return start as f64;
            }
            if search.0.len() > self.0.len() {
                return -1.0;
            }
            (0..=start.min(self.0.len() - search.0.len()))
                .rev()
                .find(|&index| self.0[index..].starts_with(&search.0))
                .map_or(-1.0, |index| index as f64)
        }
        #[must_use]
        pub fn split(&self, separator: Option<&Self>, limit: f64) -> Vec<Self> {
            let maximum = uint32(limit) as usize;
            if maximum == 0 {
                return Vec::new();
            }
            let Some(separator) = separator else {
                return vec![self.clone()];
            };
            if separator.0.is_empty() {
                return self
                    .0
                    .iter()
                    .take(maximum)
                    .map(|&unit| Self(vec![unit]))
                    .collect();
            }
            let mut out = Vec::new();
            let mut start = 0;
            loop {
                let index = self.index_of(separator, start as f64);
                if index < 0.0 {
                    break;
                }
                out.push(Self(self.0[start..index as usize].to_vec()));
                if out.len() == maximum {
                    return out;
                }
                start = index as usize + separator.0.len();
            }
            out.push(Self(self.0[start..].to_vec()));
            out
        }
        #[must_use]
        pub fn code_point_at(&self, position: f64) -> Option<f64> {
            let index = integer(position);
            if index < 0.0 || index >= self.0.len() as f64 {
                return None;
            }
            let index = index as usize;
            let first = self.0[index];
            if (0xd800..=0xdbff).contains(&first)
                && let Some(&second) = self.0.get(index + 1)
                && (0xdc00..=0xdfff).contains(&second)
            {
                return Some(f64::from(
                    (u32::from(first) - 0xd800) * 1024 + u32::from(second) - 0xdc00 + 0x10000,
                ));
            }
            Some(f64::from(first))
        }
        #[must_use]
        pub fn code_points(&self) -> Vec<Self> {
            let mut out = Vec::new();
            let mut index = 0;
            while index < self.0.len() {
                let first = self.0[index];
                let width = if (0xd800..=0xdbff).contains(&first)
                    && self
                        .0
                        .get(index + 1)
                        .is_some_and(|second| (0xdc00..=0xdfff).contains(second))
                {
                    2
                } else {
                    1
                };
                out.push(Self(self.0[index..index + width].to_vec()));
                index += width;
            }
            out
        }
        #[must_use]
        pub fn repeat(&self, count: f64) -> Self {
            let count = integer(count);
            assert!(
                count >= 0.0 && count.is_finite(),
                "Invalid count value: {count}"
            );
            if self.0.is_empty() {
                return Self(Vec::new());
            }
            let size = count * self.0.len() as f64;
            assert!(size <= 536_870_888.0, "Invalid string length");
            Self(self.0.repeat(count as usize))
        }
        #[must_use]
        pub fn pad(&self, length: f64, fill: &Self, start: bool) -> Self {
            let length = integer(length).max(0.0);
            if length <= self.0.len() as f64 || fill.0.is_empty() {
                return self.clone();
            }
            assert!(length <= 536_870_888.0, "Invalid string length");
            let count = length as usize - self.0.len();
            let padding: Vec<u16> = fill.0.iter().copied().cycle().take(count).collect();
            let pad = Self(padding);
            if start {
                pad.concat(self)
            } else {
                self.concat(&pad)
            }
        }
        #[must_use]
        pub fn replace(&self, search: &Self, replacement: &Self, all: bool) -> Self {
            let mut out = Vec::new();
            let mut start = 0;
            let mut from = 0;
            loop {
                let index = self.index_of(search, from as f64);
                if index < 0.0 {
                    break;
                }
                let index = index as usize;
                out.extend_from_slice(&self.0[start..index]);
                substitute(&mut out, &self.0, &search.0, &replacement.0, index);
                start = index + search.0.len();
                if !all || (search.0.is_empty() && index == self.0.len()) {
                    break;
                }
                from = start + usize::from(search.0.is_empty());
            }
            out.extend_from_slice(&self.0[start..]);
            Self(out)
        }
        #[must_use]
        pub fn trim(&self, start: bool, end: bool, rust: bool) -> Self {
            let whitespace = |unit: u16| {
                char::from_u32(u32::from(unit)).is_some_and(|c| {
                    if rust {
                        c.is_whitespace()
                    } else {
                        (c.is_whitespace() && c != '\u{85}') || c == '\u{feff}'
                    }
                })
            };
            let mut first = 0;
            let mut last = self.0.len();
            if start {
                while first < last && whitespace(self.0[first]) {
                    first += 1;
                }
            }
            if end {
                while last > first && whitespace(self.0[last - 1]) {
                    last -= 1;
                }
            }
            Self(self.0[first..last].to_vec())
        }
        #[must_use]
        pub fn map_case(&self, upper: bool) -> Self {
            let mut out = Vec::new();
            let mut run = String::new();
            let flush = |run: &mut String, out: &mut Vec<u16>| {
                let mapped = if upper {
                    run.to_uppercase()
                } else {
                    run.to_lowercase()
                };
                out.extend(mapped.encode_utf16());
                run.clear();
            };
            for scalar in char::decode_utf16(self.0.iter().copied()) {
                match scalar {
                    Ok(c) => run.push(c),
                    Err(error) => {
                        flush(&mut run, &mut out);
                        out.push(error.unpaired_surrogate());
                    }
                }
            }
            flush(&mut run, &mut out);
            Self(out)
        }
    }
    #[must_use]
    pub fn join(values: &[Text], separator: &Text) -> Text {
        let mut units = Vec::new();
        for (index, value) in values.iter().enumerate() {
            if index != 0 {
                units.extend_from_slice(&separator.0);
            }
            units.extend_from_slice(&value.0);
        }
        Text(units)
    }
    #[must_use]
    pub const fn integer(value: f64) -> f64 {
        if value.is_nan() { 0.0 } else { value.trunc() }
    }
    #[must_use]
    pub const fn absolute(value: f64, length: usize) -> usize {
        integer(value).max(0.0).min(length as f64) as usize
    }
    #[must_use]
    pub fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 {
            (length as f64 + value).max(0.0) as usize
        } else {
            value.min(length as f64) as usize
        }
    }
    fn uint32(value: f64) -> u32 {
        if !value.is_finite() || value == 0.0 {
            0
        } else {
            value.trunc().rem_euclid(4_294_967_296.0) as u32
        }
    }
    fn substitute(
        out: &mut Vec<u16>,
        text: &[u16],
        search: &[u16],
        replacement: &[u16],
        position: usize,
    ) {
        let mut index = 0;
        while index < replacement.len() {
            if replacement[index] == 36 && index + 1 < replacement.len() {
                match replacement[index + 1] {
                    36 => {
                        out.push(36);
                        index += 2;
                        continue;
                    }
                    38 => {
                        out.extend_from_slice(search);
                        index += 2;
                        continue;
                    }
                    96 => {
                        out.extend_from_slice(&text[..position]);
                        index += 2;
                        continue;
                    }
                    39 => {
                        out.extend_from_slice(&text[position + search.len()..]);
                        index += 2;
                        continue;
                    }
                    _ => {}
                }
            }
            out.push(replacement[index]);
            index += 1;
        }
    }
}

#[path = "ml_string_number.rs"]
pub mod ml_string_number;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_loop_operation() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        114, 117, 115, 116, 79, 119, 110, 101, 100, 82, 101, 99, 111, 114, 100, 70, 105, 110, 105,
        116, 101, 76, 111, 111, 112,
    ])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_loop_chunk_size() -> f64 {
    8f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_loop_chunk_name(
    loop_: f64,
    chunk: f64,
    collision: f64,
) -> self::ml_string::Text {
    if (!(self::accept_rust_owned_record_loop_count(loop_))
        || !(self::accept_rust_owned_record_loop_count(chunk)))
        || !(self::accept_rust_owned_record_loop_count(collision))
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        ((((self::ml_string::Text::from_units(vec![
            109, 108, 95, 102, 105, 110, 105, 116, 101, 95, 114, 101, 99, 111, 114, 100, 95, 108,
            111, 111, 112, 95, 99, 104, 117, 110, 107, 95,
        ]))
        .concat(&{
            let source_ml_source2_string_argument1 = loop_;
            self::ml_string::Text::from_text(&self::ml_number::js_number(
                source_ml_source2_string_argument1,
            ))
        }))
        .concat(&self::ml_string::Text::from_units(vec![95])))
        .concat(&{
            let source_ml_source2_string_argument2 = chunk;
            self::ml_string::Text::from_text(&self::ml_number::js_number(
                source_ml_source2_string_argument2,
            ))
        }))
        .concat(&if collision == 0f64 {
            self::ml_string::Text::from_units(vec![])
        } else {
            (self::ml_string::Text::from_units(vec![95])).concat(&{
                let source_ml_source2_string_argument3 = collision;
                self::ml_string::Text::from_text(&self::ml_number::js_number(
                    source_ml_source2_string_argument3,
                ))
            })
        })
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_owned_record_loop_chunk_name(name: &self::ml_string::Text) -> bool {
    name.starts_with(&self::ml_string::Text::from_units(vec![
        109, 108, 95, 102, 105, 110, 105, 116, 101, 95, 114, 101, 99, 111, 114, 100, 95, 108, 111,
        111, 112, 95, 99, 104, 117, 110, 107, 95,
    ]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_loop_helper_error(
    generated: bool,
    recursive: bool,
    mutual: bool,
    unknown: bool,
) -> self::ml_string::Text {
    if generated {
        if recursive || mutual {
            self::ml_string::Text::from_units(vec![
                102, 105, 110, 105, 116, 101, 32, 114, 101, 99, 111, 114, 100, 32, 108, 111, 111,
                112, 115, 32, 99, 97, 110, 110, 111, 116, 32, 117, 115, 101, 32, 97, 32, 114, 101,
                99, 117, 114, 115, 105, 118, 101, 32, 111, 114, 32, 109, 117, 116, 117, 97, 108,
                108, 121, 32, 114, 101, 99, 117, 114, 115, 105, 118, 101, 32, 104, 101, 108, 112,
                101, 114, 32, 103, 114, 97, 112, 104,
            ])
        } else if unknown {
            self::ml_string::Text::from_units(vec![
                102, 105, 110, 105, 116, 101, 32, 114, 101, 99, 111, 114, 100, 32, 108, 111, 111,
                112, 115, 32, 110, 101, 101, 100, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116,
                101, 32, 115, 116, 97, 116, 105, 99, 32, 104, 101, 108, 112, 101, 114, 32, 99, 97,
                108, 108, 32, 103, 114, 97, 112, 104, 32, 119, 105, 116, 104, 111, 117, 116, 32,
                100, 121, 110, 97, 109, 105, 99, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111,
                110, 32, 111, 114, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 99, 97, 108, 108,
                115,
            ])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    } else {
        self::ml_string::Text::from_units(vec![
            97, 32, 102, 105, 110, 105, 116, 101, 32, 114, 101, 99, 111, 114, 100, 32, 108, 111,
            111, 112, 32, 99, 104, 117, 110, 107, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115,
            32, 97, 99, 116, 117, 97, 108, 32, 103, 101, 110, 101, 114, 97, 116, 101, 100, 32, 100,
            101, 99, 108, 97, 114, 97, 116, 105, 111, 110,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_loop_counter_error(
    kind: &self::ml_string::Text,
    bits: f64,
    signed: bool,
    wrappers: f64,
    owner: &self::ml_string::Text,
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((((((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
        && (bits == 8f64))
        && !signed)
        && (wrappers == 0f64))
        && ((*owner) == (self::ml_string::Text::from_units(vec![117, 56]))))
        && (((*operation) == (self::ml_string::Text::from_units(vec![110, 101])))
            || ((*operation) == (self::ml_string::Text::from_units(vec![103, 116]))))
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            97, 32, 102, 105, 110, 105, 116, 101, 32, 114, 101, 99, 111, 114, 100, 32, 108, 111,
            111, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
            108, 32, 100, 105, 114, 101, 99, 116, 32, 117, 56, 32, 99, 111, 117, 110, 116, 101,
            114, 32, 97, 110, 100, 32, 112, 117, 114, 101, 32, 110, 111, 110, 122, 101, 114, 111,
            32, 104, 101, 97, 100,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_owned_record_loop_iterations(bits: f64) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        if bits == 8f64 {
            {
                let index = 0f64;
                {
                    self::source_ml_source1_temporary_plan_rust_owned_record_loop_iterations_loop1(
                        result, index,
                    )
                }
            }
        } else {
            result
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_owned_record_loop_continuing_exit(kind: &self::ml_string::Text) -> bool {
    ((*kind) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))
        || ((*kind)
            == (self::ml_string::Text::from_units(vec![99, 111, 110, 116, 105, 110, 117, 101])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_record_loop_exit(kind: &self::ml_string::Text) -> bool {
    (((((*kind) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))
        || ((*kind)
            == (self::ml_string::Text::from_units(vec![99, 111, 110, 116, 105, 110, 117, 101]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![98, 114, 101, 97, 107]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![97, 98, 111, 114, 116])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_loop_progress_error(
    exits: &[self::ml_string::Text],
    counts: &[f64],
) -> self::ml_string::Text {
    {
        let budget_error = self::ml_string::Text::from_units(vec![
            116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 99, 111, 114,
            100, 32, 108, 111, 111, 112, 32, 101, 120, 105, 116, 115, 32, 101, 120, 99, 101, 101,
            100, 32, 116, 104, 101, 105, 114, 32, 102, 105, 110, 105, 116, 101, 32, 115, 111, 117,
            114, 99, 101, 32, 98, 117, 100, 103, 101, 116,
        ]);
        if (((exits.len() as f64) != (counts.len() as f64)) || ((exits.len() as f64) == 0f64))
            || ((exits.len() as f64) > 256f64)
        {
            budget_error.clone()
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source2_s4 = self::source_ml_source1_temporary_read_rust_owned_record_loop_progress_error_loop2(exits, counts, &(budget_error), index);
                    match source_ml_source2_s4 {
                        self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Done => {
                            self::ml_string::Text::from_units(vec![])
                        }
                        self::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Result::MlSource1TemporaryReadRustOwnedRecordLoopProgressErrorLoop2Return(source_ml_source1_temporary_result2) => {
                            source_ml_source1_temporary_result2
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_record_loop_count(value: f64) -> bool {
    (((0f64..=9_007_199_254_740_991f64).contains(&value)) && ((value % 1f64) == 0f64))
        && ((value != 0f64) || ((1f64 / value) > 0f64))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_loop_write_error(
    overlaps: bool,
    exact_decrement: bool,
) -> self::ml_string::Text {
    if overlaps && !exact_decrement {
        self::ml_string::Text::from_units(vec![
            97, 32, 114, 101, 99, 111, 114, 100, 32, 108, 111, 111, 112, 32, 99, 97, 110, 110, 111,
            116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 111, 114, 32, 97, 108, 105, 97, 115, 45,
            119, 114, 105, 116, 101, 32, 105, 116, 115, 32, 112, 114, 111, 103, 114, 101, 115, 115,
            32, 99, 111, 117, 110, 116, 101, 114,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_loop_invariant_error(
    before: &self::ml_string::Text,
    after: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*before) == (*after) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            101, 97, 99, 104, 32, 99, 111, 110, 116, 105, 110, 117, 105, 110, 103, 32, 114, 101,
            99, 111, 114, 100, 32, 108, 111, 111, 112, 32, 112, 97, 116, 104, 32, 109, 117, 115,
            116, 32, 112, 114, 101, 115, 101, 114, 118, 101, 32, 101, 118, 101, 114, 121, 32, 97,
            99, 116, 117, 97, 108, 32, 111, 117, 116, 101, 114, 32, 111, 119, 110, 101, 114, 115,
            104, 105, 112, 32, 97, 110, 100, 32, 108, 111, 97, 110, 32, 115, 116, 97, 116, 101,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_loop_signal(kind: &self::ml_string::Text) -> self::ml_string::Text {
    if (*kind) == (self::ml_string::Text::from_units(vec![98, 114, 101, 97, 107])) {
        self::ml_string::Text::from_units(vec![76, 111, 111, 112, 66, 114, 101, 97, 107])
    } else if (*kind)
        == (self::ml_string::Text::from_units(vec![99, 111, 110, 116, 105, 110, 117, 101]))
    {
        self::ml_string::Text::from_units(vec![
            76, 111, 111, 112, 67, 111, 110, 116, 105, 110, 117, 101,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_loop_leaf_kind(
    kind: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*kind) == (self::ml_string::Text::from_units(vec![98, 114, 101, 97, 107])) {
        self::ml_string::Text::from_units(vec![98, 114, 111, 107, 101, 110])
    } else if (*kind)
        == (self::ml_string::Text::from_units(vec![99, 111, 110, 116, 105, 110, 117, 101]))
    {
        self::ml_string::Text::from_units(vec![99, 111, 110, 116, 105, 110, 117, 101, 100])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_loop_exit_disposition(
    constructor: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*constructor)
        == (self::ml_string::Text::from_units(vec![
            70, 117, 110, 99, 116, 105, 111, 110, 82, 101, 115, 117, 108, 116,
        ]))
    {
        self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110])
    } else if (*constructor)
        == (self::ml_string::Text::from_units(vec![76, 111, 111, 112, 66, 114, 101, 97, 107]))
    {
        self::ml_string::Text::from_units(vec![98, 114, 101, 97, 107])
    } else if ((*constructor)
        == (self::ml_string::Text::from_units(vec![
            76, 111, 111, 112, 67, 111, 110, 116, 105, 110, 117, 101,
        ])))
        || ((*constructor)
            == (self::ml_string::Text::from_units(vec![
                83, 116, 97, 116, 101, 109, 101, 110, 116, 67, 111, 110, 116, 105, 110, 117, 97,
                116, 105, 111, 110,
            ])))
    {
        self::ml_string::Text::from_units(vec![99, 111, 110, 116, 105, 110, 117, 101])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_loop_expansion_error(
    used: f64,
    nodes: f64,
    text: f64,
    copies: f64,
) -> self::ml_string::Text {
    if ((((((!(self::accept_rust_owned_record_loop_count(used))
        || !(self::accept_rust_owned_record_loop_count(nodes)))
        || !(self::accept_rust_owned_record_loop_count(text)))
        || !(self::accept_rust_owned_record_loop_count(copies)))
        || (copies > 255f64))
        || (nodes > 100_000f64))
        || (used > (100_000f64 - (nodes * copies))))
        || (text > (16_777_216f64 / (copies + 1f64)))
    {
        self::ml_string::Text::from_units(vec![
            116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 102, 105, 110, 105, 116,
            101, 32, 114, 101, 99, 111, 114, 100, 32, 108, 111, 111, 112, 32, 101, 120, 112, 97,
            110, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32,
            115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_loop_expansion_reservation(nodes: f64, text: f64) -> Vec<f64> {
    if ((!(self::accept_rust_owned_record_loop_count(nodes))
        || !(self::accept_rust_owned_record_loop_count(text)))
        || (nodes > 100_000f64))
        || (text > 16_777_216f64)
    {
        Vec::<f64>::new()
    } else {
        vec![((nodes * 2f64) + 64f64), (text * 2f64)]
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_nested_loop_error(
    admitted: bool,
    returns: f64,
    overlap: bool,
) -> self::ml_string::Text {
    if !admitted || !(self::accept_rust_owned_record_loop_count(returns)) {
        self::ml_string::Text::from_units(vec![
            97, 32, 110, 101, 115, 116, 101, 100, 32, 108, 111, 111, 112, 32, 110, 101, 101, 100,
            115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101,
            116, 101, 32, 102, 105, 110, 105, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 97,
            100, 109, 105, 115, 115, 105, 111, 110,
        ])
    } else if returns == 0f64 {
        if overlap {
            self::ml_string::Text::from_units(vec![
                97, 32, 110, 101, 115, 116, 101, 100, 32, 108, 111, 111, 112, 32, 99, 97, 110, 110,
                111, 116, 32, 119, 114, 105, 116, 101, 32, 105, 116, 115, 32, 101, 110, 99, 108,
                111, 115, 105, 110, 103, 32, 112, 114, 111, 103, 114, 101, 115, 115, 32, 99, 111,
                117, 110, 116, 101, 114,
            ])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    } else {
        self::ml_string::Text::from_units(vec![
            97, 32, 110, 101, 115, 116, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32,
            114, 101, 116, 117, 114, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 99,
            111, 109, 112, 108, 101, 116, 101, 32, 101, 110, 99, 108, 111, 115, 105, 110, 103, 32,
            108, 111, 111, 112, 32, 101, 120, 105, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97,
            116, 105, 111, 110,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_nested_loop_depth_error(depth: f64) -> self::ml_string::Text {
    if !(self::accept_rust_owned_record_loop_count(depth)) || (depth > 8f64) {
        self::ml_string::Text::from_units(vec![
            116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 115, 116, 101, 100, 32,
            102, 105, 110, 105, 116, 101, 32, 108, 111, 111, 112, 115, 32, 101, 120, 99, 101, 101,
            100, 32, 116, 104, 101, 105, 114, 32, 115, 111, 117, 114, 99, 101, 32, 100, 101, 112,
            116, 104, 32, 98, 117, 100, 103, 101, 116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_nested_completion_error(
    admitted: bool,
    overlap: bool,
) -> self::ml_string::Text {
    if admitted {
        if overlap {
            self::ml_string::Text::from_units(vec![
                97, 32, 110, 101, 115, 116, 101, 100, 32, 108, 111, 111, 112, 32, 99, 97, 110, 110,
                111, 116, 32, 119, 114, 105, 116, 101, 32, 105, 116, 115, 32, 101, 110, 99, 108,
                111, 115, 105, 110, 103, 32, 112, 114, 111, 103, 114, 101, 115, 115, 32, 99, 111,
                117, 110, 116, 101, 114,
            ])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    } else {
        self::ml_string::Text::from_units(vec![
            97, 32, 110, 101, 115, 116, 101, 100, 32, 108, 111, 111, 112, 32, 110, 101, 101, 100,
            115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101,
            116, 101, 32, 102, 105, 110, 105, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 97,
            100, 109, 105, 115, 115, 105, 111, 110,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_loop_footprint_error(nodes: f64, text: f64) -> self::ml_string::Text {
    if ((!(self::accept_rust_owned_record_loop_count(nodes))
        || !(self::accept_rust_owned_record_loop_count(text)))
        || (nodes > 100_000f64))
        || (text > 16_777_216f64)
    {
        self::ml_string::Text::from_units(vec![
            116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 108, 111, 111, 112, 32,
            119, 114, 105, 116, 101, 32, 102, 111, 111, 116, 112, 114, 105, 110, 116, 32, 101, 120,
            99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 111, 117, 114, 99, 101, 32, 98,
            117, 100, 103, 101, 116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_loop_residual_count_error(count: f64) -> self::ml_string::Text {
    if (self::accept_rust_owned_record_loop_count(count) && (count >= 1f64)) && (count <= 256f64) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            97, 32, 112, 114, 111, 112, 97, 103, 97, 116, 101, 100, 32, 108, 111, 111, 112, 32,
            101, 120, 105, 116, 32, 110, 101, 101, 100, 115, 32, 101, 118, 101, 114, 121, 32, 97,
            99, 116, 117, 97, 108, 32, 112, 114, 111, 100, 117, 99, 101, 100, 32, 114, 101, 115,
            105, 100, 117, 97, 108, 32, 98, 114, 97, 110, 99, 104, 32, 119, 105, 116, 104, 105,
            110, 32, 105, 116, 115, 32, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101,
            116,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_update_version_error(minor: f64) -> self::ml_string::Text {
    if (self::ml_math::is_integer(minor) && (minor >= 90f64)) && (minor < 100f64) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            111, 119, 110, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 117, 112, 100, 97, 116,
            101, 32, 115, 111, 117, 114, 99, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 115,
            32, 97, 114, 101, 32, 117, 110, 99, 111, 118, 101, 114, 101, 100, 32, 111, 117, 116,
            115, 105, 100, 101, 32, 82, 117, 115, 116, 32, 62, 61, 49, 46, 57, 48, 46, 48, 32, 60,
            49, 46, 49, 48, 48, 46, 48,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_owned_record_field_copy(
    proven: bool,
    copy: bool,
    wrappers: &[self::ml_string::Text],
) -> bool {
    proven
        && ({
            let source_ml_source2_comparison_operand5 =
                self::ml_array::optional_at(wrappers, self::ml_array::number_index(0f64));
            {
                let source_ml_source2_comparison_operand6 =
                    self::ml_string::Text::from_units(vec![
                        114, 101, 102, 101, 114, 101, 110, 99, 101,
                    ]);
                match (
                    source_ml_source2_comparison_operand5,
                    source_ml_source2_comparison_operand6,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } || (((wrappers.len() as f64) == 0f64) && copy))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_field_mode(copy: bool, borrowed: bool) -> self::ml_string::Text {
    if copy {
        self::ml_string::Text::from_units(vec![
            99, 111, 112, 105, 101, 100, 32, 118, 97, 108, 117, 101,
        ])
    } else if borrowed {
        self::ml_string::Text::from_units(vec![
            98, 111, 114, 114, 111, 119, 101, 100, 32, 112, 108, 97, 99, 101,
        ])
    } else {
        self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 112, 108, 97, 99, 101])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_field_identity(
    root: &self::ml_string::Text,
    index: f64,
) -> self::ml_string::Text {
    if ((((*root) != (self::ml_string::Text::from_units(vec![])))
        && self::ml_math::is_integer(index))
        && (index >= 0f64))
        && (index < 1024f64)
    {
        (((self::ml_string::Text::from_units(vec![
            114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32,
        ]))
        .concat(root))
        .concat(&self::ml_string::Text::from_units(vec![32])))
        .concat(&{
            let source_ml_source2_string_argument7 = index;
            self::ml_string::Text::from_text(&self::ml_number::js_number(
                source_ml_source2_string_argument7,
            ))
        })
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}
