// Generated from complete js/src/unsigned-link-identity-rules.js. Do not edit by hand.
use super::MlTemporaryDecodeUnsignedLinkEnvelopeLoop16Result;
use super::MlTemporaryDecodeUnsignedLinkEnvelopeLoop17Result;
use super::MlTemporaryDecodeUnsignedLinkEnvelopeLoop18Result;
use super::MlTemporaryDecodeUnsignedLinkEnvelopeLoop19Result;
use super::MlTemporaryDecodeUnsignedLinkEnvelopeLoop20Result;
use super::MlTemporaryEncodeUnsignedLinkEnvelopeLoop12Result;
use super::MlTemporaryEncodeUnsignedLinkEnvelopeLoop14Result;
use super::MlTemporaryParseUnsignedLinkIdentityLoop2Result;
use super::MlTemporaryReadUnsignedLinkIdentityLoop6Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::source_ml_temporary_decode_unsigned_link_envelope_loop16;
use super::source_ml_temporary_decode_unsigned_link_envelope_loop17;
use super::source_ml_temporary_decode_unsigned_link_envelope_loop18;
use super::source_ml_temporary_decode_unsigned_link_envelope_loop19;
use super::source_ml_temporary_decode_unsigned_link_envelope_loop20;
use super::source_ml_temporary_encode_unsigned_link_envelope_loop10;
use super::source_ml_temporary_encode_unsigned_link_envelope_loop11;
use super::source_ml_temporary_encode_unsigned_link_envelope_loop12;
use super::source_ml_temporary_encode_unsigned_link_envelope_loop13;
use super::source_ml_temporary_encode_unsigned_link_envelope_loop14;
use super::source_ml_temporary_encode_unsigned_link_identity_loop4;
use super::source_ml_temporary_parse_unsigned_link_identity_loop2;
use super::source_ml_temporary_read_unsigned_link_byte_factor_loop1;
use super::source_ml_temporary_read_unsigned_link_identity_loop6;
use super::source_ml_temporary_render_unsigned_link_identity_loop3;

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
pub fn read_unsigned_link_byte_factor(exponent: f64) -> f64 {
    if (!(self::ml_math::is_integer(exponent)) || (exponent < 0f64)) || (exponent > 3f64) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32,
                108, 105, 110, 107, 32, 98, 121, 116, 101, 32, 112, 111, 115, 105, 116, 105, 111,
                110,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        {
            let factor = 1f64;
            {
                let index = 0f64;
                {
                    self::source_ml_temporary_read_unsigned_link_byte_factor_loop1(
                        exponent, factor, index,
                    )
                }
            }
        }
    }
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
pub fn compare_unsigned_link_identities(first: &[f64], second: &[f64]) -> f64 {
    if !(self::accept_unsigned_link_identity(first))
        || !(self::accept_unsigned_link_identity(second))
    {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32,
                108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else if (self::read_unsigned_link_limb(first, 1f64))
        == (self::read_unsigned_link_limb(second, 1f64))
    {
        if (self::read_unsigned_link_limb(first, 0f64))
            == (self::read_unsigned_link_limb(second, 0f64))
        {
            0f64
        } else if (self::read_unsigned_link_limb(first, 0f64))
            < (self::read_unsigned_link_limb(second, 0f64))
        {
            -1f64
        } else {
            1f64
        }
    } else {
        if (self::read_unsigned_link_limb(first, 1f64))
            < (self::read_unsigned_link_limb(second, 1f64))
        {
            -1f64
        } else {
            1f64
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
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32,
                108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn parse_unsigned_link_identity(text: &self::ml_string::Text) -> Vec<f64> {
    if ((text.length()) == 0f64) || ((text.length()) > 20f64) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32,
                108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        {
            let low = 0f64;
            {
                let high = 0f64;
                {
                    let index = 0f64;
                    {
                        let source_ml_source1_s1 =
                            self::source_ml_temporary_parse_unsigned_link_identity_loop2(
                                text, low, high, index,
                            );
                        match source_ml_source1_s1 {
                            self::MlTemporaryParseUnsignedLinkIdentityLoop2Result::MlTemporaryParseUnsignedLinkIdentityLoop2Done(low_2, high_2) => {
                                {
                                    let result = vec![low_2, high_2];
                                    if (self::accept_unsigned_link_identity(&(result))) {
                                        result
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121])).0, message_present: true }).raise()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_unsigned_link_identity(words: &[f64]) -> self::ml_string::Text {
    if (self::accept_unsigned_link_identity(words)) {
        {
            let low = self::read_unsigned_link_limb(words, 0f64);
            {
                let high = self::read_unsigned_link_limb(words, 1f64);
                {
                    let text = self::ml_string::Text::from_units(vec![]);
                    {
                        self::source_ml_temporary_render_unsigned_link_identity_loop3(
                            low, high, text,
                        )
                    }
                }
            }
        }
    } else {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32,
                108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn encode_unsigned_link_identity(words: &[f64]) -> Vec<f64> {
    if (self::accept_unsigned_link_identity(words)) {
        {
            let bytes = Vec::<f64>::new();
            {
                let limb = 0f64;
                {
                    self::source_ml_temporary_encode_unsigned_link_identity_loop4(
                        words, bytes, limb,
                    )
                }
            }
        }
    } else {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32,
                108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_unsigned_link_identity(bytes: &[f64], offset: f64) -> Vec<f64> {
    if (!(self::ml_math::is_safe_integer(offset)) || (offset < 0f64))
        || ((offset + 8f64) > (bytes.len() as f64))
    {
        Vec::<f64>::new()
    } else {
        {
            let low = 0f64;
            {
                let high = 0f64;
                {
                    let index = 0f64;
                    {
                        let source_ml_source1_s2 =
                            self::source_ml_temporary_read_unsigned_link_identity_loop6(
                                bytes, offset, low, high, index,
                            );
                        match source_ml_source1_s2 {
                            self::MlTemporaryReadUnsignedLinkIdentityLoop6Result::MlTemporaryReadUnsignedLinkIdentityLoop6Done(low_2, high_2) => {
                                {
                                    let words = vec![low_2, high_2];
                                    if (self::accept_unsigned_link_identity(&(words))) {
                                        words
                                    } else {
                                        Vec::<f64>::new()
                                    }
                                }
                            }
                            self::MlTemporaryReadUnsignedLinkIdentityLoop6Result::MlTemporaryReadUnsignedLinkIdentityLoop6Return(source_ml_temporary_result6) => {
                                source_ml_temporary_result6
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
pub fn decode_unsigned_link_identity(bytes: &[f64], offset: f64) -> Vec<f64> {
    {
        let words = self::read_unsigned_link_identity(bytes, offset);
        if (words.len() as f64) == 2f64 {
            words
        } else {
            (self::ml_source_exception::SourceException {
                class_name: String::from("RangeError"),
                message_units: (self::ml_string::Text::from_units(vec![
                    105, 110, 118, 97, 108, 105, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100,
                    32, 108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 98,
                    121, 116, 101, 115,
                ]))
                .0,
                message_present: true,
            })
            .raise()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn encode_unsigned_link_envelope(
    identities: &[f64],
    next: &[f64],
    body: &[f64],
    byte_limit: f64,
) -> Vec<f64> {
    if (((((!(self::ml_math::is_safe_integer(byte_limit)) || (byte_limit < 33f64))
        || (((identities.len() as f64) % 2f64) != 0f64))
        || (((identities.len() as f64) / 2f64) > 4_294_967_295f64))
        || ((body.len() as f64) > 4_294_967_295f64))
        || ((body.len() as f64) == 0f64))
        || (((((identities.len() as f64) * 4f64) + (body.len() as f64)) + 25f64) > byte_limit)
    {
        vec![-1f64]
    } else if ((next.len() as f64) != 0f64) && !(self::accept_unsigned_link_identity(next)) {
        vec![-2f64]
    } else {
        {
            let output = vec![
                77f64,
                76f64,
                85f64,
                54f64,
                52f64,
                78f64,
                48f64,
                49f64,
                if (next.len() as f64) == 0f64 {
                    1f64
                } else {
                    0f64
                },
            ];
            {
                let cursor = if (next.len() as f64) == 0f64 {
                    vec![1f64, 0f64]
                } else {
                    next.to_vec()
                };
                {
                    let source_ml_temporary_values7 =
                        self::encode_unsigned_link_identity(&(cursor));
                    {
                        let source_ml_temporary_index7 = self::ml::Big::from_i128(0);
                        {
                            let output_2 =
                                self::source_ml_temporary_encode_unsigned_link_envelope_loop10(
                                    output,
                                    &(source_ml_temporary_values7),
                                    source_ml_temporary_index7,
                                );
                            {
                                let byte = 0f64;
                                {
                                    let output_3 = self::source_ml_temporary_encode_unsigned_link_envelope_loop11(identities, output_2, byte);
                                    {
                                        let offset = 0f64;
                                        {
                                            let source_ml_source1_s3 = self::source_ml_temporary_encode_unsigned_link_envelope_loop12(identities, output_3, offset);
                                            match source_ml_source1_s3 {
                                                    self::MlTemporaryEncodeUnsignedLinkEnvelopeLoop12Result::MlTemporaryEncodeUnsignedLinkEnvelopeLoop12Done(output_4) => {
                                                        {
                                                            let byte_2 = 0f64;
                                                            {
                                                                let output_5 = self::source_ml_temporary_encode_unsigned_link_envelope_loop13(body, output_4, byte_2);
                                                                {
                                                                    let source_ml_temporary_values9 = body.to_vec();
                                                                    {
                                                                        let source_ml_temporary_index9 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let source_ml_source1_s4 = self::source_ml_temporary_encode_unsigned_link_envelope_loop14(output_5, &(source_ml_temporary_values9), source_ml_temporary_index9);
                                                                            match source_ml_source1_s4 {
                                                                                self::MlTemporaryEncodeUnsignedLinkEnvelopeLoop14Result::MlTemporaryEncodeUnsignedLinkEnvelopeLoop14Done(output_6) => {
                                                                                    output_6
                                                                                }
                                                                                self::MlTemporaryEncodeUnsignedLinkEnvelopeLoop14Result::MlTemporaryEncodeUnsignedLinkEnvelopeLoop14Return(source_ml_temporary_result14) => {
                                                                                    source_ml_temporary_result14
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryEncodeUnsignedLinkEnvelopeLoop12Result::MlTemporaryEncodeUnsignedLinkEnvelopeLoop12Return(source_ml_temporary_result14_2) => {
                                                        source_ml_temporary_result14_2
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn decode_unsigned_link_envelope(input: &[f64], byte_limit: f64) -> Vec<f64> {
    if ((!(self::ml_math::is_safe_integer(byte_limit)) || (byte_limit < 33f64))
        || ((input.len() as f64) > byte_limit))
        || ((input.len() as f64) < 26f64)
    {
        vec![-1f64]
    } else {
        {
            let header = vec![77f64, 76f64, 85f64, 54f64, 52f64, 78f64, 48f64, 49f64];
            {
                let index = 0f64;
                {
                    let source_ml_source1_s5 =
                        self::source_ml_temporary_decode_unsigned_link_envelope_loop16(
                            input,
                            &(header),
                            index,
                        );
                    match source_ml_source1_s5 {
                        self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop16Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop16Done => {
                            if ((self::read_unsigned_link_limb(input, 8f64)) != 0f64) && ((self::read_unsigned_link_limb(input, 8f64)) != 1f64) {
                                vec![-6f64]
                            } else {
                                {
                                    let next = self::read_unsigned_link_identity(input, 9f64);
                                    {
let ml_library_inequality_1 = (next.len() as f64) != 2f64 ;
if ml_library_inequality_1 {
                                        vec![-2f64]
                                    } else if ((self::read_unsigned_link_limb(input, 8f64)) == 1f64) && (((self::read_unsigned_link_limb(&(next), 0f64)) != 1f64) || ((self::read_unsigned_link_limb(&(next), 1f64)) != 0f64)) {
                                            vec![-7f64]
                                        } else {
                                            {
                                                let count = 0f64;
                                                {
                                                    let byte = 0f64;
                                                    {
                                                        let source_ml_source1_s6 = self::source_ml_temporary_decode_unsigned_link_envelope_loop17(input, count, byte);
                                                        match source_ml_source1_s6 {
                                                            self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop17Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop17Done(count_2) => {
                                                                if count_2 > ((((input.len() as f64) - 25f64) / 8f64).floor()) {
                                                                    vec![-8f64]
                                                                } else {
                                                                    {
                                                                        let output = vec![self::read_unsigned_link_limb(input, 8f64), self::read_unsigned_link_limb(&(next), 0f64), self::read_unsigned_link_limb(&(next), 1f64), count_2];
                                                                        {
                                                                            let previous = Vec::<f64>::new();
                                                                            {
                                                                                let index_2 = 0f64;
                                                                                {
                                                                                    let source_ml_source1_s7 = self::source_ml_temporary_decode_unsigned_link_envelope_loop18(input, count_2, output, previous, index_2);
                                                                                    match source_ml_source1_s7 {
                                                                                        self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop18Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop18Done(output_2) => {
                                                                                            {
                                                                                                let position = 21f64 + (count_2 * 8f64);
                                                                                                {
                                                                                                    let length = 0f64;
                                                                                                    {
                                                                                                        let byte_2 = 0f64;
                                                                                                        {
                                                                                                            let source_ml_source1_s8 = self::source_ml_temporary_decode_unsigned_link_envelope_loop19(input, position, length, byte_2);
                                                                                                            match source_ml_source1_s8 {
                                                                                                                self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop19Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop19Done(length_2) => {
                                                                                                                    if (length_2 == 0f64) || (((position + 4f64) + length_2) != (input.len() as f64)) {
                                                                                                                        vec![-9f64]
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let at = position + 4f64;
                                                                                                                            {
                                                                                                                                let source_ml_source1_s9 = self::source_ml_temporary_decode_unsigned_link_envelope_loop20(input, output_2, at);
                                                                                                                                match source_ml_source1_s9 {
                                                                                                                                    self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop20Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop20Done(output_3) => {
                                                                                                                                        output_3
                                                                                                                                    }
                                                                                                                                    self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop20Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop20Return(source_ml_temporary_result20) => {
                                                                                                                                        source_ml_temporary_result20
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                                self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop19Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop19Return(source_ml_temporary_result20_2) => {
                                                                                                                    source_ml_temporary_result20_2
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop18Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop18Return(source_ml_temporary_result20_3) => {
                                                                                            source_ml_temporary_result20_3
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop17Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop17Return(source_ml_temporary_result20_4) => {
                                                                source_ml_temporary_result20_4
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
                        self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop16Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop16Return(source_ml_temporary_result20_5) => {
                            source_ml_temporary_result20_5
                        }
                    }
                }
            }
        }
    }
}
