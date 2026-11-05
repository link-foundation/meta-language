// Generated from complete js/src/translation/source-version-column-rules.js. Do not edit by hand.
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

#[path = "ml_utf16_text.rs"]
pub mod ml_utf16_text;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_version_columns(text: &self::ml_string::Text) -> Vec<f64> {
    if (text.length()) > 32f64 {
        Vec::<f64>::new()
    } else {
        {
            let parts = {
                let source_ml_source1_string_unit_operand1 = text.clone();
                {
                    let source_ml_source1_string_unit_operand2 =
                        self::ml_string::Text::from_units(vec![46]);
                    {
                        let source_ml_source1_string_split_limit3 = 4_294_967_295f64;
                        {
                            let source_ml_source1_string_split_separator4 =
                                source_ml_source1_string_unit_operand2;
                            (|ml_text: &self::ml_string::Text,
                              ml_separator: Option<&self::ml_string::Text>,
                              ml_limit: f64| {
                                self::ml_utf16_text::split_utf16_text(
                                    &(ml_text
                                        .0
                                        .iter()
                                        .map(|ml_unit| f64::from(*ml_unit))
                                        .collect::<Vec<f64>>()),
                                    &(ml_separator
                                        .map(|ml_value| {
                                            ml_value
                                                .0
                                                .iter()
                                                .map(|ml_unit| f64::from(*ml_unit))
                                                .collect::<Vec<f64>>()
                                        })
                                        .unwrap_or_default()),
                                    ml_limit,
                                    ml_separator.is_none(),
                                )
                                .into_iter()
                                .map(|ml_units| {
                                    self::ml_string::Text::from_units(
                                        ml_units
                                            .into_iter()
                                            .map(|ml_unit| ml_unit as u16)
                                            .collect(),
                                    )
                                })
                                .collect::<Vec<_>>()
                            })(
                                &(source_ml_source1_string_unit_operand1),
                                Some(&source_ml_source1_string_split_separator4),
                                source_ml_source1_string_split_limit3,
                            )
                        }
                    }
                }
            };
            {
                let ml_library_inequality_1 = (parts.len() as f64) != 3f64;
                if ml_library_inequality_1 {
                    Vec::<f64>::new()
                } else {
                    {
                        let columns = {
                            let ml_values1 = parts;
                            let ml_callback1 = std::rc::Rc::new(
                                move |part: self::ml_string::Text| -> f64 {
                                    if ((part.length()) == 0f64)
                                        || (((part.length()) > 1f64)
                                            && (({
                                                let source_ml_source1_string_unit_operand14 =
                                                    part.clone();
                                                {
                                                    let source_ml_source1_string_unit_operand15 =
                                                        0f64;
                                                    (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> f64 { {
                                let index = if position.is_nan() {
                                    0f64
                                } else {
                                    position.trunc()
                                };
                                if (index < 0f64) || (index >= (text_2.length())) {
                                    f64::NAN
                                } else {
                                    ({
                                        let source_ml_source1_string_unit_operand21 = {
                                            let source_ml_source1_string_range_operand16 = text_2.clone();
                                            {
                                                let source_ml_source1_string_range_operand17 = index;
                                                {
                                                    let source_ml_source1_string_range_operand18 = index + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position19 = source_ml_source1_string_range_operand17;
                                                        {
                                                            let source_ml_source1_string_range_position20 = source_ml_source1_string_range_operand18;
                                                            source_ml_source1_string_range_operand16.slice(source_ml_source1_string_range_position19, source_ml_source1_string_range_position20)
                                                        }
                                                    }
                                                }
                                            }
                                        };
                                        {
                                            let source_ml_source1_string_unit_operand22 = 0f64;
                                            source_ml_source1_string_unit_operand21.code_point_at(source_ml_source1_string_unit_operand22)
                                        }
                                    }).clone().unwrap_or_else(|| f64::NAN)
                                }
                            } }))(source_ml_source1_string_unit_operand14, source_ml_source1_string_unit_operand15)
                                                }
                                            }) == 48f64))
                                    {
                                        -1f64
                                    } else {
                                        {
                                            let value = 0f64;
                                            {
                                                let index = 0f64;
                                                {
                                                    let source_ml_source1_s23 =
                                                        self::source_ml_temporary_closure1_loop2(
                                                            &(part),
                                                            value,
                                                            index,
                                                        );
                                                    match source_ml_source1_s23 {
                                        self::MlTemporaryClosure1Loop2Result::MlTemporaryClosure1Loop2Done(value_2) => {
                                            value_2
                                        }
                                        self::MlTemporaryClosure1Loop2Result::MlTemporaryClosure1Loop2Return(source_ml_temporary_result2) => {
                                            source_ml_temporary_result2
                                        }
                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                            );
                            ml_values1
                                .iter()
                                .cloned()
                                .enumerate()
                                .map(|(ml_index1, ml_value1)| ml_callback1(ml_value1))
                                .collect::<Vec<_>>()
                        };
                        if {
                            let ml_values2 = columns.clone();
                            let ml_callback2 =
                                std::rc::Rc::new(move |value: f64| -> bool { value < 0f64 });
                            ml_values2
                                .iter()
                                .cloned()
                                .enumerate()
                                .any(|(ml_index2, ml_value2)| ml_callback2(ml_value2))
                        } {
                            Vec::<f64>::new()
                        } else {
                            columns
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryClosure1Loop2Result {
    MlTemporaryClosure1Loop2Done(f64),
    MlTemporaryClosure1Loop2Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_closure1_loop2(
    part: &self::ml_string::Text,
    mut value: f64,
    mut index: f64,
) -> self::MlTemporaryClosure1Loop2Result {
    loop {
        return if index < (part.length()) {
            {
                let unit = {
                    let source_ml_source1_string_unit_operand24 = part.clone();
                    {
                        let source_ml_source1_string_unit_operand25 = index;
                        (std::rc::Rc::new(
                            move |text: self::ml_string::Text, position: f64| -> f64 {
                                {
                                    let index_2 = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index_2 < 0f64) || (index_2 >= (text.length())) {
                                        f64::NAN
                                    } else {
                                        ({
                                    let source_ml_source1_string_unit_operand31 = {
                                        let source_ml_source1_string_range_operand26 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand27 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand28 = index_2 + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position29 = source_ml_source1_string_range_operand27;
                                                    {
                                                        let source_ml_source1_string_range_position30 = source_ml_source1_string_range_operand28;
                                                        source_ml_source1_string_range_operand26.slice(source_ml_source1_string_range_position29, source_ml_source1_string_range_position30)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand32 = 0f64;
                                        source_ml_source1_string_unit_operand31.code_point_at(source_ml_source1_string_unit_operand32)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source1_string_unit_operand24,
                            source_ml_source1_string_unit_operand25,
                        )
                    }
                };
                if (unit < 48f64) || (unit > 57f64) {
                    self::MlTemporaryClosure1Loop2Result::MlTemporaryClosure1Loop2Return(-1f64)
                } else {
                    {
                        let value_2 = ((value * 10f64) + unit) - 48f64;
                        if value_2 > 4_294_967_295f64 {
                            self::MlTemporaryClosure1Loop2Result::MlTemporaryClosure1Loop2Return(
                                -1f64,
                            )
                        } else {
                            {
                                let index_2 = index + 1f64;
                                {
                                    (value, index) = (value_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryClosure1Loop2Result::MlTemporaryClosure1Loop2Done(value)
        };
    }
}
