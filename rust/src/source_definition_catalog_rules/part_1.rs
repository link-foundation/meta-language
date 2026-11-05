// Generated from complete js/src/source-definition-catalog-rules.js. Do not edit by hand.
use super::MlTemporaryAcceptSourceDefinitionCatalogRootLoop3Result;
use super::MlTemporarySelectSourceDefinitionCatalogEntryLoop6Result;
use super::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Result;
use super::source_ml_temporary_accept_source_definition_catalog_digest_loop2;
use super::source_ml_temporary_accept_source_definition_catalog_root_loop3;
use super::source_ml_temporary_select_source_definition_catalog_entry_loop6;
use super::source_ml_temporary_validate_source_definition_catalog_entries_loop4;

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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_definition_catalog_header(
    version: f64,
    model: &self::ml_string::Text,
    count: f64,
) -> bool {
    ((((version == 1f64)
        && ((*model)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 109, 112, 108, 101, 116, 101, 32, 113, 117, 97, 108, 105, 102, 105, 101,
                100, 32, 116, 121, 112, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 112, 114,
                111, 103, 114, 97, 109, 115, 32, 49,
            ]))))
        && (count >= 1f64))
        && (count <= 4096f64))
        && ((count % 1f64) == 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_definition_catalog_asset(text: &self::ml_string::Text) -> bool {
    if ((text.length()) < 14f64)
        || (({
            let source_ml_source1_string_range_operand5 = text.clone();
            {
                let source_ml_source1_string_range_operand6 = text.length() - 13f64;
                {
                    let source_ml_source1_string_range_position7 =
                        source_ml_source1_string_range_operand6;
                    {
                        let source_ml_source1_string_range_position8 = f64::INFINITY;
                        source_ml_source1_string_range_operand5.slice(
                            source_ml_source1_string_range_position7,
                            source_ml_source1_string_range_position8,
                        )
                    }
                }
            }
        }) != (self::ml_string::Text::from_units(vec![
            45, 112, 114, 111, 103, 114, 97, 109, 46, 108, 105, 110, 111,
        ])))
    {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s9 =
                    self::source_ml_temporary_accept_source_definition_catalog_asset_loop1(
                        text, index,
                    );
                match source_ml_source1_s9 {
                    self::MlTemporaryAcceptSourceDefinitionCatalogAssetLoop1Result::MlTemporaryAcceptSourceDefinitionCatalogAssetLoop1Done => {
                        true
                    }
                    self::MlTemporaryAcceptSourceDefinitionCatalogAssetLoop1Result::MlTemporaryAcceptSourceDefinitionCatalogAssetLoop1Return(source_ml_temporary_result1) => {
                        source_ml_temporary_result1
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_definition_catalog_digest(text: &self::ml_string::Text) -> bool {
    {
        let ml_library_inequality_1 = (text.length()) != 64f64;
        if ml_library_inequality_1 {
            false
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source1_s10 =
                        self::source_ml_temporary_accept_source_definition_catalog_digest_loop2(
                            text, index,
                        );
                    match source_ml_source1_s10 {
                    self::MlTemporaryAcceptSourceDefinitionCatalogDigestLoop2Result::MlTemporaryAcceptSourceDefinitionCatalogDigestLoop2Done => {
                        true
                    }
                    self::MlTemporaryAcceptSourceDefinitionCatalogDigestLoop2Result::MlTemporaryAcceptSourceDefinitionCatalogDigestLoop2Return(source_ml_temporary_result2) => {
                        source_ml_temporary_result2
                    }
                }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_definition_catalog_root(text: &self::ml_string::Text) -> bool {
    if (((text.length()) < 1f64) || ((text.length()) > 16f64))
        || (({
            let source_ml_source1_string_unit_operand20 = text.clone();
            {
                let source_ml_source1_string_unit_operand21 = 0f64;
                (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> f64 {
                    {
                        let index = if position.is_nan() {
                            0f64
                        } else {
                            position.trunc()
                        };
                        if (index < 0f64) || (index >= (text_2.length())) {
                            f64::NAN
                        } else {
                            ({
                                let source_ml_source1_string_unit_operand27 = {
                                    let source_ml_source1_string_range_operand22 = text_2.clone();
                                    {
                                        let source_ml_source1_string_range_operand23 = index;
                                        {
                                            let source_ml_source1_string_range_operand24 =
                                                index + 1f64;
                                            {
                                                let source_ml_source1_string_range_position25 =
                                                    source_ml_source1_string_range_operand23;
                                                {
                                                    let source_ml_source1_string_range_position26 =
                                                        source_ml_source1_string_range_operand24;
                                                    source_ml_source1_string_range_operand22.slice(
                                                        source_ml_source1_string_range_position25,
                                                        source_ml_source1_string_range_position26,
                                                    )
                                                }
                                            }
                                        }
                                    }
                                };
                                {
                                    let source_ml_source1_string_unit_operand28 = 0f64;
                                    source_ml_source1_string_unit_operand27
                                        .code_point_at(source_ml_source1_string_unit_operand28)
                                }
                            })
                            .clone()
                            .unwrap_or_else(|| f64::NAN)
                        }
                    }
                }))(
                    source_ml_source1_string_unit_operand20,
                    source_ml_source1_string_unit_operand21,
                )
            }
        }) == 48f64)
    {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s29 =
                    self::source_ml_temporary_accept_source_definition_catalog_root_loop3(
                        text, index,
                    );
                match source_ml_source1_s29 {
                    self::MlTemporaryAcceptSourceDefinitionCatalogRootLoop3Result::MlTemporaryAcceptSourceDefinitionCatalogRootLoop3Done => {
                        ((text.length()) < 16f64) || <self::ml_string::Text as std::cmp::PartialOrd>::le(text, &self::ml_string::Text::from_units(vec![57, 48, 48, 55, 49, 57, 57, 50, 53, 52, 55, 52, 48, 57, 57, 49]))
                    }
                    self::MlTemporaryAcceptSourceDefinitionCatalogRootLoop3Result::MlTemporaryAcceptSourceDefinitionCatalogRootLoop3Return(source_ml_temporary_result3) => {
                        source_ml_temporary_result3
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn validate_source_definition_catalog_entries(
    identifiers: &[self::ml_string::Text],
    assets: &[self::ml_string::Text],
    digests: &[self::ml_string::Text],
    bytes: &[f64],
    roots: &[self::ml_string::Text],
) -> f64 {
    if ((((identifiers.len() as f64) != (assets.len() as f64))
        || ((identifiers.len() as f64) != (digests.len() as f64)))
        || ((identifiers.len() as f64) != (bytes.len() as f64)))
        || ((identifiers.len() as f64) != (roots.len() as f64))
    {
        -1f64
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s30 =
                    self::source_ml_temporary_validate_source_definition_catalog_entries_loop4(
                        identifiers,
                        assets,
                        digests,
                        bytes,
                        roots,
                        index,
                    );
                match source_ml_source1_s30 {
                    self::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Result::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Done => {
                        1f64
                    }
                    self::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Result::MlTemporaryValidateSourceDefinitionCatalogEntriesLoop4Return(source_ml_temporary_result4) => {
                        source_ml_temporary_result4
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn select_source_definition_catalog_entry(
    identifiers: &[self::ml_string::Text],
    identifier: &self::ml_string::Text,
) -> f64 {
    {
        let index = 0f64;
        {
            let source_ml_source1_s31 =
                self::source_ml_temporary_select_source_definition_catalog_entry_loop6(
                    identifiers,
                    identifier,
                    index,
                );
            match source_ml_source1_s31 {
                self::MlTemporarySelectSourceDefinitionCatalogEntryLoop6Result::MlTemporarySelectSourceDefinitionCatalogEntryLoop6Done => {
                    -1f64
                }
                self::MlTemporarySelectSourceDefinitionCatalogEntryLoop6Result::MlTemporarySelectSourceDefinitionCatalogEntryLoop6Return(source_ml_temporary_result6) => {
                    source_ml_temporary_result6
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_definition_catalog_limits(
    byte_limit: f64,
    node_limit: f64,
    depth_limit: f64,
) -> bool {
    ((((((((1f64..=134_217_728f64).contains(&byte_limit)) && ((byte_limit % 1f64) == 0f64))
        && (node_limit >= 1f64))
        && (node_limit <= 1_000_000f64))
        && ((node_limit % 1f64) == 0f64))
        && (depth_limit >= 1f64))
        && (depth_limit <= 1024f64))
        && ((depth_limit % 1f64) == 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_definition_catalog_identity(
    identifier: &self::ml_string::Text,
    digest: &self::ml_string::Text,
) -> self::ml_string::Text {
    (identifier.concat(&self::ml_string::Text::from_units(vec![
        64, 115, 104, 97, 50, 53, 54, 58,
    ])))
    .concat(digest)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_definition_command_arity(command: &self::ml_string::Text) -> f64 {
    if (*command) == (self::ml_string::Text::from_units(vec![108, 105, 115, 116])) {
        0f64
    } else if (*command)
        == (self::ml_string::Text::from_units(vec![112, 114, 111, 103, 114, 97, 109]))
    {
        1f64
    } else if (*command)
        == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))
    {
        2f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_definition_command_arguments(
    command: &self::ml_string::Text,
    count: f64,
    binary: bool,
) -> bool {
    (((self::read_source_definition_command_arity(command)) >= 0f64)
        && ((self::read_source_definition_command_arity(command)) == count))
        && !(((*command) == (self::ml_string::Text::from_units(vec![108, 105, 115, 116])))
            && binary)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_definition_command_usage() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        117, 115, 97, 103, 101, 58, 32, 109, 101, 116, 97, 45, 108, 97, 110, 103, 117, 97, 103,
        101, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 108, 105, 115, 116, 10, 32,
        32, 32, 32, 32, 32, 32, 109, 101, 116, 97, 45, 108, 97, 110, 103, 117, 97, 103, 101, 32,
        100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 112, 114, 111, 103, 114, 97, 109, 32,
        60, 113, 117, 97, 108, 105, 102, 105, 101, 100, 45, 105, 100, 101, 110, 116, 105, 102, 105,
        101, 114, 62, 32, 91, 45, 45, 98, 105, 110, 97, 114, 121, 93, 10, 32, 32, 32, 32, 32, 32,
        32, 109, 101, 116, 97, 45, 108, 97, 110, 103, 117, 97, 103, 101, 32, 100, 101, 102, 105,
        110, 105, 116, 105, 111, 110, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 60, 113, 117,
        97, 108, 105, 102, 105, 101, 100, 45, 105, 100, 101, 110, 116, 105, 102, 105, 101, 114, 62,
        32, 60, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 62, 32, 91, 45, 45, 98, 105,
        110, 97, 114, 121, 93, 10,
    ])
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptSourceDefinitionCatalogAssetLoop1Result {
    MlTemporaryAcceptSourceDefinitionCatalogAssetLoop1Done,
    MlTemporaryAcceptSourceDefinitionCatalogAssetLoop1Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_source_definition_catalog_asset_loop1(
    text: &self::ml_string::Text,
    mut index: f64,
) -> self::MlTemporaryAcceptSourceDefinitionCatalogAssetLoop1Result {
    loop {
        return if index < (text.length() - 13f64) {
            {
                let code = {
                    let source_ml_source1_string_unit_operand32 = text.clone();
                    {
                        let source_ml_source1_string_unit_operand33 = index;
                        (std::rc::Rc::new(
                            move |text_2: self::ml_string::Text, position: f64| -> f64 {
                                {
                                    let index_2 = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index_2 < 0f64) || (index_2 >= (text_2.length())) {
                                        f64::NAN
                                    } else {
                                        ({
                                    let source_ml_source1_string_unit_operand39 = {
                                        let source_ml_source1_string_range_operand34 = text_2.clone();
                                        {
                                            let source_ml_source1_string_range_operand35 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand36 = index_2 + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position37 = source_ml_source1_string_range_operand35;
                                                    {
                                                        let source_ml_source1_string_range_position38 = source_ml_source1_string_range_operand36;
                                                        source_ml_source1_string_range_operand34.slice(source_ml_source1_string_range_position37, source_ml_source1_string_range_position38)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand40 = 0f64;
                                        source_ml_source1_string_unit_operand39.code_point_at(source_ml_source1_string_unit_operand40)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source1_string_unit_operand32,
                            source_ml_source1_string_unit_operand33,
                        )
                    }
                };
                if ((((97f64..=122f64).contains(&code))
                    || (((index > 0f64) && (code >= 48f64)) && (code <= 57f64)))
                    || ((index > 0f64) && (code == 45f64)))
                {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
                {
                    self::MlTemporaryAcceptSourceDefinitionCatalogAssetLoop1Result::MlTemporaryAcceptSourceDefinitionCatalogAssetLoop1Return(false)
                }
            }
        } else {
            self::MlTemporaryAcceptSourceDefinitionCatalogAssetLoop1Result::MlTemporaryAcceptSourceDefinitionCatalogAssetLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptSourceDefinitionCatalogDigestLoop2Result {
    MlTemporaryAcceptSourceDefinitionCatalogDigestLoop2Done,
    MlTemporaryAcceptSourceDefinitionCatalogDigestLoop2Return(bool),
}
