// Generated from complete js/src/translation/rust-owned-record-update-rules.js. Do not edit by hand.
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;

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
            let source_ml_source1_comparison_operand1 =
                self::ml_array::optional_at(wrappers, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand2 =
                    self::ml_string::Text::from_units(vec![
                        114, 101, 102, 101, 114, 101, 110, 99, 101,
                    ]);
                match (
                    source_ml_source1_comparison_operand1,
                    source_ml_source1_comparison_operand2,
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
            let source_ml_source1_string_argument3 = index;
            self::ml_string::Text::from_text(&self::ml_number::js_number(
                source_ml_source1_string_argument3,
            ))
        })
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_field_root(
    identity: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (identity.starts_with(&self::ml_string::Text::from_units(vec![
        114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32,
    ]))) {
        {
            let boundary = {
                let source_ml_source1_string_search_operand4 = identity.clone();
                {
                    let source_ml_source1_string_search_operand5 =
                        self::ml_string::Text::from_units(vec![32]);
                    {
                        let source_ml_source1_string_search_text6 =
                            source_ml_source1_string_search_operand5;
                        {
                            let source_ml_source1_string_search_position7 = f64::NAN;
                            source_ml_source1_string_search_operand4.last_index_of(
                                &source_ml_source1_string_search_text6,
                                source_ml_source1_string_search_position7,
                            )
                        }
                    }
                }
            };
            {
                let root = {
                    let source_ml_source1_string_range_operand8 = identity.clone();
                    {
                        let source_ml_source1_string_range_operand9 = 13f64;
                        {
                            let source_ml_source1_string_range_operand10 = boundary;
                            {
                                let source_ml_source1_string_range_position11 =
                                    source_ml_source1_string_range_operand9;
                                {
                                    let source_ml_source1_string_range_position12 =
                                        source_ml_source1_string_range_operand10;
                                    source_ml_source1_string_range_operand8.slice(
                                        source_ml_source1_string_range_position11,
                                        source_ml_source1_string_range_position12,
                                    )
                                }
                            }
                        }
                    }
                };
                {
                    let index = self::ml_string_number::convert_java_script_string_number_units(
                        &({
                            let source_ml_source1_string_range_operand13 = identity.clone();
                            {
                                let source_ml_source1_string_range_operand14 = boundary + 1f64;
                                {
                                    let source_ml_source1_string_range_position15 =
                                        source_ml_source1_string_range_operand14;
                                    {
                                        let source_ml_source1_string_range_position16 =
                                            f64::INFINITY;
                                        source_ml_source1_string_range_operand13.slice(
                                            source_ml_source1_string_range_position15,
                                            source_ml_source1_string_range_position16,
                                        )
                                    }
                                }
                            }
                        })
                        .0
                        .iter()
                        .copied()
                        .map(f64::from)
                        .collect::<Vec<f64>>(),
                    );
                    if (self::read_rust_owned_record_field_identity(&(root), index)) == (*identity)
                    {
                        root.clone()
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            }
        }
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_absent_field_state(
    root: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*root) == (self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101]))
    {
        self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101])
    } else {
        self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 116])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_ultimate_root(
    identity: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let root = self::read_rust_owned_record_field_root(identity);
        if root == (self::ml_string::Text::from_units(vec![])) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let depth = 0f64;
                {
                    let source_ml_source1_s17 =
                        self::source_ml_temporary_read_rust_owned_record_ultimate_root_loop1(
                            root, depth,
                        );
                    match source_ml_source1_s17 {
                        self::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Done => {
                            self::ml_string::Text::from_units(vec![])
                        }
                        self::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Return(source_ml_temporary_result1) => {
                            source_ml_temporary_result1
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_owned_record_field_descendant(
    root: &self::ml_string::Text,
    identity: &self::ml_string::Text,
) -> bool {
    {
        let parent = self::read_rust_owned_record_field_root(identity);
        {
            let depth = 0f64;
            {
                let source_ml_source1_s18 =
                    self::source_ml_temporary_test_rust_owned_record_field_descendant_loop2(
                        root, parent, depth,
                    );
                match source_ml_source1_s18 {
                    self::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Done => {
                        false
                    }
                    self::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Return(source_ml_temporary_result2) => {
                        source_ml_temporary_result2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_whole_error(
    root: &self::ml_string::Text,
    identities: &[self::ml_string::Text],
    states: &[self::ml_string::Text],
) -> self::ml_string::Text {
    {
        let ml_library_inequality_1 = (identities.len() as f64) != (states.len() as f64);
        if ml_library_inequality_1 {
            self::ml_string::Text::from_units(vec![
                114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 97, 118, 97, 105,
                108, 97, 98, 105, 108, 105, 116, 121, 32, 110, 101, 101, 100, 115, 32, 101, 118,
                101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32,
                115, 116, 97, 116, 101,
            ])
        } else if (*root) == (self::ml_string::Text::from_units(vec![])) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_temporary_values3 = identities.to_vec();
                    {
                        let source_ml_temporary_index3 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source1_s19 =
                                self::source_ml_temporary_read_rust_owned_record_whole_error_loop4(
                                    root,
                                    states,
                                    index,
                                    &(source_ml_temporary_values3),
                                    source_ml_temporary_index3,
                                );
                            match source_ml_source1_s19 {
                                self::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Done => {
                                    self::ml_string::Text::from_units(vec![])
                                }
                                self::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Return(source_ml_temporary_result4) => {
                                    source_ml_temporary_result4
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
pub fn read_rust_owned_record_field_error(
    root: &self::ml_string::Text,
    field: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*root)
        == (self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101])))
        && ((*field)
            == (self::ml_string::Text::from_units(vec![97, 118, 97, 105, 108, 97, 98, 108, 101])))
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            97, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 114, 101, 113,
            117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 118,
            97, 105, 108, 97, 98, 108, 101, 32, 114, 111, 111, 116, 32, 97, 110, 100, 32, 117, 110,
            109, 111, 118, 101, 100, 32, 102, 105, 101, 108, 100,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_update_error(
    profile: &self::ml_string::Text,
    owner: bool,
    layers: f64,
    fields: f64,
    positions: f64,
) -> self::ml_string::Text {
    if ((((((((*profile)
        == (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100])))
        || ((*profile) == (self::ml_string::Text::from_units(vec![99, 111, 112, 121]))))
        && owner)
        && (layers == 0f64))
        && self::ml_math::is_integer(fields))
        && (fields >= 0f64))
        && (fields <= 1024f64))
        && ((positions).partial_cmp(&(fields)) == Some(std::cmp::Ordering::Equal))
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            97, 110, 32, 111, 119, 110, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 117, 112,
            100, 97, 116, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116,
            117, 97, 108, 32, 100, 101, 115, 116, 114, 117, 99, 116, 111, 114, 45, 102, 114, 101,
            101, 32, 115, 116, 114, 117, 99, 116, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108,
            101, 116, 101, 32, 102, 105, 101, 108, 100, 32, 108, 97, 121, 111, 117, 116,
        ])
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result {
    MlTemporaryReadRustOwnedRecordUltimateRootLoop1Done,
    MlTemporaryReadRustOwnedRecordUltimateRootLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_owned_record_ultimate_root_loop1(
    mut root: self::ml_string::Text,
    mut depth: f64,
) -> self::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result {
    loop {
        return if depth < 1024f64 {
            {
                let parent = self::read_rust_owned_record_field_root(&(root));
                if parent == (self::ml_string::Text::from_units(vec![])) {
                    self::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Return(root.clone())
                } else {
                    {
                        let root_2 = parent;
                        {
                            let depth_2 = depth + 1f64;
                            {
                                (root, depth) = (root_2, depth_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Result::MlTemporaryReadRustOwnedRecordUltimateRootLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result {
    MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Done,
    MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_rust_owned_record_field_descendant_loop2(
    root: &self::ml_string::Text,
    mut parent: self::ml_string::Text,
    mut depth: f64,
) -> self::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result {
    loop {
        return if (depth < 1024f64) && (parent != (self::ml_string::Text::from_units(vec![]))) {
            if parent == (*root) {
                self::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Return(true)
            } else {
                {
                    let parent_2 = self::read_rust_owned_record_field_root(&(parent));
                    {
                        let depth_2 = depth + 1f64;
                        {
                            (parent, depth) = (parent_2, depth_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Result::MlTemporaryTestRustOwnedRecordFieldDescendantLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result {
    MlTemporaryReadRustOwnedRecordWholeErrorLoop4Done,
    MlTemporaryReadRustOwnedRecordWholeErrorLoop4Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_owned_record_whole_error_loop4(
    root: &self::ml_string::Text,
    states: &[self::ml_string::Text],
    mut index: f64,
    source_ml_temporary_values3: &[self::ml_string::Text],
    mut source_ml_temporary_index3: self::ml::Big,
) -> self::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result {
    loop {
        return if source_ml_temporary_index3
            < (self::ml::Big::from_u128(source_ml_temporary_values3.len() as u128))
        {
            {
                let identity = self::ml_array::at(
                    source_ml_temporary_values3,
                    source_ml_temporary_index3.to_index(),
                );
                if self::test_rust_owned_record_field_descendant(root, &(identity)) && {
                    let source_ml_source1_comparison_operand20 =
                        self::ml_array::optional_at(states, self::ml_array::number_index(index));
                    {
                        let source_ml_source1_comparison_operand21 =
                            self::ml_string::Text::from_units(vec![
                                97, 118, 97, 105, 108, 97, 98, 108, 101,
                            ]);
                        !(match (
                            source_ml_source1_comparison_operand20,
                            source_ml_source1_comparison_operand21,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }
                } {
                    self::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Return(self::ml_string::Text::from_units(vec![97, 32, 112, 97, 114, 116, 105, 97, 108, 108, 121, 32, 109, 111, 118, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 117, 115, 101, 100, 32, 97, 115, 32, 97, 32, 119, 104, 111, 108, 101, 32, 118, 97, 108, 117, 101]))
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            let source_ml_temporary_index3_2 =
                                source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                            {
                                (index, source_ml_temporary_index3) =
                                    (index_2, source_ml_temporary_index3_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Result::MlTemporaryReadRustOwnedRecordWholeErrorLoop4Done
        };
    }
}
